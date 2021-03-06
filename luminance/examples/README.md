# luminance examples

This directory holds several examples you can use to learn how to use luminance. They are sorted
following a specific order that will help you learn from basics to more advanced features.

Each example comes in with a few explanations and how to use them at the top of the `main.rs` file.

> Have fun!

  - [01-hello-world](./01-hello-world): learn how to draw two colored triangles by using vertex
    colors (comes in *direct* and *indexed* geometry versions).
  - [02-render-state](./02-render-state): learn how to change the render state to change the way the
    triangles are rendered or how fragment blending happens.
  - [03-sliced-tess](./03-sliced-tess): learn how to slice a single GPU geometry to dynamically
    select contiguous regions of it to render!
  - [04-shader-uniforms](./04-shader-uniforms): send colors and position information to the GPU to
    add interaction with a simple yet colorful triangle!
  - [05-attributeless](./05-attributeless): render a triangle without sending any vertex data to the
    GPU!
  - [06-texture](./06-texture): learn how to use a loaded image as a luminance texture on the GPU!
  - [07-offscreen](./07-offscreen): get introduced to *offscreen rendering*, a powerful technique
    used to render frames into memory without directly displaying them on your screen. Offscreen
    framebuffers can be seen as a generalization of your screen.
  - [08-shader-uniforms-adapt](./08-shader-uniforms-adapt): learn how to change the type of a shader
    program’s uniform interface on the fly.
  - [09-dynamic-uniform-interface](./09-dynamic-uniform-interface): implement a dynamic lookup for
    shader program the easy way by using program interfaces to query uniforms on the fly!
