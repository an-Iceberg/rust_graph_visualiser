mod graph;
mod ui;
mod utils;

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

#[derive(PartialEq, Eq, Clone, Copy)]
pub(crate) enum Mode {
  Move,
  Point,
  Line,
  Path,
}

// TODO: HEXAGONS
// FIX: path from 6 to 18 is wrong, there exist a much shorter one (point 10 seems to cause that somehow)
#[macroquad::main(window_configuration)]
async fn main()
{
  let mut graph = DijkstraGraph::new();
  // let mut start: Option<usize> = None;
  // let mut end: Option<usize> = None;
  // This is the id of the point that the mouse is currently hovering over
  let mut hovered_point_id: Option<usize> = None;
  // This is the id of the point the mouse is currently hovering over and mouse 1 is pressed
  let mut selected_point_id: Option<usize> = None;

  let ui_width: f32 = 200.;
  let mut mode = Mode::Move;

  let padding: u8 = 3;
  let mut angle: f32 = 0.436;
  let mut arrow_head_length: f32 = 20.;
  let mut radius: f32 = 13.;
  let mut line_length: u16 = 1;
  let mut path_thickness: f32 = 2.;
  let mut base_point: f32 = 15.;

  let mut bg_color: u32 = 0x400080;
  let mut path_color: u32 = 0x00ff00;
  let mut line_color: u32 = 0xff8000;
  let mut point_color: u32 = 0x00ffff;

  loop
  {
    clear_background(Color::from_hex(bg_color));

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
      hovered_point_id = graph.find_hovered_point(mouse_position().0, mouse_position().1, radius);
      utils::handle_mouse_input(
        is_mouse_button_pressed(MouseButton::Left),
        is_mouse_button_down(MouseButton::Left),
        is_mouse_button_released(MouseButton::Left),
        is_mouse_button_pressed(MouseButton::Right),
        &mode,
        &mut graph,
        &mut hovered_point_id,
        &mut selected_point_id,
        &mut line_length
      );
    }

    // --- GUI ---
    // TODO: style the GUI
    ui::paint_ui(
      &mut mode,
      &mut graph,
      &mut radius,
      &mut angle,
      &mut arrow_head_length,
      &mut path_thickness,
      &mut base_point,
      &mut selected_point_id,
      &mut line_length,
      &mut path_color,
      &mut line_color,
      &mut point_color,
      &mut bg_color
    );

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
      &angle,
      &base_point,
      &arrow_head_length,
      &mut hovered_point_id,
      &selected_point_id,
      &path_color,
      &line_color,
      &point_color
    );

    draw();

    next_frame().await;
  }
}
