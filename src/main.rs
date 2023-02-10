mod config;
mod graph;

use graph::*;
use macroquad::prelude::*;
use crate::config::window_config;

#[macroquad::main(window_config)]
async fn main()
{
  let center = get_text_center("Hello World", Option::None, 32, 1.0, 0.0);
  println!("X:{}, Y:{}", center.x, center.y);

  let graph = Graph::new();

  loop
  {
    clear_background(BLACK);

    next_frame().await;
  }
}
