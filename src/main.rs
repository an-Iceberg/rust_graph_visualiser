mod config;
mod graph;
mod utils;
#[path ="../tests/utils_tests.rs"]
mod utils_tests;

use graph::*;
use macroquad::{prelude::*, ui::root_ui, hash};
use crate::config::window_config;

#[macroquad::main(window_config)]
async fn main()
{
  let mut graph = Graph::new();
  let ui_width = 200;
  let mut line_length = 1.0;
  let mut mode = 0;
  let mut hovered_point_id: Option<u8> = Option::None;

  setup(&mut graph);

  loop
  {
    clear_background(Color::from_rgba(10, 0, 20, 255));

    // TODO: style the GUI
    root_ui().window(hash!(), Vec2 { x: screen_width() - ui_width as f32, y: 0.0 }, Vec2 { x: ui_width as f32, y: screen_height() }, |ui|
    {
      ui.combo_box(hash!(), "Mode", &["Move", "Point", "Line", "Path"], &mut mode);
      if mode == 2
      {
        ui.drag(hash!(), "Line Length", Some((1.0, 100.0)), &mut line_length);
      }
    });

    draw_pill(40.0, 40.0, 40.0, 20.0, ORANGE);
    draw_pill(80.0, 80.0, 80.0, 10.0, Color::from_rgba(0, 255, 255, 255));

    draw_text(format!("{:?}", mouse_position()).as_str(), 0.0, 10.0, 16.0, WHITE);

    graph.paint_graph();

    next_frame().await;
  }
}

fn setup(graph: &mut Graph)
{
  graph.add_point(Vec2 { x: 233.0, y: 73.0 });
  graph.add_point(Vec2 { x: 76.0, y: 234.0 });
  graph.add_point(Vec2 { x: 72.0, y: 150.0 });
  graph.add_point(Vec2 { x: 29.0, y: 495.0 });
  graph.add_point(Vec2 { x: 120.0, y: 684.0 });
  graph.add_point(Vec2 { x: 290.0, y: 521.0 });
  graph.add_point(Vec2 { x: 380.0, y: 537.0 });
  graph.add_point(Vec2 { x: 414.0, y: 127.0 });
  graph.add_point(Vec2 { x: 584.0, y: 88.0 });
  graph.add_point(Vec2 { x: 535.0, y: 514.0 });
  graph.add_point(Vec2 { x: 650.0, y: 616.0 });
  graph.add_point(Vec2 { x: 857.0, y: 529.0 });
  graph.add_point(Vec2 { x: 250.0, y: 309.0 });
  graph.add_point(Vec2 { x: 502.0, y: 343.0 });
  graph.add_point(Vec2 { x: 752.0, y: 251.0 });
}

pub fn draw_pill(x: f32, y: f32, width: f32, height: f32, color: Color)
{
  draw_rectangle(x, y, width, height, color);
  draw_circle(x, y + (height / 2.0), height / 2.0, color);
  draw_circle(x + width, y + (height / 2.0), height / 2.0, color);
}
