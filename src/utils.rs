use crate::graph::DijkstraGraph;
use macroquad::{
  prelude::{
    is_mouse_button_down, is_mouse_button_pressed, is_mouse_button_released, mouse_position, Color, MouseButton,
  },
  shapes::{draw_circle, draw_rectangle, draw_circle_lines, draw_line, draw_triangle}, text::{get_text_center, draw_text, measure_text}, math::{Vec4, Vec2}, color::{YELLOW, BLACK, MAGENTA, GREEN},
};
use std::ops::{Div, Mul};
use crate::Mode;

// TODO: rework everything
// TODO: make points drawable as hexagons

pub(crate) fn is_point_in_circle(
  point_x: f32, point_y: f32,
  circle_x: f32, circle_y: f32,
  circle_radius: f32
) -> bool
{
  return (circle_x - point_x).powf(2_f32) + (circle_y - point_y).powf(2_f32) <= circle_radius.powf(2_f32);
}

pub(crate) fn is_point_in_rectangle(
  point_x: f32, point_y: f32,
  rectangle_x: f32, rectangle_y: f32,
  rectangle_width: f32,
  rectangle_height: f32,
) -> bool
{
  if point_x < rectangle_x
    || point_y < rectangle_y
    || point_x > (rectangle_x + rectangle_width)
    || point_y > (rectangle_y + rectangle_height)
  { return false; }

  return true;
}

pub(crate) fn draw_pill(x: f32, y: f32, width: f32, height: f32, color: Color)
{
  draw_rectangle(x, y, width, height, color);
  draw_circle(x, y + height.div(2.0), height.div(2.0), color);
  draw_circle(x + width, y + height.div(2.0), height.div(2.0), color);
}

pub(crate) fn handle_mouse_input(
  mode: &Mode,
  graph: &mut DijkstraGraph,
  hovered_point_id: &Option<usize>,
  &mut mut selected_point_id: &mut Option<usize>,
  line_length: &mut u16
)
{
  use Mode::{Move, Line, Point, Path};

  match (
    mode,
    is_mouse_button_pressed(MouseButton::Left),
    is_mouse_button_down(MouseButton::Left),
    is_mouse_button_released(MouseButton::Left),
    is_mouse_button_pressed(MouseButton::Right),
    hovered_point_id,
    selected_point_id,
  )
  {
    // --- MOVE ---
    // FIX: can't move points around

    // Select a point to be moved around
    (Move, true, _, _, false, Some(hovered_point_id), _) =>
      selected_point_id = Some(*hovered_point_id),

    // Move a point around
    (Move, _, true, _, false, _, selected_point_id) =>
    {
      if let Some(selected_point_id) = selected_point_id
      {
        if let Some(point) = graph.get(selected_point_id)
        { point.position().set(mouse_position().0, mouse_position().1); }
      }
    },

    // Releasing the selected point
    (Move, _, _, true, false, _, _) =>
      selected_point_id = None,

    // --- POINT ---

    // Create a point
    (Point, true, _, _, false, None, None) =>
      graph.append_point(mouse_position().0, mouse_position().1),

    // Remove a point
    (Point, false, _, _, true, Some(hovered_point_id), _) =>
      graph.remove_point(*hovered_point_id),

    // --- LINE ---

    // Select a point to draw a line from
    (Line, true, _, _, false, Some(hovered_point_id), None) =>
      selected_point_id = Some(*hovered_point_id),

    // Unset the selected point if no other point is clicked on
    (Line, true, _, _, _, None, Some(_)) | (Mode::Line, _, _, _, true, None, Some(_)) =>
      selected_point_id = None,

    // Select a point to draw the line to
    (Line, true, _, _, false, Some(hovered_point_id), mut selected_point_id) =>
    {
      if selected_point_id.is_none()
      { return; }

      graph.add_line(selected_point_id.unwrap(), *hovered_point_id, *line_length);
      selected_point_id = None;
    },

    // Deletes the selected line
    (Line, false, _, _, true, Some(hovered_point_id), mut selected_point_id) =>
    {
      if selected_point_id.is_none()
      { return; }

      graph.remove_line(selected_point_id.unwrap(), *hovered_point_id);
      selected_point_id = None;
    },

    // --- PATH ---

    // Select a start point with left click
    (Path, true, _, _, false, Some(hovered_point_id), None) =>
    {
      graph.set_start(*hovered_point_id);
      graph.clear_path();
    },

    // Unsetting the start point
    (Path, true, _, _, false, None, None) =>
    {
      graph.clear_start();
      graph.clear_path();
    },

    // Select an end point with right click
    (Path, false, _, _, true, Some(hovered_point_id), None) =>
    {
      graph.set_end(*hovered_point_id);
      graph.clear_path();
    },

    // Unsetting the end point
    (Path, false, _, _, true, None, None) =>
    {
      graph.clear_end();
      graph.clear_path();
    },

    (_, _, _, _, _, _, _) => (),
  }
}

