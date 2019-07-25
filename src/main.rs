#[macro_use]
extern crate glium;
extern crate rand;

struct Cells {
   width: usize,
   height: usize,
   states: Vec<f32>
}

impl Cells {
   fn new(alive: f32, width: usize, height: usize) -> Cells {
      let states = (0..width*height).map(|x| {
         (rand::random::<u32>() % 2) as f32
      }).collect();

      Cells {
         width,
         height,
         states
      }
   }

   fn tick(&mut self, neighbors: Vec<f32>) {
      let new_state = vec![0.0f32;self.width * self.height];
   }
}

fn main() {
   use glium::index::PrimitiveType;
   use glium::{glutin, Surface};

   let mut events_loop = glutin::EventsLoop::new();
   let wb = glutin::WindowBuilder::new();
   let cb = glutin::ContextBuilder::new();
   let display = glium::Display::new(wb, cb, &events_loop).unwrap();
   let CELL_ROW_COUNT = 50;
   let CELL_COLUMN_COUNT = 50;
   let CELL_COUNT = CELL_COLUMN_COUNT * CELL_ROW_COUNT;

   let vertex_shader_src = r#"
      #version 140

      in vec2 position;

      uniform mat4 matrix;

      void main() {
         gl_Position = matrix * vec4(position, 0.0, 1.0);
      }
   "#;

   let fragment_shader_src = r#"
      #version 140

      out vec4 color;

      uniform float alive;

      void main() {
         color = vec4(alive, alive, alive, 1.0);
      }
   "#;

   let program =
      glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();

   let (mut vertex_buffer, index_buffer) = {
      #[derive(Copy, Clone)]
      struct Vertex {
         position: [f32; 2],
         index: f32
      }

      implement_vertex!(Vertex, position);

      let mut vb: glium::VertexBuffer<Vertex> =
         glium::VertexBuffer::empty_dynamic(&display, CELL_COUNT * 4).unwrap();
      let mut ib_data: Vec<u16> = Vec::with_capacity(CELL_COUNT * 6);

      for (num, cell) in vb.map().chunks_mut(4).enumerate() {
         let step: f32 = 1.0 / 51.0;
         let index: f32 = num as f32;
         let position: (f32, f32) = (
            step * (num as f32 % CELL_ROW_COUNT as f32),
            step * (num as f32 % CELL_COLUMN_COUNT as f32),
         );

         cell[0].position[0] = position.0;
         cell[0].position[1] = position.1;
         cell[0].index = index;
         cell[1].position[0] = position.0 + step;
         cell[1].position[1] = position.1;
         cell[1].index = index;
         cell[2].position[0] = position.0 + step;
         cell[2].position[1] = position.1;
         cell[2].index = index;
         cell[3].position[0] = position.0 + step;
         cell[3].position[1] = position.1 + step;
         cell[3].index = index;

         let num = num as u16;
         ib_data.push(num * 4);
         ib_data.push(num * 4 + 1);
         ib_data.push(num * 4 + 2);
         ib_data.push(num * 4 + 1);
         ib_data.push(num * 4 + 3);
         ib_data.push(num * 4 + 2);
      }

      (
         vb,
         glium::index::IndexBuffer::new(&display, PrimitiveType::TrianglesList, &ib_data).unwrap(),
      )
   };

   let grid = create_initial();

   let mut closed = false;
   while !closed {
      let is_alive = 1.0f32;

      let uniforms = uniform! {
         matrix: [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0f32],
         ],
         alive: is_alive,
      };

      let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

      let mut target = display.draw();
      target.clear_color(0.0, 0.0, 0.0, 1.0);
      target
         .draw(
            &vertex_buffer,
            &indices,
            &program,
            &uniforms,
            &Default::default(),
         )
         .unwrap();
      target.finish().unwrap();

      events_loop.poll_events(|ev| match ev {
         glutin::Event::WindowEvent { event, .. } => match event {
            glutin::WindowEvent::CloseRequested => closed = true,
            _ => (),
         },
         _ => (),
      });
   }
}