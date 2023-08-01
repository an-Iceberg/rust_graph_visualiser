use crate::graph::Graph;
use macroquad::{
  prelude::{
    is_mouse_button_down, is_mouse_button_pressed, is_mouse_button_released, mouse_position, Color, IVec2, MouseButton,
  },
  shapes::{draw_circle, draw_rectangle},
};
use std::ops::Div;

pub(crate) fn is_point_in_circle(point_x: i32, point_y: i32, circle_x: i32, circle_y: i32, circle_radius: i32) -> bool {
  return (circle_x - point_x).pow(2) + (circle_y - point_y).pow(2) <= circle_radius.pow(2);
}

pub(crate) fn is_point_in_rectangle(
  point_x: i32, point_y: i32, rectangle_x: i32, rectangle_y: i32, rectangle_width: i32, rectangle_height: i32,
) -> bool {
  if point_x < rectangle_x
    || point_y < rectangle_y
    || point_x > (rectangle_x + rectangle_width)
    || point_y > (rectangle_y + rectangle_height)
  {
    return false;
  }

  return true;
}

pub(crate) fn draw_pill(x: f32, y: f32, width: f32, height: f32, color: Color) {
  draw_rectangle(x, y, width, height, color);
  draw_circle(x, y + height.div(2.0), height.div(2.0), color);
  draw_circle(x + width, y + height.div(2.0), height.div(2.0), color);
}

pub(crate) fn handle_mouse_input(mode: &Mode, graph: &mut Graph) {
  match (
    mode,
    is_mouse_button_pressed(MouseButton::Left),
    is_mouse_button_down(MouseButton::Left),
    is_mouse_button_released(MouseButton::Left),
    is_mouse_button_pressed(MouseButton::Right),
    graph.get_hovered_point_id(),
    graph.selected_point_id,
  ) {
    // --- MOVE ---

    // Select a point to be moved around
    (Mode::Move, true, _, _, false, Some(hovered_point_id), _) => {
      graph.selected_point_id = Some(hovered_point_id);
    },

    // Move a point around
    (Mode::Move, _, true, _, false, _, Some(selected_point_id)) => {
      graph.set_point_coordinates(
        selected_point_id,
        IVec2 {
          x: mouse_position().0 as i32,
          y: mouse_position().1 as i32,
        },
      );
    },

    // Releasing the selected point
    (Mode::Move, _, _, true, false, _, _) => {
      graph.selected_point_id = None;
    },

    // --- POINT ---

    // Create a point
    (Mode::Point, true, _, _, false, None, None) => {
      graph.add_point(IVec2 {
        x: mouse_position().0 as i32,
        y: mouse_position().1 as i32,
      });
    },

    // Remove a point
    (Mode::Point, false, _, _, true, Some(hovered_point_id), _) => {
      graph.remove_point(hovered_point_id);
    },

    // --- LINE ---

    // Select a point to draw a line from
    (Mode::Line, true, _, _, false, Some(hovered_point_id), None) => {
      graph.selected_point_id = Some(hovered_point_id);
    },

    // Unset the selected point if no other point is clicked on
    (Mode::Line, true, _, _, _, None, Some(_)) | (Mode::Line, _, _, _, true, None, Some(_)) => {
      graph.selected_point_id = None;
    },

    // Select a point to draw the line to
    (Mode::Line, true, _, _, false, Some(hovered_point_id), Some(selected_point_id)) => {
      graph.add_line(selected_point_id, hovered_point_id);
      graph.selected_point_id = None;
    },

    // Deletes the selected line
    (Mode::Line, false, _, _, true, Some(hovered_point_id), Some(selected_point_id)) => {
      graph.remove_line(selected_point_id, hovered_point_id);
      graph.selected_point_id = None;
    },

    // --- PATH ---

    // Select a start point with left click
    (Mode::Path, true, _, _, false, Some(hovered_point_id), None) => {
      graph.start = Some(hovered_point_id);
      graph.clear_path();
    },

    // Unsetting the start point
    (Mode::Path, true, _, _, false, None, None) => {
      graph.start = None;
      graph.clear_path();
    },

    // Select an end point with right click
    (Mode::Path, false, _, _, true, Some(hovered_point_id), None) => {
      graph.end = Some(hovered_point_id);
      graph.clear_path();
    },

    // Unsetting the end point
    (Mode::Path, false, _, _, true, None, None) => {
      graph.end = None;
      graph.clear_path();
    },

    (_, _, _, _, _, _, _) => (),
  }
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub(crate) enum Mode {
  Move,
  Point,
  Line,
  Path,
}

// Tests
#[path = "./tests/utils.rs"]
#[cfg(test)]
mod utils;
