use crate::{graph::DijkstraGraph, PADDING, LINE_COLOR, POINT_COLOR, PATH_COLOR, BG_COLOR, LINE_LENGTH_COLOR};
use macroquad::{
  prelude::{
    mouse_position, Color,
  },
  shapes::{draw_circle, draw_rectangle, draw_circle_lines, draw_line, draw_triangle, draw_hexagon}, text::{get_text_center, draw_text, measure_text}, math::{Vec4, Vec2}, color::{YELLOW, BLACK, MAGENTA, GREEN, RED},
};
use std::ops::{Div, Mul};
use crate::Mode;

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
  left_mouse_pressed: bool,
  left_mouse_down: bool,
  left_mouse_released: bool,
  right_mouse_pressed: bool,
  mode: &Mode,
  graph: &mut DijkstraGraph,
  hovered_point_id_option: &Option<usize>,
  selected_point_id_option: &mut Option<usize>,
  line_length: &mut u16
)
{
  use Mode::{Move, Line, Point, Path};

  match (
    mode,
    left_mouse_pressed,
    left_mouse_down,
    left_mouse_released,
    right_mouse_pressed,
    hovered_point_id_option.as_ref(),
    selected_point_id_option.as_ref(),
  )
  {
    // --- MOVE ---

    // Select a point to be moved around
    (Move, true, _, _, false, Some(hovered_point_id), _) =>
      *selected_point_id_option = Some(*hovered_point_id),

    // Move a point around
    (Move, _, true, _, false, _, Some(selected_point_id)) =>
    {
      if let Some(point) = graph.get_mut(*selected_point_id)
      {
        point.x = mouse_position().0;
        point.y = mouse_position().1;
      }
    },

    // Releasing the selected point
    (Move, _, _, true, false, _, _) =>
      *selected_point_id_option = None,

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
      *selected_point_id_option = Some(*hovered_point_id),

    // Unset the selected point if no other point is clicked on
    (Line, true, _, _, _, None, Some(_)) | (Mode::Line, _, _, _, true, None, Some(_)) =>
      *selected_point_id_option = None,

    // Select a point to draw the line to
    (Line, true, _, _, false, Some(hovered_point_id), Some(selected_point_id)) =>
    {
      graph.add_line(*selected_point_id, *hovered_point_id, *line_length);
      *selected_point_id_option = None;
    },

    // Deletes the selected line
    (Line, false, _, _, true, Some(hovered_point_id), Some(selected_point_id)) =>
    {
      graph.remove_line(*selected_point_id, *hovered_point_id);
      *selected_point_id_option = None;
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
  angle: &f32,
  base_point: &f32,
  arrow_head_length: &f32,
  hovered_point_id: &mut Option<usize>,
  selected_point_id: &Option<usize>,
  hexagons: &bool,
)
{
  // Paint lines
  paint_lines(graph, path_thickness, base_point, radius);
  paint_path(graph, path_thickness);
  paint_arrow_heads(graph, radius, angle, arrow_head_length, base_point);
  paint_line_lengths(graph);

  // Paint points
  paint_points(graph, radius, hovered_point_id, selected_point_id, hexagons);

  // TODO: consider replacing this with Option::inspect
  // Paints start label
  if let Some(start_id) = graph.start()
  {
    if let Some(start_point) = graph.get(start_id)
    { paint_label("Start", start_point.x, start_point.y, &radius); }
  }

  // TODO: consider replacing this with Option::inspect
  // Paints end label
  if let Some(end_id) = graph.end()
  {
    if let Some(end_point) = graph.get(end_id)
    { paint_label("End", end_point.x, end_point.y, &radius); }
  }
}

/// The `position` is the center of the point over which the label is painted.
fn paint_label(text: &str, x: f32, y: f32, radius: &f32)
{
  let text_center = get_text_center(text, None, 20, 1.0, 0.0);
  let text_dimensions = measure_text(text, None, 20, 1.0);

  // A 2 pixel gap between the label and the point is hard-coded
  draw_pill(
    x - text_dimensions.width.div(2.0),
    y - text_dimensions.height - radius - PADDING.mul(2) as f32 - 2.0,
    text_dimensions.width,
    text_dimensions.height + PADDING.mul(2) as f32,
    GREEN,
  );

  draw_text(
    text,
    x - text_center.x,
    y - text_center.y - radius - text_dimensions.height.div(2.0) - PADDING as f32 - 2.0,
    20.0,
    Color::from_hex(BG_COLOR)
  );
}

fn paint_path(graph: &DijkstraGraph, path_thiccness: &f32)
{
  let Some(path) = graph.get_path() else { return; };

  path.iter().zip(path.iter().skip(1))
    .map(|(from_id, to_id)| (graph.get(*from_id), graph.get(*to_id)))
    .filter(|(from_option, to_option)| from_option.is_some() && to_option.is_some())
    .map(|(from_option, to_option)| (from_option.as_ref().unwrap(), to_option.as_ref().unwrap()))
    .for_each(|(from, to)|
    {
      draw_line(
        from.x,
        from.y,
        to.x,
        to.y,
        *path_thiccness + 1.5,
        Color::from_hex(PATH_COLOR),
      );
    });
}

fn paint_points(
  graph: &DijkstraGraph,
  radius: &f32,
  hovered_point_id_option: &mut Option<usize>,
  selected_point_id_option: &Option<usize>,
  hexagons: &bool
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
      if *hexagons
      { draw_hexagon(point.x, point.y, *radius, 0., true, Color::from_hex(0xffffff), Color::from_hex(POINT_COLOR)); }
      else
      { draw_circle(point.x, point.y, *radius, if Some(id) == *selected_point_id_option { YELLOW } else { Color::from_hex(POINT_COLOR) }); }

      let text_center = get_text_center(id.to_string().as_str(), None, 20, 1.0, 0.0);

      // Drawing the point id
      draw_text(
        id.to_string().as_str(),
        point.x - text_center.x,
        point.y - text_center.y,
        20.0,
        Color::from_hex(BG_COLOR)
      );
    });

  // Drawing an outline around the hovered point
  if let Some(hovered_point_id) = hovered_point_id_option
  {
    if let Some(point) = graph.get(*hovered_point_id)
    {
      draw_circle_lines(
        point.x,
        point.y,
        *radius + 4_f32, 1_f32, MAGENTA
      );
    }
  }

  // Reset the hovered point id
  *hovered_point_id_option = None;
}

