mod config;
mod graph;
mod utils;
#[path ="../tests/utils_tests.rs"]
mod utils_tests;

use std::ops::Div;

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

  setup(&mut graph);

  loop
  {
    clear_background(Color::from_rgba(20, 0, 40, 255));

    // Delete or backspace clears the graph of all points and lines
    if is_key_pressed(KeyCode::Backspace) || is_key_pressed(KeyCode::Delete)
    {
      graph.clear();
    }

    // The mouse must be within bounds in order to make these actions
    let position = mouse_position();

    if
      utils::is_point_in_rectangle(
        position.0,
        position.1,
        graph.get_radius() as f32,
        graph.get_radius() as f32,
        screen_width() - (ui_width as f32 + (2.0 * graph.get_radius() as f32)),
        screen_height() - (2.0 * graph.get_radius() as f32)
      )
    {
      match
        (
          mode,
          is_mouse_button_pressed(MouseButton::Left),
          is_mouse_button_down(MouseButton::Left),
          is_mouse_button_released(MouseButton::Left),
          is_mouse_button_pressed(MouseButton::Right),
          graph.get_hovered_point_id(),
          graph.selected_point_id
        )
      {
        // MOVE

        // Select a point to be moved around
        (0, true, _, _, false, Some(hovered_point_id), _) =>
        {
          graph.selected_point_id = Some(hovered_point_id);
        }

        // Move a point around
        (0, _, true, _, false, _, Some(selected_point_id)) =>
        {
          graph.set_point_coordinates(selected_point_id, Vec2 { x: position.0, y: position.1 });
        }

        // Releasing the selected point
        (0, _, _, true, false, _, _) =>
        {
          graph.selected_point_id = None;
        }

        // POINT

        // Create a point
        (1, true, _, _, false, None, None) =>
        {
          graph.add_point(Vec2 { x: position.0, y: position.1 });
        }

        // Remove a point
        (1, false, _, _, true, Some(hovered_point_id), _) =>
        {
          graph.remove_point(hovered_point_id);
        }

        // LINE

        // Select a point to draw a line from
        (2, true, _, _, false, Some(hovered_point_id), None) =>
        {
          graph.selected_point_id = Some(hovered_point_id);
        }

        // Unset the selected point if no other point is clicked on
        (2, true, _, _, _, None, Some(_)) | (2, _, _, _, true, None, Some(_)) =>
        {
          graph.selected_point_id = None;
        }

        // Select a point to draw the line to
        (2, true, _, _, false, Some(hovered_point_id), Some(selected_point_id)) =>
        {
          graph.add_line(selected_point_id, hovered_point_id, line_length as u16);
          graph.selected_point_id = None;
        }

        // Deletes the selected line
        (2, false, _, _, true, Some(hovered_point_id), Some(selected_point_id)) =>
        {
          graph.remove_line(selected_point_id, hovered_point_id);
          graph.selected_point_id = None;
        }

        // PATH

        // Select a start point with left click
        (3, true, _, _, false, Some(hovered_point_id), None) =>
        {
          graph.start = Some(hovered_point_id);
        }

        // Unsetting the start point
        (3, true, _, _, false, None, None) =>
        {
          graph.start = None;
        }

        // Select an end point with right click
        (3, false, _, _, true, Some(hovered_point_id), None) =>
        {
          graph.end = Some(hovered_point_id);
        }

        // Unsetting the end point
        (3, false, _, _, true, None, None) =>
        {
          graph.end = None;
        }

        (_, _, _, _, _, _, _) => ()
      }
    }

    // TODO: extract GUI into separate component
    // TODO: style the GUI
    root_ui().window(hash!(), Vec2 { x: screen_width() - ui_width as f32, y: 0.0 }, Vec2 { x: ui_width as f32, y: screen_height() }, |ui|
    {
      ui.combo_box(hash!(), "Mode", &["Move", "Point", "Line", "Path"], &mut mode);

      match mode
      {
        // Move
        0 =>
        {
          ui.label(None, "Left click: move a point");
        }

        // Point
        1 =>
        {
          ui.label(None, "Left click: create point");
          ui.label(None, "Right click: delete point");
        }

        // Line
        2 =>
        {
          ui.drag(hash!(), "Line Length", Some((1.0, 100.0)), &mut line_length);

          if graph.selected_point_id == None
          {
            ui.label(None, "Left click: selecte a point");
          }
          else
          {
            ui.label(None, "Left click: select another");
            ui.label(None, "  point to make a line");
            ui.label(None, "Right click: select another");
            ui.label(None, "  point to delete a line");
          }
        }

        // Path
        3 =>
        {
          if graph.start != None && graph.end != None
          {
            ui.button(None, "Find shortest path");
          }

          ui.label(None, "Left click: set start");
          ui.label(None, "Right click: set end");
        }

        _ => ()
      }

      // TODO: Add user instructions

      // TODO: Add sample graphs
      // TODO: Adjustements for point radius, arrowhead etc.
      // TODO: FPS
    });

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

  graph.add_line(3, 12, 15);
  graph.add_line(10, 5, 30);
  graph.add_line(9, 1, 43);
  graph.add_line(2, 14, 67);
  graph.add_line(15, 11, 93);
  graph.add_line(6, 13, 9);
  graph.add_line(13, 6, 24);
  graph.add_line(7, 14, 33);
  graph.add_line(9, 15, 47);
}

pub fn draw_pill(x: f32, y: f32, width: f32, height: f32, color: Color)
{
  draw_rectangle(x, y, width, height, color);
  draw_circle(x, y + height.div(2.0), height.div(2.0), color);
  draw_circle(x + width, y + height.div(2.0), height.div(2.0), color);
}
