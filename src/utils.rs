use crate::graph::DijkstraGraph;
use macroquad::{
  prelude::{
    is_mouse_button_down, is_mouse_button_pressed, is_mouse_button_released, mouse_position, Color, IVec2, MouseButton,
  },
  shapes::{draw_circle, draw_rectangle},
};
use std::ops::Div;

// TODO: rework everything

pub(crate) fn is_point_in_circle(point_x: f32, point_y: f32, circle_x: f32, circle_y: f32, circle_radius: f32) -> bool
{
  return (circle_x - point_x).powf(2_f32) + (circle_y - point_y).powf(2_f32) <= circle_radius.powf(2_f32);
}

pub(crate) fn is_point_in_rectangle(
  point_x: f32, point_y: f32, rectangle_x: f32, rectangle_y: f32, rectangle_width: f32, rectangle_height: f32,
) -> bool
{
  if point_x < rectangle_x
    || point_y < rectangle_y
    || point_x > (rectangle_x + rectangle_width)
    || point_y > (rectangle_y + rectangle_height)
  { return false; }

  return true;
}

pub(crate) fn draw_pill(x: f32, y: f32, width: f32, height: f32, color: Color) {
  draw_rectangle(x, y, width, height, color);
  draw_circle(x, y + height.div(2.0), height.div(2.0), color);
  draw_circle(x + width, y + height.div(2.0), height.div(2.0), color);
}

