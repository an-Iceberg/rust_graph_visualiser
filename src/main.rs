mod graph;
mod ui;
mod utils;

use crate::utils::Mode;
use egui_macroquad::draw;
use graph::*;
use macroquad::prelude::*;

fn window_configuration() -> Conf
{
  return Conf
  {
    window_title: "Graph Visualiser".to_string(),
    window_width: 1290,
    window_height: 720,
    fullscreen: false,
    window_resizable: false,
    ..Conf::default()
  };
}

// FIX: path from 6 to 18 is wrong, there exist a much shorter one (point 10 seems to cause that somehow)
#[macroquad::main(window_configuration)]
async fn main()
{
  let mut graph = ShortestPathGraph::new();
  let mut path: Option<Vec<u8>> = None;
  let mut start: Option<usize> = None;
  let mut end: Option<usize> = None;
  let mut hovered_point_id: Option<usize> = None;
  let mut selected_point_id: Option<usize> = None;

  let ui_width = 200;
  let mut mode = Mode::Move;

  let mut padding: u8 = 3;
  let mut angle: f32 = 0.436;
  let mut arrow_head_length: f32 = 20;
  let mut radius = 13_u8;
  let mut line_length = 1_u16;
  let mut path_thickness = 2_f32;
  let mut base_point = 15_f32;

  let mut bg_color: [f32;3] = [0.25_f32, 0., 0.5];
  let mut path_color: [f32;3] = [0., 1., 0.];
  let mut line_color: [f32;3] = [1., 0.5, 0.];
  let mut point_color: [f32;3] = [0., 1., 1.];

  loop
  {
    clear_background(Color::from_vec(Vec4::new(bg_color[0], bg_color[1], bg_color[2], 1.)));

    // Delete or backspace clears the graph of all points and lines
    if is_key_pressed(KeyCode::Backspace) || is_key_pressed(KeyCode::Delete)
    { graph.clear(); }

    // --- INPUT ---
    if utils::is_point_in_rectangle(
      mouse_position().0 as i32,
      mouse_position().1 as i32,
      graph.get_radius() as i32,
      graph.get_radius() as i32,
      screen_width() as i32 - (ui_width + (3 * graph.get_radius() as i32)),
      screen_height() as i32 - (2 * graph.get_radius() as i32),
    )
    { utils::handle_mouse_input(&mode, &mut graph); }

    // --- GUI ---
    // TODO: style the GUI
    ui::paint_ui(&mut mode, &mut graph, &mut bg_color);

    // ! dbg
    if is_key_pressed(KeyCode::P)
    {
      println!("Graph data:");
      graph.print_graph_data();
      println!("Path data:");
      graph.print_path();
    }

    graph.paint_graph();

    draw();

    next_frame().await;
  }
}
