#![allow(clippy::single_match)]

#[macro_use]
extern crate glium;
extern crate rand;

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
   position: [i32; 2],
   draw_coord: [f32; 2],
   deltas: [f32; 2],
   alive: u32
}

trait Drawable {
   fn draw(&self,
      vertex_buffer: &glium::VertexBuffer<Vertex>,
      index_buffer: &glium::IndexBuffer<u16>,
      program: &glium::Program,
      frame: &mut glium::Frame);
   fn create_buffers(&self, display: &glium::Display) -> (glium::VertexBuffer<Vertex>, glium::IndexBuffer<u16>);
}

impl Cell {
   fn new(x: i32, y: i32, draw_x: f32, draw_y: f32, d_x: f32, d_y: f32, alive: u32) -> Cell {
      Cell {
         position: [x, y],
         draw_coord: [draw_x, draw_y],
         deltas: [d_x, d_y],
         alive
      }
   }
}

impl Drawable for Cell {
   fn draw(&self,
      vertex_buffer: &glium::VertexBuffer<Vertex>,
      index_buffer: &glium::IndexBuffer<u16>,
      program: &glium::Program,
      frame: &mut glium::Frame)
   {
      use glium::Surface;

      if self.alive == 0 {
         return;
      }

      let uniforms = uniform! {
      };

      frame
         .draw(
            vertex_buffer,
            index_buffer,
            program,
            &uniforms,
            &Default::default(),
         )
         .unwrap();
   }

   fn create_buffers(&self, display: &glium::Display) -> (glium::VertexBuffer<Vertex>, glium::IndexBuffer<u16>) {
      use glium::{
         VertexBuffer,
         IndexBuffer,
         index::PrimitiveType
      };

      let x = self.draw_coord[0];
      let y = self.draw_coord[1];
      let d_x = self.deltas[0];
      let d_y = self.deltas[1];

      let shape = vec![
         Vertex { position: [ x      , y      ] },
         Vertex { position: [ x + d_x, y      ] },
         Vertex { position: [ x      , y + d_y] },
         Vertex { position: [ x + d_x, y + d_y] },
      ];

      (
         VertexBuffer::new(display, &shape).unwrap(),
         IndexBuffer::new(display, PrimitiveType::TrianglesList, &[0u16, 1, 2, 1, 2, 3]).unwrap()
      )
   }
}

struct Grid {
   width: i32,
   height: i32,
   cells: Vec<Cell>
}

impl Grid {
   fn new(width: i32, height: i32) -> Grid {
      let mut cells: Vec<Cell> = Vec::new();

      let d_x = 2.0 / width as f32;
      let d_y = 2.0 / height as f32;

      for y in 0..height {
         for x in 0..width {
            let draw_x = -1.0 + (d_x * x as f32);
            let draw_y = -1.0 + (d_y * y as f32);

            cells.push(Cell::new(
               x,
               y,
               draw_x,
               draw_y,
               d_x,
               d_y,
               rand::random::<u32>() % 2
            ));
         }
      }

      Grid {
         width,
         height,
         cells
      }
   }

   fn draw_grid(grid: &Grid, display: &glium::Display, program: &glium::Program) {
      use glium::Surface;

      let mut frame = display.draw();
      frame.clear_color(0.0, 0.0, 0.0, 1.0);

      let cells = &grid.cells;

      for cell in cells {
         let (vert_buf, ind_buf) = cell.create_buffers(display);
         cell.draw(&vert_buf, &ind_buf, program, &mut frame);
      }

      frame.finish().unwrap();
   }

   fn next_state(grid: &mut Grid) {
      let mut next_state: Vec<Cell> = Vec::new();

      for (i, cell) in grid.cells.iter().enumerate() {
         let i = i as i32;
         let width = grid.width;
         let area = width * grid.height;

         let up = ((i + area) + width)%area;
         let down = ((i + area) - width)%area;
         let left = (width * (i/width)) + ((i + width) - 1)%width;
         let right = (width * (i/width)) + ((i + width) + 1)%width;
         let ul = (width * (up/width)) + ((up + width) - 1)%width;
         let ur = (width * (up/width)) + ((up + width) + 1)%width;
         let ll = (width * (down/width)) + ((down + width) - 1)%width;
         let lr = (width * (down/width)) + ((down + width) + 1)%width;

         let sum =
            grid.cells[ul as usize].alive + 
            grid.cells[up as usize].alive +
            grid.cells[ur as usize].alive + 
            grid.cells[left as usize].alive +
            grid.cells[right as usize].alive +
            grid.cells[ll as usize].alive +
            grid.cells[down as usize].alive +
            grid.cells[lr as usize].alive;

         let alive = 
            if cell.alive == 1 && (sum < 2 || sum > 3) {
               0
            }
            else if cell.alive == 0 && sum == 3 {
               1
            }
            else {
               cell.alive
            };

         next_state.push(Cell::new(
            cell.position[0],
            cell.position[1],
            cell.draw_coord[0],
            cell.draw_coord[1],
            cell.deltas[0],
            cell.deltas[1],
            alive
         ));
      }

      grid.cells = next_state;
   }
}

fn main() {
   let width = 720.0f64;
   let height = 720.0f64;
   let title = "Conway's Game of Life";

   let (mut events_loop, display) = create_display(width as f64, height as f64, title);

   let shaders = Shaders::new();

   let program =
      glium::Program::from_source(&display, shaders.vert, shaders.frag, None).unwrap();

   main_loop(&mut events_loop, &program, &display);
}

fn main_loop(
   events_loop: &mut glium::glutin::EventsLoop,
   program: &glium::Program,
   display: &glium::Display)
{
   use glium::glutin::{ Event, WindowEvent };

   let rows = 50;
   let columns = 50;

   let mut grid = Grid::new(rows, columns);

   let mut closed = false;
   while !closed {
      Grid::draw_grid(&grid, &display, program);

      events_loop.poll_events(|ev| match ev {
         Event::WindowEvent { event, .. } => match event {
            WindowEvent::CloseRequested => closed = true,
            _ => (),
         },
         _ => (),
      });

      Grid::next_state(&mut grid);
   }
}

fn create_display(width: f64, height: f64, title: &str) -> (glium::glutin::EventsLoop, glium::Display) {
   use glium::glutin::{
      EventsLoop,
      dpi,
      ContextBuilder,
      WindowBuilder
   };

   let events_loop = EventsLoop::new();
   let wb = WindowBuilder::new()
      .with_dimensions(dpi::LogicalSize::new(width as f64, height as f64))
      .with_title(title);
   let cb = ContextBuilder::new();
   let display = glium::Display::new(wb, cb, &events_loop).unwrap();

   (events_loop, display)
}