fn paint_arrow_heads(
  graph: &DijkstraGraph,
  radius: &f32,
  angle: &f32,
  arrow_head_length: &f32,
  base_point: &f32,
)
{
  graph.lines()
    .iter()
    .for_each(|(_, from, _, _, to)|
    {
      let mut direction = Vec2
      {
        x: from.x - to.x,
        y: from.y - to.y
      };
      let direction_length = direction.length();
      direction = direction.normalize();

      // Calculating the tip of the triangle that touches the node (position + (direction * (radius / length)))
      let arrow_head_location = Vec2
      {
        x: to.x + (direction.x * (radius/* + 2.*/)),
        y: to.y + (direction.y * (radius/* + 2.*/)),
      };

      // This point is at the base of the arrow head that "connects" it to the line
      let helper_point = Vec2
      {
        x: to.x + (direction.x * (radius + base_point)),
        y: to.y + (direction.y * (radius + base_point)),
      };

      /*
      draw_line(
        from.x + (direction.x * (-(self.radius) / direction.length())),
        from.y + (direction.y * (-(self.radius) / direction.length())),
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

      // Left arrow head wing
      draw_triangle(
        arrow_head_location,
        helper_point,
        Vec2
        {
          x: arrow_head_location.x + ((arrow_head_length / direction_length) * (((from.x - to.x) * angle.cos()) - ((from.y - to.y) * angle.sin()))),
          y: arrow_head_location.y + ((arrow_head_length / direction_length) * (((from.y - to.y) * angle.cos()) + ((from.x - to.x) * angle.sin()))),
        },
        Color::from_hex(LINE_COLOR)
      );

      // Right arrow head wing
      draw_triangle(
        arrow_head_location,
        helper_point,
        Vec2
        {
          x: arrow_head_location.x + ((arrow_head_length / direction_length) * (((from.x - to.x) * angle.cos()) + ((from.y - to.y) * angle.sin()))),
          y: arrow_head_location.y + ((arrow_head_length / direction_length) * (((from.y - to.y) * angle.cos()) - ((from.x - to.x) * angle.sin()))),
        },
        Color::from_hex(LINE_COLOR)
      );
    });
}

fn paint_lines(graph: &DijkstraGraph, path_thickness: &f32, base_point: &f32, radius: &f32)
{
  graph.lines()
    .iter()
    .for_each(|(_, from, _, _, to)|
    {
      let mut back_direction = Vec2
      {
        x: to.x - from.x,
        y: to.y - from.y
      };
      back_direction = back_direction.normalize();
      back_direction = back_direction.mul(*radius + *base_point);

      draw_line(
        from.x,
        from.y,
        to.x - back_direction.x,
        to.y - back_direction.y,
        *path_thickness,
        Color::from_hex(LINE_COLOR)
      );
    });
}

fn paint_line_lengths(graph: &DijkstraGraph)
{
  graph.lines()
    .iter()
    .for_each(|(_, from, distance, _, to)|
    {
      let position = Vec2
      {
        x: ((1.0 / 3.0) * from.x + (2.0 / 3.0) * to.x),
        y: ((1.0 / 3.0) * from.y + (2.0 / 3.0) * to.y),
      };

      let text_center = get_text_center(distance.to_string().as_str(), None, 20, 1.0, 0.0);
      let text_dimensions = measure_text(distance.to_string().as_str(), None, 20, 1.0);

      draw_pill(
        position.x - text_dimensions.width.div(2.0),
        position.y - text_dimensions.height.div(2.0) - PADDING as f32,
        text_dimensions.width,
        text_dimensions.height + PADDING.mul(2) as f32,
        Color::from_hex(LINE_LENGTH_COLOR)
      );

      draw_text(
        distance.to_string().as_str(),
        position.x - text_center.x,
        position.y - text_center.y,
        20.0,
        Color::from_hex(BG_COLOR)
      );
    });
}

// Tests
#[path = "./tests/utils_tests.rs"]
#[cfg(test)]
mod utils_tests;
