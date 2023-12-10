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
  let mut graph = DijkstraGraph::new();
  let mut path: Option<Vec<u8>> = None;
  // let mut start: Option<usize> = None;
  // let mut end: Option<usize> = None;
  // This is the id of the point that the mouse is currently hovering over
  let mut hovered_point_id: Option<usize> = None;
  // This is the id of the point the mouse is currently hovering over and mouse 1 is pressed
  let mut selected_point_id: Option<usize> = None;

  let ui_width = 200_f32;
  let mut mode = Mode::Move;

  let mut padding = 3_u8;
  let mut angle = 0.436_f32;
  let mut arrow_head_length = 20_f32;
  let mut radius = 13_f32;
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
      mouse_position().0,
      mouse_position().1,
      radius,
      radius,
      screen_width() - (ui_width + (3_f32 * radius)),
      screen_height() - (2_f32 * radius),
    )
    {
      utils::handle_mouse_input(
        &mode,
        &mut graph,
        &mut hovered_point_id,
        &mut selected_point_id,
        &mut line_length
      );
    }

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

    utils::paint_graph(
      &graph, &radius,
      &path_thickness,
      &padding,
      &hovered_point_id,
      &selected_point_id,
      &path_color,
      &line_color,
      &point_color
    );

    draw();

    next_frame().await;
  }
}
