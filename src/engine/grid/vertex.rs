use glium;

#[derive(Copy, Clone)]
pub struct Vertex {
   pub position: [f32; 2]
}

glium::implement_vertex!(Vertex, position);
