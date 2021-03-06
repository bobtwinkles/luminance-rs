extern crate proc_macro;

mod attrib;
mod vertex;
mod vertex_attrib_sem;

use crate::vertex::generate_vertex_impl;
use crate::vertex_attrib_sem::generate_enum_vertex_attrib_sem_impl;
use proc_macro::TokenStream;
use syn::{self, Data, DeriveInput, parse_macro_input};

#[proc_macro_derive(Vertex, attributes(vertex))]
pub fn derive_vertex(input: TokenStream) -> TokenStream {
  let di: DeriveInput = parse_macro_input!(input);

  match di.data {
    // for now, we only handle structs
    Data::Struct(struct_) => {
      match generate_vertex_impl(di.ident, di.attrs.iter(), struct_) {
        Ok(impl_) => impl_,
        Err(e) => panic!("{}", e)
      }
    }

    _ => panic!("only structs are currently supported for deriving Vertex")
  }
}

#[proc_macro_derive(VertexAttribSem, attributes(sem))]
pub fn derive_vertex_attrib_sem(input: TokenStream) -> TokenStream {
  let di: DeriveInput = parse_macro_input!(input);

  match di.data {
    // for now, we only handle enums
    Data::Enum(enum_) => {
      match generate_enum_vertex_attrib_sem_impl(di.ident, enum_) {
        Ok(impl_) => impl_,
        Err(e) => panic!("{}", e)
      }
    }

    _ => panic!("only enums are currently supported for deriving VertexAttribSem")
  }
}
