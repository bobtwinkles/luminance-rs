use crate::attrib::{AttrError, get_field_attr_once};
use proc_macro::TokenStream;
use quote::quote;
use std::fmt;
use syn::{Attribute, DataEnum, Ident, Type};

const KNOWN_SUBKEYS: &[&str] = &["name", "repr", "type_name"];

#[derive(Debug)]
pub(crate) enum VertexAttribSemImplError {
  AttributeErrors(Vec<AttrError>),
}

impl fmt::Display for VertexAttribSemImplError {
  fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
    match *self {
      VertexAttribSemImplError::AttributeErrors(ref errs) => {
        for err in errs {
          err.fmt(f)?;
          writeln!(f, "").unwrap();
        }

        Ok(())
      }
    }
  }
}

/// Get vertex semantics attributes.
///
///   (name, repr, type_name)
fn get_vertex_sem_attribs<'a, A>(var_name: &Ident, attrs: A) -> Result<(Ident, Type, Type), AttrError> where A: IntoIterator<Item = &'a Attribute> + Clone {
  let sem_name = get_field_attr_once::<_, Ident>(var_name, attrs.clone(), "sem", "name", KNOWN_SUBKEYS)?;
  let sem_repr = get_field_attr_once::<_, Type>(var_name, attrs.clone(), "sem", "repr", KNOWN_SUBKEYS)?;
  let sem_type_name = get_field_attr_once::<_, Type>(var_name, attrs, "sem", "type_name", KNOWN_SUBKEYS)?;

  Ok((sem_name, sem_repr, sem_type_name))
}

pub(crate) fn generate_enum_vertex_attrib_sem_impl(ident: Ident, enum_: DataEnum) -> Result<TokenStream, VertexAttribSemImplError> {
  let fields = enum_.variants.into_iter().map(|var| {
    get_vertex_sem_attribs(&var.ident, var.attrs.iter()).map(|attrs| {
      (var.ident, attrs.0, attrs.1, attrs.2)
    })
  });

  let mut parse_branches = Vec::new();
  let mut field_based_gen = Vec::new();

  let mut errors = Vec::new();

  for field in fields {
    match field {
      Ok(field) => {
        // parse branches
        let sem_var = field.0;
        let sem_name = field.1.to_string();
        let repr_ty_name = field.2;
        let ty_name = field.3;

        // dynamic branch used for parsing the semantics from a string
        let branch = quote!{
          #sem_name => Some(#ident::#sem_var)
        };

        parse_branches.push(branch);

        // field-based code generation
        let field_gen = quote!{
          // vertex attrib type
          #[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
          pub struct #ty_name {
            repr: #repr_ty_name
          }

          // convert from the repr type to the vertex attrib type
          impl From<#repr_ty_name> for #ty_name {
            fn from(repr: #repr_ty_name) -> Self {
              #ty_name::new(repr)
            }
          }

          // convert from the repr type to the vertex attrib type
          impl #ty_name {
            pub const fn new(repr: #repr_ty_name) -> Self {
             #ty_name {
               repr
             }
            }
          }

          // get the associated semantics
          impl luminance::vertex::HasSemantics for #ty_name {
            type Sem = #ident;

            const VERTEX_ATTRIB_SEM: Self::Sem = #ident::#sem_var;
          }

          // make the vertex attrib impl VertexAttrib by forwarding implementation to the repr type
          unsafe impl luminance::vertex::VertexAttrib for #ty_name {
            const VERTEX_ATTRIB_FMT: luminance::vertex::VertexAttribFmt =
              <#repr_ty_name as luminance::vertex::VertexAttrib>::VERTEX_ATTRIB_FMT;
          }
        };

        field_based_gen.push(field_gen);
      }

      Err(e) => errors.push(e)
    }
  }

  if !errors.is_empty() {
    return Err(VertexAttribSemImplError::AttributeErrors(errors));
  }

  // generate the implementation of VertexAttribSem
  let vertex_attrib_sem_impl = quote!{
    impl luminance::vertex::VertexAttribSem for #ident {
      fn index(&self) -> usize {
        *self as usize
      }

      fn parse(name: &str) -> Option<Self> {
        match name {
          #(#parse_branches,)*
          _ => None
        }
      }
    }
  };

  let output = quote!{
    #vertex_attrib_sem_impl
    #(#field_based_gen)*
  };

  Ok(output.into())
}