pub(crate) fn paint_graph(
  graph: &DijkstraGraph,
  radius: &f32,
  path_thickness: &f32,
  padding: &u8,
  angle: &f32,
  base_point: &f32,
  arrow_head_length: &f32,
  hovered_point_id: &Option<usize>,
  selected_point_id: &Option<usize>,
  path_color: &[f32;3],
  line_color: &[f32;3],
  point_color: &[f32;3]
)
{
  // Paint lines
  paint_lines(graph, line_color, path_thickness);
  paint_path(graph, path_color, path_thickness);
  paint_arrow_heads(graph, radius, angle, arrow_head_length, base_point, line_color);
  paint_line_lengths(graph, padding, line_color);

  // Paint points
  paint_points(graph, radius, hovered_point_id, selected_point_id, point_color);

  // TODO: consider replacing this with Option::inspect
  // Paints start label
  if let Some(start_id) = graph.start()
  {
    if let Some(start_point) = graph.get(start_id)
    { paint_label("Start", start_point.position.get(), &radius, &padding); }
  }

  // TODO: consider replacing this with Option::inspect
  // Paints end label
  if let Some(end_id) = graph.end()
  {
    if let Some(end_point) = graph.get(end_id)
    { paint_label("End", end_point.position.get(), &radius, &padding); }
  }
}

/// The `position` is the center of the point over which the label is painted.
fn paint_label(text: &str, position: (f32, f32), radius: &f32, padding: &u8)
{
  let text_center = get_text_center(text, None, 20, 1.0, 0.0);
  let text_dimensions = measure_text(text, None, 20, 1.0);

  // A 2 pixel gap between the label and the point is hard-coded
  draw_pill(
    position.0 - text_dimensions.width.div(2.0),
    position.1 - text_dimensions.height - radius - padding.mul(2) as f32 - 2.0,
    text_dimensions.width,
    text_dimensions.height + padding.mul(2) as f32,
    GREEN,
  );

  draw_text(
    text,
    position.0 - text_center.x,
    position.1 - text_center.y - radius - text_dimensions.height.div(2.0) - *padding as f32 - 2.0,
    20.0,
    Color::from_rgba(20, 0, 40, 255),
  );
}

fn paint_path(graph: &DijkstraGraph, path_color: &[f32;3], path_thiccness: &f32)
{
  let path = graph.get_path().unwrap_or_else(|| vec![]);

  path.iter().zip(path.iter().skip(1))
    .map(|(from_id, to_id)| (graph.get(*from_id), graph.get(*to_id)))
    .filter(|(from_option, to_option)| from_option.is_some() && to_option.is_some())
    .map(|(from_option, to_option)| (from_option.as_ref().unwrap(), to_option.as_ref().unwrap()))
    .for_each(|(from, to)|
    {
      draw_line(
        from.position.x,
        from.position.y,
        to.position.x,
        to.position.y,
        *path_thiccness,
        Color::from_vec(Vec4::new(path_color[0],path_color[1],path_color[2], 1.,)),
      );
    });
}

fn paint_points(
  graph: &DijkstraGraph,
  radius: &f32,
  mut hovered_point_id: &Option<usize>,
  selected_point_id: &Option<usize>,
  point_color: &[f32;3]
)
{
  graph.points()
    .iter()
    .enumerate()
    .filter(|(_, point)| point.is_some())
    .map(|(id, point_option)| (id, point_option.as_ref().unwrap()))
    .for_each(|(id, point)|
    {
      // Drawing the points
      draw_circle(
        point.position.x,
        point.position.y,
        *radius,
        Color::from_vec(Vec4::new(point_color[0], point_color[1], point_color[2], 1_f32))
      );

      let text_center = get_text_center(id.to_string().as_str(), None, 20, 1.0, 0.0);

      // Drawing the point id
      draw_text(
        id.to_string().as_str(),
        point.position.x - text_center.x,
        point.position.y - text_center.y,
        20.0,
        BLACK,
      );
    });

  // Drawing the selected point differently
  if let Some(selected_point_id) = selected_point_id
  {
    if let Some(point) = graph.get(*selected_point_id)
    {
      draw_circle(
        point.position.x,
        point.position.y,
        *radius, YELLOW
      );
    }
  }

  // Drawing an outline around the hovered point
  if let Some(hovered_point_id) = hovered_point_id
  {
    if let Some(point) = graph.get(*hovered_point_id)
    {
      draw_circle_lines(
        point.position.x,
        point.position.y,
        *radius + 4_f32, 1_f32, MAGENTA
      );
    }
  }

  // Reset the hovered point id
  hovered_point_id = &None;
}

