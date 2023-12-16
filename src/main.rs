mod graph;
mod ui;
mod utils;
#[path ="icons/small.rs"]
mod small_icon;
#[path ="icons/medium.rs"]
mod medium_icon;
#[path ="icons/big.rs"]
mod big_icon;

use egui_macroquad::draw;
use graph::*;
use macroquad::{prelude::*, telemetry::disable, miniquad::conf::Icon};
// use std::{fs::File, io::Write};
// use image;

fn window_configuration() -> Conf
{
  // This is used to generate the icons for macroquad
  // The output is put into an array and baked into the executable
  /*
  let small: [u8; 1024] = image::open("16x16.png").unwrap().to_rgba8().to_vec().try_into().unwrap();
  let medium: [u8; 4096] = image::open("32x32.png").unwrap().to_rgba8().to_vec().try_into().unwrap();
  let big: [u8; 16384] = image::open("64x64.png").unwrap().to_rgba8().to_vec().try_into().unwrap();

  let mut small_file = File::create("small.txt").unwrap();
  let mut medium_file = File::create("medium.txt").unwrap();
  let mut big_file = File::create("big.txt").unwrap();

  small.iter().for_each(|byte|
  { write!(small_file, "0x{:02x}, ", byte); });
  medium.iter().for_each(|byte|
  { write!(medium_file, "0x{:02x}, ", byte); });
  big.iter().for_each(|byte|
  { write!(big_file, "0x{:02x}, ", byte); });
  */

  return Conf
  {
    window_title: "Rust Graph Visualiser".to_string(),
    window_width: 1290,
    window_height: 720,
    fullscreen: false,
    window_resizable: false,
    icon: Some(Icon
      {
        small: small_icon::give(),
        medium: medium_icon::give(),
        big: big_icon::give()
      }),
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

pub(crate) const VERSION: Option<&str> = option_env!("CARGO_PKG_VERSION");
pub(crate) const AUTHORS: Option<&str> = option_env!("CARGO_PKG_AUTHORS");
pub(crate) const UI_WIDTH: f32 = 200.;
pub(crate) const PADDING: u8 = 3;
pub(crate) const BG_COLOR: u32 = 0x400080;
pub(crate) const PATH_COLOR: u32 = 0x00ff00;
pub(crate) const LINE_COLOR: u32 = 0x00c0c0;
pub(crate) const POINT_COLOR: u32 = 0xff8000;
pub(crate) const LINE_LENGTH_COLOR: u32 = 0xc09ac0;
pub(crate) const  UI_SPACING: f32 = 229.;

#[macroquad::main(window_configuration)]
async fn main()
{
  disable();

  let mut graph = DijkstraGraph::new();
  // This is the id of the point that the mouse is currently hovering over
  let mut hovered_point_id: Option<usize> = None;
  // This is the id of the point the mouse is currently hovering over and mouse 1 is pressed
  let mut selected_point_id: Option<usize> = None;

  let mut mode = Mode::Move;

  let mut angle: f32 = 0.436;
  let mut arrow_head_length: f32 = 20.;
  let mut radius: f32 = 13.;
  let mut line_length: u16 = 1;
  let mut path_thickness: f32 = 2.;
  let mut base_point: f32 = 15.;

  let mut hexagons: bool = false;

  loop
  {
    clear_background(Color::from_hex(BG_COLOR));

    // Delete or backspace clears the graph of all points and lines
    if is_key_pressed(KeyCode::Backspace) || is_key_pressed(KeyCode::Delete)
    { graph.clear(); }

    // --- INPUT ---
    if utils::is_point_in_rectangle(
      mouse_position().0,
      mouse_position().1,
      radius,
      radius,
      screen_width() - (UI_WIDTH + (3_f32 * radius)),
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
      &mut hexagons,
    );

    // ! dbg
    if is_key_pressed(KeyCode::P)
    {
      graph.print_graph_data();
    }

    utils::paint_graph(
      &graph, &radius,
      &path_thickness,
      &angle,
      &base_point,
      &arrow_head_length,
      &mut hovered_point_id,
      &selected_point_id,
      &hexagons,
    );

    draw();

    next_frame().await;
  }
}