pub(crate) fn handle_mouse_input(mode: &Mode, graph: &mut DijkstraGraph) {
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

pub fn paint_path(&self)
{
  let Some(path) = &self.path else { return; };

  for (from, to) in path.iter().zip(path.iter().skip(1))
  {
    let Some(from_point) = self.points.get(from) else { continue; };
    let Some(to_point) = self.points.get(to) else { continue; };

    draw_line(
      from_point.position.x as f32,
      from_point.position.y as f32,
      to_point.position.x as f32,
      to_point.position.y as f32,
      self.path_thickness,
      Color::from_vec(Vec4::new(
        self.path_color[0],
        self.path_color[1],
        self.path_color[2],
        1.,
      )),
    );
  }
}

pub fn paint_points(&mut self)
{
  // Painting all points and centering the text
  for (id, node) in self.points.iter()
  {
    draw_circle(
        node.position.x as f32,
        node.position.y as f32,
        self.radius as f32,
        if self.selected_point_id == Some(*id)
        { YELLOW }
        else
        {
          Color::from_vec(Vec4::new(
            self.point_color[0],
            self.point_color[1],
            self.point_color[2],
            1.,
          ))
        },
    );

    let text_center = get_text_center(id.to_string().as_str(), None, 20, 1.0, 0.0);

    draw_text(
      id.to_string().as_str(),
      node.position.x as f32 - text_center.x,
      node.position.y as f32 - text_center.y,
      20.0,
      BLACK,
    );
  }

  // Checking for the hovered point id (if it hasn't been done already)
  if !self.has_hovered_point_been_checked { self.find_hovered_point(); }

  // TODO: consider replacing this with Option::inspect
  // Painting an outline for the hovered point (if it exists)
  if let Some(hovered_point_id) = self.hovered_point_id
  {
    if let Some(node) = self.points.get(&hovered_point_id)
    {
      draw_circle_lines(
        node.position.x as f32,
        node.position.y as f32,
        (self.radius + 4) as f32,
        1 as f32,
        MAGENTA,
      );
    }
  }

  // Reset the hovered point id
  self.hovered_point_id = None;
  self.has_hovered_point_been_checked = false;
}

pub fn paint_arrow_heads(&self) {
    for (line, _) in self.lines.iter() {
        match (self.points.get(&line.from), self.points.get(&line.to)) {
            (Some(from_point), Some(to_point)) => {
                let direction = Vec2 {
                    x: from_point.position.x - to_point.position.x,
                    y: from_point.position.y - to_point.position.y,
                };

                // Calculating the tip of the triangle that touches the node (position + (direction * (radius / length)))
                let arrow_head_location = Vec2 {
                    x: to_point.position.x
                        + (direction.x * ((self.radius + 2) / direction.as_vec2().length())),
                    y: to_point.position.y
                        + (direction.y * ((self.radius + 2) / direction.as_vec2().length())),
                };

                // This point is at the base of the arrow head that "connects" it to the line
                let helper_point = Vec2 {
                    x: to_point.position.x
                        + (direction.x
                            * ((self.radius + self.base_point) / direction.as_vec2().length())),
                    y: to_point.position.y
                        + (direction.y
                            * ((self.radius + self.base_point) / direction.as_vec2().length())),
                };

                /*
                draw_line(
                  from_point.position.x as f32 + (direction.x as f32 * (-(self.radius as f32) / direction.as_vec2().length())),
                  from_point.position.y as f32 + (direction.y as f32 * (-(self.radius as f32) / direction.as_vec2().length())),
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
                    arrow_head_location.as_vec2(),
                    helper_point.as_vec2(),
                    IVec2 {
                        x: arrow_head_location.x
                            + ((self.arrow_head_length / direction.as_vec2().length())
                                * (((from_point.position.x - to_point.position.x) as f32
                                    * self.angle.cos())
                                    - ((from_point.position.y - to_point.position.y) as f32
                                        * self.angle.sin())))
                                as i32,
                        y: arrow_head_location.y
                            + ((self.arrow_head_length / direction.as_vec2().length())
                                * (((from_point.position.y - to_point.position.y) as f32
                                    * self.angle.cos())
                                    + ((from_point.position.x - to_point.position.x) as f32
                                        * self.angle.sin())))
                                as i32,
                    }
                    .as_vec2(),
                    Color::from_vec(Vec4::new(
                        self.line_color[0],
                        self.line_color[1],
                        self.line_color[2],
                        1.,
                    )),
                );

                // Right arrow head wing
                draw_triangle(
                    arrow_head_location.as_vec2(),
                    helper_point.as_vec2(),
                    IVec2 {
                        x: arrow_head_location.x
                            + ((self.arrow_head_length / direction.as_vec2().length())
                                * (((from_point.position.x - to_point.position.x) as f32
                                    * self.angle.cos())
                                    + ((from_point.position.y - to_point.position.y) as f32
                                        * self.angle.sin())))
                                as i32,
                        y: arrow_head_location.y
                            + ((self.arrow_head_length / direction.as_vec2().length())
                                * (((from_point.position.y - to_point.position.y) as f32
                                    * self.angle.cos())
                                    - ((from_point.position.x - to_point.position.x) as f32
                                        * self.angle.sin())))
                                as i32,
                    }
                    .as_vec2(),
                    Color::from_vec(Vec4::new(
                        self.line_color[0],
                        self.line_color[1],
                        self.line_color[2],
                        1.,
                    )),
                );
            }

            (_, _) => (),
        }
    }
}

pub fn paint_lines(&self)
{
  for (line, _) in self.lines.iter()
  {
    let Some(from_point) = self.points.get(&line.from) else { continue; };
    let Some(to_point) = self.points.get(&line.to) else { continue; };

    draw_line(
      from_point.position.x as f32,
      from_point.position.y as f32,
      to_point.position.x as f32,
      to_point.position.y as f32,
      1.0,
      Color::from_vec(Vec4::new(
        self.line_color[0],
        self.line_color[1],
        self.line_color[2],
        1.0,
      )),
    );
  }
}

pub fn paint_line_lengths(&self)
{
  for (line, length) in self.lines.iter()
  {
    match (self.points.get(&line.from), self.points.get(&line.to))
    {
      (Some(from_point), Some(to_point)) =>
      {
        let position = IVec2 {
          x: ((1.0 / 3.0) * from_point.position.x as f32 + (2.0 / 3.0) * to_point.position.x as f32) as i32,
          y: ((1.0 / 3.0) * from_point.position.y as f32 + (2.0 / 3.0) * to_point.position.y as f32) as i32,
        };

        let text_center = get_text_center(length.to_string().as_str(), None, 20, 1.0, 0.0);
        let text_dimensions = measure_text(length.to_string().as_str(), None, 20, 1.0);

        utils::draw_pill(
          position.x as f32 - text_dimensions.width.div(2.0),
          position.y as f32 - text_dimensions.height.div(2.0) - self.padding as f32,
          text_dimensions.width,
          text_dimensions.height + self.padding.mul(2) as f32,
          Color::from_vec(Vec4::new(
            self.line_color[0],
            self.line_color[1],
            self.line_color[2],
            1.,
          )),
        );

        draw_text(
          length.to_string().as_str(),
          position.x as f32 - text_center.x,
          position.y as f32 - text_center.y,
          20.0,
          BLACK,
        );
      }
      (_, _) => (),
    }
  }
}

/// The `position` is the center of the point over which the label is painted.
pub fn paint_label(&self, text: &str, position: &IVec2)
{
  let text_center = get_text_center(text, None, 20, 1.0, 0.0);
  let text_dimensions = measure_text(text, None, 20, 1.0);

  // A 2 pixel gap between the label and the point is hard-coded
  utils::draw_pill(
    position.x as f32 - text_dimensions.width.div(2.0),
    position.y as f32 - text_dimensions.height - self.radius as f32 - self.padding.mul(2) as f32 - 2.0,
    text_dimensions.width,
    text_dimensions.height + self.padding.mul(2) as f32,
    GREEN,
  );

  draw_text(
    text,
    position.x as f32 - text_center.x,
    position.y as f32 - text_center.y as f32 - self.radius as f32 - text_dimensions.height.div(2.0) - self.padding as f32 - 2.0,
    20.0,
    Color::from_rgba(20, 0, 40, 255),
  );
}

pub fn paint_graph(&mut self)
{
  // Paints lines
  if !self.lines.is_empty()
  {
    self.paint_lines();
    self.paint_path();
    self.paint_arrow_heads();
    self.paint_line_lengths();
  }

  // Paints points
  if !self.points.is_empty()
  { self.paint_points(); }

  // TODO: consider replacing this with Option::inspect
  // Paints start label
  if let Some(start_id) = self.start
  {
    if let Some(start_point) = self.points.get(&start_id)
    { self.paint_label("Start", &start_point.position); }
  }

  // TODO: consider replacing this with Option::inspect
  // Paints end label
  if let Some(end_id) = self.end
  {
    if let Some(end_point) = self.points.get(&end_id)
    { self.paint_label("End", &end_point.position); }
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
#[path = "./tests/utils_tests.rs"]
#[cfg(test)]
mod utils_tests;
