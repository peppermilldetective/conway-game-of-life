#[macro_use]
extern crate glium;

#[derive(Copy, Clone)]
struct Vertex {
   position: [f32; 2]
}

implement_vertex!(Vertex, position);

struct Shaders<'a> {
   vert: &'a str,
   frag: &'a str
}

impl<'a> Shaders<'a> {
   fn new() -> Shaders<'a> {
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

#[derive(Copy, Clone)]
struct Cell {
   position: [f32; 2]
}

trait Drawable {
   fn draw(&self);
}

impl Cell {
   fn new(x: f32, y: f32) -> Cell {
      Cell {
         position: [x, y]
      }
   }
}

impl Drawable for Cell {
   fn draw(&self) {
      // TODO
   }
}

struct Grid {
   cells: Vec<Cell>
}

impl Grid {
   fn new(width: i32, height: i32) -> Grid {
      let mut cells: Vec<Cell> = Vec::new();

      for x in 0..width {
         for y in 0..height {
            cells.push(Cell::new(
               x as f32,
               y as f32
            ));
         }
      }

      Grid {
         cells
      }
   }

   fn draw_grid(grid: Grid) {
      for cell in grid.cells {
         cell.draw();
      }
   }
}

fn create_buffers(display: &glium::Display) -> (glium::VertexBuffer<Vertex>, glium::IndexBuffer<u16>) {
   use glium::{
      VertexBuffer,
      IndexBuffer,
      index::PrimitiveType
   };

   let shape = vec![
      Vertex { position: [-0.5,  0.5] },
      Vertex { position: [-0.5, -0.5] },
      Vertex { position: [ 0.5, -0.5] },
      Vertex { position: [ 0.5,  0.5] },
   ];

   (
      VertexBuffer::new(display, &shape).unwrap(),
      IndexBuffer::new(display, PrimitiveType::TrianglesList, &[0u16, 1, 2, 0, 2, 3]).unwrap()
   )
}

fn main() {
   use glium::{glutin, Surface};

   let width = 1024.0f64;
   let height =1024.0f64;
   let title = "Conway's Game of Life";
   
   let rows = 50;
   let columns = 50;
   let cell_count = rows * columns;

   let grid = Grid::new(rows, columns);

   let mut events_loop = glutin::EventsLoop::new();
   let wb = glutin::WindowBuilder::new()
      .with_dimensions(glutin::dpi::LogicalSize::new(width as f64, height as f64))
      .with_title(title);
   let cb = glutin::ContextBuilder::new();
   let display = glium::Display::new(wb, cb, &events_loop).unwrap();

   let shaders = Shaders::new();

   let program =
      glium::Program::from_source(&display, shaders.vert, shaders.frag, None).unwrap();

   let (vertex_buffer, index_buffer) = create_buffers(&display);

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
   }
}