// FIX: draw arrow heads properly
fn paint_arrow_heads(
  graph: &DijkstraGraph,
  radius: &f32,
  angle: &f32,
  arrow_head_length: &f32,
  base_point: &f32,
  line_color: &[f32;3]
)
{
  graph.lines()
    .iter()
    .for_each(|(from_id, from, length, to_id, to)|
    {
      let mut direction = Vec2
      {
        x: from.position.x - to.position.x,
        y: from.position.y - to.position.y
      };

      direction = direction.normalize();

      // ? Maybe use arrow_head_length instead of direction.length()
      // Calculating the tip of the triangle that touches the node (position + (direction * (radius / length)))
      let arrow_head_location = Vec2
      {
        x: to.position.x + (direction.x * ((radius + 2.) / direction.length())),
        y: to.position.y + (direction.y * ((radius + 2.) / direction.length())),
      };

      // This point is at the base of the arrow head that "connects" it to the line
      let helper_point = Vec2
      {
        x: to.position.x + (direction.x * ((radius + base_point) / direction.length())),
        y: to.position.y + (direction.y * ((radius + base_point) / direction.length())),
      };

      /*
      draw_line(
        from.position.x + (direction.x * (-(self.radius) / direction.length())),
        from.position.y + (direction.y * (-(self.radius) / direction.length())),
        arrow_head_location.x as f32,
        arrow_head_location.y as f32,
        1.0,
        Color::from_vec(Vec4::new(self.line_color[0], self.line_color[1], self.line_color[2], 1.)),
      );
      */

      /*
        x1/y1 are the start of the line, x2/y2 are the end of the line where the head of the arrow should be
        L1 is the length from x1/y1 to x2/y2
        L2 is the length of the arrow head
        a is the angle

        Formula:
        x3 = x2 + L2/L1 * [(x1 - x2) * cos(a) + (y1 - y2) * sin(a)]
        y3 = y2 + L2/L1 * [(y1 - y2) * cos(a) - (x1 - x2) * sin(a)]
        x4 = x2 + L2/L1 * [(x1 - x2) * cos(a) - (y1 - y2) * sin(a)]
        y4 = y2 + L2/L1 * [(y1 - y2) * cos(a) + (x1 - x2) * sin(a)]

        Source: https://math.stackexchange.com/questions/1314006/drawing-an-arrow
      */

      // TODO: extract common stuff from these
      // Left arrow head wing
      draw_triangle(
        arrow_head_location,
        helper_point,
        Vec2
        {
          x: arrow_head_location.x + ((arrow_head_length / direction.length()) * (((from.position.x - to.position.x) * angle.cos()) - ((from.position.y - to.position.y) * angle.sin()))),
          y: arrow_head_location.y + ((arrow_head_length / direction.length()) * (((from.position.y - to.position.y) * angle.cos()) + ((from.position.x - to.position.x) * angle.sin()))),
        },
        Color::from_vec(Vec4::new(line_color[0], line_color[1], line_color[2], 1.)),
      );

      // Right arrow head wing
      draw_triangle(
        arrow_head_location,
        helper_point,
        Vec2
        {
          x: arrow_head_location.x + ((arrow_head_length / direction.length()) * (((from.position.x - to.position.x) * angle.cos()) + ((from.position.y - to.position.y) * angle.sin()))),
          y: arrow_head_location.y + ((arrow_head_length / direction.length()) * (((from.position.y - to.position.y) * angle.cos()) - ((from.position.x - to.position.x) * angle.sin()))),
        },
        Color::from_vec(Vec4::new(line_color[0], line_color[1], line_color[2], 1.)),
      );
    });
}

fn paint_lines(graph: &DijkstraGraph, line_color: &[f32;3], path_thickness: &f32)
{
  graph.lines()
    .iter()
    .for_each(|(from_id, from, length, to_id, to)|
    {
      draw_line(
        from.position.x,
        from.position.y,
        to.position.x,
        to.position.y,
        *path_thickness,
        Color::from_vec(Vec4::new(line_color[0], line_color[1], line_color[2], 1.0)),
      );
    });
}

fn paint_line_lengths(graph: &DijkstraGraph, padding: &u8, line_color: &[f32;3])
{
  graph.lines()
    .iter()
    .for_each(|(from_id, from, distance, to_id, to)|
    {
      let position = Vec2
      {
        x: ((1.0 / 3.0) * from.position.x + (2.0 / 3.0) * to.position.x),
        y: ((1.0 / 3.0) * from.position.y + (2.0 / 3.0) * to.position.y),
      };

      let text_center = get_text_center(distance.to_string().as_str(), None, 20, 1.0, 0.0);
      let text_dimensions = measure_text(distance.to_string().as_str(), None, 20, 1.0);

      draw_pill(
        position.x - text_dimensions.width.div(2.0),
        position.y - text_dimensions.height.div(2.0) - *padding as f32,
        text_dimensions.width,
        text_dimensions.height + padding.mul(2) as f32,
        Color::from_hex(0x00ffff)
      );

      draw_text(
        distance.to_string().as_str(),
        position.x - text_center.x,
        position.y - text_center.y,
        20.0,
        BLACK,
      );
    });
}

// Tests
#[path = "./tests/utils_tests.rs"]
#[cfg(test)]
mod utils_tests;
