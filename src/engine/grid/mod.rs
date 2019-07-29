mod vertex;

use glium;

pub struct Grid {
   width: i32,
   height: i32,
   positions: Vec<[f32; 2]>,
   deltas: [f32; 2],
   alive: Vec<u32>
}

impl Grid {
   pub fn create_grid() -> Grid {
      let rows = 400;
      let columns = 400;

      Grid::new(rows, columns)
   }

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

   pub fn draw_grid(grid: &Grid, display: &glium::Display, program: &glium::Program) {
      use glium::Surface;
      use glium::{
         VertexBuffer,
         IndexBuffer,
         index::PrimitiveType
      };

      use vertex::Vertex;

      let mut frame = display.draw();
      frame.clear_color(0.0, 0.0, 0.0, 1.0);

      let cells = &grid.positions;
      let mut verts: Vec<Vertex> = Vec::new();
      let mut inds: Vec<u32> = Vec::new();

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

      frame
         .draw(
            &vertex_buffer,
            &index_buffer,
            program,
            &glium::uniforms::EmptyUniforms,
            &Default::default(),
         ) .unwrap();

      frame.finish().unwrap();
   }

   pub fn next_state(grid: &mut Grid) {
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

   pub fn clear_grid(grid: &mut Grid) {
      let cleared = vec![0; grid.alive.len()];

      grid.alive = cleared;
   }
}
