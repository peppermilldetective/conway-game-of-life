mod grid;
mod shaders;

pub fn start(width: f64, height: f64, title: &str) {
   use shaders::Shaders;
   let (mut events_loop, display) = create_display(width as f64, height as f64, title);

   let shaders = Shaders::new();

   let program =
      glium::Program::from_source(&display, shaders.vert, shaders.frag, None).unwrap();

   main_loop(&mut events_loop, &program, &display);
}

#[cfg_attr(feature="cargo-clippy", allow(clippy::single_match))]
fn main_loop(
   events_loop: &mut glium::glutin::EventsLoop,
   program: &glium::Program,
   display: &glium::Display)
{
   use glium::glutin::{ Event, WindowEvent };
   use grid::Grid;

   let mut grid = Grid::create_grid();

   let mut closed = false;
   let mut paused = false;
   let mut input_taken = false;
   while !closed {
      Grid::draw_grid(&grid, &display, program);

      events_loop.poll_events(|ev| match ev {
         Event::WindowEvent { event, .. } => match event {
            WindowEvent::CloseRequested => closed = true,
            WindowEvent::KeyboardInput { input, .. } => {
               if !input_taken && input.scancode == 57 { // SPACE
                  paused = !paused;
                  input_taken = true;
               }
               else if !input_taken && input.scancode == 19 { // 'R' Key
                  grid = Grid::create_grid();
                  input_taken = true;
               }
               else if !input_taken && input.scancode == 32 { // 'D' Key
                  Grid::clear_grid(&mut grid);
                  input_taken = true;
               }
               else {
                  input_taken = false;
               }
            },
            _ => (),
         },
         _ => (),
      });

      if !paused {
         Grid::next_state(&mut grid);
      }
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