pub struct Shaders<'a> {
   pub vert: &'a str,
   pub frag: &'a str
}

impl<'a> Shaders<'a> {
   pub fn new() -> Shaders<'a> {
      let vertex_shader_src = r#"
         #version 140

         in vec2 position;

         uniform 

         void main() {
            gl_Position = vec4(position, 0.0, 1.0);
         }
      "#;

      let fragment_shader_src = r#"
         #version 140

         out vec4 color;

         void main() {
            color = vec4(1.0, 1.0, 1.0, 1.0);
         }
      "#;

      Shaders::<'a> {
         vert: vertex_shader_src,
         frag: fragment_shader_src
      }
   }
}