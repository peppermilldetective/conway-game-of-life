mod engine;

fn main() {
   let width = 720.0f64;
   let height = 720.0f64;
   let title = "Conway's Game of Life";

   engine::start(width, height, title);
}