#[macro_use]
extern crate glium;
extern crate rand;

struct Cells {
   width: usize,
   height: usize,
   states: Vec<i32>,
   neighbors: Vec<Vec<usize>>
}

macro_rules! go_up {
   ($x:expr, $w:expr, $a:expr) => {
      ($x-$w+$a)%$a
   };
}

macro_rules! go_down {
   ($x:expr, $w:expr, $a:expr) => {
      ($x+$w+$a)%$a
   };
}

macro_rules! go_left {
   ($x:expr, $w:expr) => {
      $w*($x/$w)+($x-1)%$w
   };
}

macro_rules! go_right {
   ($x:expr, $w:expr) => {
      $w*($x/$w)+($x+1)%$w
   };
}

impl Cells {
   fn new(width: usize, height: usize) -> Cells {
      let area = width*height;
      let states = (0..area).map(|x| {
         (rand::random::<u32>() % 2) as i32
      }).collect();

      let neighbors: Vec<Vec<usize>> = (0..area).map(|x| {
         let up = go_up!(x, width, area);
         let down = go_down!(x, width, area);
         let left = go_left!(x, width);
         let right = go_right!(x, width);
         let ul_corner = go_left!(up, width);
         let ur_corner = go_right!(up, width);
         let ll_corner = go_left!(down, width);
         let lr_corner = go_right!(down, width);

         vec![
            ul_corner,
            up,
            ur_corner,
            left,
            right,
            ll_corner,
            down,
            lr_corner
         ]
      }).collect();

      Cells {
         width,
         height,
         states,
         neighbors
      }
   }

   fn tick(&mut self) {
      let length = self.width * self.height;
      let mut new_state = vec![0i32; length];

      for i in 0..length {
         new_state[i] = {
            let sum: i32 = self.neighbors[i].iter().map(|index| {
               self.states[index.to_owned()]
            }).collect::<Vec<i32>>().iter().sum();

            if self.states[i] > 0 && (sum < 2 || sum > 3) {
                  0
            }
            else if self.states[i] < 1 && sum == 3 {
                  1
            }
            else {
               self.states[i]
            }
         }
      }

      self.states = new_state;
   }
}

fn main() {
   use glium::index::PrimitiveType;
   use glium::{glutin, Surface};

   let width = 1024;
   let height = 720;
   let title = "Conway's Game of Life";

   let mut events_loop = glutin::EventsLoop::new();
   let wb = glutin::WindowBuilder::new()
      .with_dimensions(glutin::dpi::LogicalSize::new(width as f64, height as f64))
      .with_title(title);
   let cb = glutin::ContextBuilder::new();
   let display = glium::Display::new(wb, cb, &events_loop).unwrap();
   let CELL_ROW_COUNT = 50;
   let CELL_COLUMN_COUNT = 50;
   let CELL_COUNT = CELL_COLUMN_COUNT * CELL_ROW_COUNT;

   let vertex_shader_src = r#"
      #version 140

      in vec2 position;
      in vec2 index;

      uniform 

      void main() {
         gl_Position = vec4(position, 0.0, 1.0);
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

   let (vertex_buffer, index_buffer) = {
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

   let mut cells: Cells = Cells::new(50, 50);

   let mut closed = false;
   while !closed {
      let uniforms = uniform! {
      };

      let mut frame = display.draw();
      frame.clear_color(0.0, 0.0, 0.0, 1.0);
      frame
         .draw(
            &vertex_buffer,
            &index_buffer,
            &program,
            &uniforms,
            &Default::default(),
         )
         .unwrap();
      frame.finish().unwrap();

      events_loop.poll_events(|ev| match ev {
         glutin::Event::WindowEvent { event, .. } => match event {
            glutin::WindowEvent::CloseRequested => closed = true,
            _ => (),
         },
         _ => (),
      });

      cells.tick();
   }
}