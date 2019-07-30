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
   use glium::glutin::{ Event, WindowEvent, dpi, MouseButton, ElementState };
   use grid::Grid;

   let mut grid = Grid::create_grid();

   let window_dim = display.gl_window().window().get_outer_size().unwrap();
   
   let mut cursor_pos = dpi::LogicalPosition::new(0.0, 0.0);

   let mut running = true;
   let mut paused = false;
   let mut input_taken = false;
   while running {
      Grid::draw_grid(&grid, &display, program);

      std::thread::sleep(std::time::Duration::from_millis(250));

      events_loop.poll_events(|ev| if let Event::WindowEvent { event, .. } = ev {
         match event {
            // WINDOW EVENTS

            WindowEvent::CloseRequested => {
               running = false
            },

            // KEYBOARD EVENTS

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
               else if !input_taken && input.scancode == 16 { // 'Q' Key
                  running = false;
               }
               else {
                  input_taken = false;
               }
            },

            // MOUSE EVENTS

            WindowEvent::CursorMoved { position, .. } => {
               cursor_pos = position;
            },
            WindowEvent::MouseInput { state, button, .. } => {
               if button == MouseButton::Left && state == ElementState::Pressed {
                  Grid::process_click(
                     &mut grid,
                     cursor_pos.x,
                     cursor_pos.y,
                     window_dim.width,
                     window_dim.height);
               }
            }

            _ => (),
         }
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
      .with_title(title)
      .with_resizable(false);
   let cb = ContextBuilder::new();
   let display = 
      glium::Display::new(wb, cb, &events_loop).unwrap();

   display.gl_window().window().set_position(dpi::LogicalPosition::new(50f64, 50f64));

   (events_loop, display)
}