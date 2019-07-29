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

struct Grid {
   width: i32,
   height: i32,
   positions: Vec<[f32; 2]>,
   deltas: [f32; 2],
   alive: Vec<u32>
}

impl Grid {
   fn new(width: i32, height: i32) -> Grid {
      let mut positions: Vec<[f32; 2]> = Vec::new();
      let mut alive: Vec<u32> = Vec::new();

      let d_x = 2.0 / width as f32;
      let d_y = 2.0 / height as f32;

      for y in 0..height {
         for x in 0..width {
            let draw_x = -1.0 + (d_x * x as f32);
            let draw_y = -1.0 + (d_y * y as f32);

            positions.push([
               draw_x,
               draw_y
            ]);


            alive.push(rand::random::<u32>() % 2);
         }
      }

      Grid {
         width,
         height,
         positions,
         deltas: [d_x, d_y],
         alive
      }
   }

   fn draw_grid(grid: &Grid, display: &glium::Display, program: &glium::Program) {
      use glium::Surface;
      use glium::{
         VertexBuffer,
         IndexBuffer,
         index::PrimitiveType
      };

      let mut frame = display.draw();
      frame.clear_color(0.0, 0.0, 0.0, 1.0);

      let cells = &grid.positions;
      let mut verts: Vec<Vertex> = Vec::new();
      let mut inds: Vec<u16> = Vec::new();

      let d_x = grid.deltas[0];
      let d_y = grid.deltas[1];

      let mut ind = 0;
      for (i, cell) in cells.iter().enumerate() {
         if grid.alive[i] == 0 {
            continue;
         }

         let x = cell[0];
         let y = cell[1];

         verts.push(Vertex { position: [ x      , y      ] });
         verts.push(Vertex { position: [ x + d_x, y      ] });
         verts.push(Vertex { position: [ x      , y + d_y] });
         verts.push(Vertex { position: [ x + d_x, y + d_y] });

         inds.push(4*ind);
         inds.push(4*ind + 1);
         inds.push(4*ind + 2);
         inds.push(4*ind + 1);
         inds.push(4*ind + 2);
         inds.push(4*ind + 3);

         ind += 1;
      }

      let (vertex_buffer, index_buffer) = (
         VertexBuffer::new(display, &verts).unwrap(),
         IndexBuffer::new(display, PrimitiveType::TrianglesList, &inds).unwrap()
      );

      let uniforms = uniform! {
      };

      frame
         .draw(
            &vertex_buffer,
            &index_buffer,
            program,
            &uniforms,
            &Default::default(),
         ) .unwrap();

      frame.finish().unwrap();
   }

   fn next_state(grid: &mut Grid) {
      let mut next_state: Vec<u32> = Vec::new();

      for (i, cell_alive) in grid.alive.iter().enumerate() {
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
            grid.alive[ul as usize] + 
            grid.alive[up as usize] +
            grid.alive[ur as usize] + 
            grid.alive[left as usize] +
            grid.alive[right as usize] +
            grid.alive[ll as usize] +
            grid.alive[down as usize] +
            grid.alive[lr as usize];

         let alive = 
            if *cell_alive == 1u32 && (sum < 2 || sum > 3) {
               0u32
            }
            else if *cell_alive == 0u32 && sum == 3 {
               1u32
            }
            else {
               *cell_alive
            };

         next_state.push(alive);
      }

      grid.alive = next_state;
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