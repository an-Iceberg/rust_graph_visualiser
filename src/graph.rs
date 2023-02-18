use std::{collections::{HashMap, BTreeMap}, ops::{Mul, Div}};
use macroquad::{prelude::{Vec2, ORANGE, BLACK, mouse_position, MAGENTA, YELLOW, GREEN, Color}, shapes::{draw_circle, draw_circle_lines, draw_line, draw_triangle, draw_rectangle}, text::{get_text_center, draw_text, measure_text}};

use crate::{utils, draw_pill};

pub(crate) struct Graph
{
  pub(crate) start: Option<u8>,
  pub(crate) end: Option<u8>,

  /// This is the id of the point that the mouse is currently hovering over
  hovered_point_id: Option<u8>,

  /// This is the id of the point the mouse is currently hovering over and mouse 1 is pressed
  pub(crate) selected_point_id: Option<u8>,

  has_hovered_point_been_checked: bool,
  max_amount_of_points: u16,
  radius: u8,
  padding: u8,

  /// Key: point id, Value: point position
  points: BTreeMap<u8, Vec2>,

  /// Key: Line (2 ids), Value: line length
  lines: HashMap<Line, u16>,

  /// The path is a vector of all the point ids that the graph traverses; the 0th element is the start, the last element is the end
  path: Option<Vec<u8>>
}

// TODO: tests
impl Graph
{
  pub fn new() -> Graph
  {
    return Graph{
      start: None,
      end: None,
      hovered_point_id: None,
      selected_point_id: None,
      has_hovered_point_been_checked: false,
      max_amount_of_points: 100,
      radius: 13,
      padding: 3,
      points: BTreeMap::<u8, Vec2>::new(),
      lines: HashMap::<Line, u16>::new(),
      path: None
    };
  }

  pub fn points_amount(&self) -> usize
  {
    return self.points.len();
  }

  pub fn add_point(&mut self, coordinates: Vec2)
  {
    // Limiting the amount of available points to 100
    if self.points_amount() > self.max_amount_of_points as usize
    {
      return;
    }

    let mut smallest_missing_id = 1;

    // Finding the smallest missing point id
    for (point_id, _) in self.points.iter()
    {
      // Incrementing the missing id until it doesn't match a given point id
      if *point_id == smallest_missing_id
      {
        smallest_missing_id += 1;
        continue;
      }
    }

    self.points.insert(smallest_missing_id, coordinates);
  }

  pub fn remove_point(&mut self, id: u8)
  {
    // Deleting all lines associated with this point
    self.lines.retain(|line, _|
      {
        return line.from != id && line.to != id;
      }
    );

    self.points.remove(&id);
  }

  pub fn set_point_coordinates(&mut self, point_id: u8, new_position: Vec2)
  {
    match self.points.get_mut(&point_id)
    {
      Some(coordinates) =>
      {
        *coordinates = new_position;
      }

      None => ()
    }
  }

  /// Adds a line; if it already exists, the length gets updated
  pub fn add_line(&mut self, from_id: u8, to_id: u8, line_length: u16)
  {
    let new_line = Line { from: from_id, to: to_id };

    match self.lines.get_mut(&new_line)
    {
      Some(length) => *length = line_length,
      None => _ = self.lines.insert(new_line, line_length)
    }
  }

  pub fn remove_line(&mut self, from_id: u8, to_id: u8)
  {
    let line = Line { from: from_id, to: to_id };

    self.lines.remove(&line);
  }

  pub fn find_hovered_point(&mut self) -> Option<u8>
  {
    let mouse_position = mouse_position();

    self.has_hovered_point_been_checked = true;

    for (id, coordinates) in self.points.iter()
    {
      if utils::is_point_in_circle(mouse_position.0, mouse_position.1, coordinates.x, coordinates.y, self.radius as f32)
      {
        self.hovered_point_id = Some(*id);
        return Some(*id);
      }
    }

    return None;
  }

  pub fn get_hovered_point_id(&mut self) -> Option<u8>
  {
    if !self.has_hovered_point_been_checked
    {
      self.has_hovered_point_been_checked = true;
      return self.find_hovered_point();
    }

    return self.hovered_point_id;
  }

  pub fn get_radius(&self) -> u8
  {
    return self.radius;
  }

  pub fn clear(&mut self)
  {
    self.lines.clear();
    self.points.clear();
    self.start = None;
    self.end = None;
    self.hovered_point_id = None;
    self.selected_point_id = None;
    self.has_hovered_point_been_checked = false;
  }

  /// Finds the shortest path from the start to the end point using dijkstra's shortest path algorithm
  pub fn find_shortest_path(&mut self)
  {
    if self.start == None || self.end == None
    {
      return;
    }

    todo!();
  }

  pub fn paint_points(&mut self)
  {
    // Painting all points and centering the text
    for (id, coordinates) in self.points.iter()
    {
      draw_circle(coordinates.x, coordinates.y,self.radius as f32, if self.selected_point_id == Some(*id) { YELLOW } else { ORANGE });
      let text_center = get_text_center(id.to_string().as_str(), None, 20, 1.0, 0.0);
      draw_text(id.to_string().as_str(), coordinates.x - text_center.x, coordinates.y - text_center.y, 20.0, BLACK);
    }

    // Checking for the hovered point id (if it hasn't been done already)
    if !self.has_hovered_point_been_checked
    {
      self.find_hovered_point();
    }

    // Painting an outline for the hovered point (if it exists)
    if let Some(hovered_point_id) = self.hovered_point_id
    {
      if let Some(coordinates) = self.points.get(&hovered_point_id)
      {
        draw_circle_lines(
          coordinates.x,
          coordinates.y,
          (self.radius + 4) as f32,
          1 as f32,
          MAGENTA
        );
      }
    }

    // Reset the hovered point id
    self.hovered_point_id = None;
    self.has_hovered_point_been_checked = false;
  }

  pub fn paint_lines(&self)
  {
    for (line, _) in self.lines.iter()
    {
      match (self.points.get(&line.from), self.points.get(&line.to))
      {
        (Some(from_point), Some(to_point)) =>
        {
          let direction = Vec2 { x: from_point.x - to_point.x, y: from_point.y - to_point.y };

          let arrow_head_location = Vec2 {
            x: to_point.x + (direction.x * ((self.radius + 2) as f32 / direction.length())),
            y: to_point.y + (direction.y * ((self.radius + 2) as f32 / direction.length())),
          };

          // This point is at the base of the arrow head that "connects" it to the line
          let helper_point = Vec2 {
            x: to_point.x + (direction.x * ((self.radius + 15) as f32 / direction.length())),
            y: to_point.y + (direction.y * ((self.radius + 15) as f32 / direction.length())),
          };

          // The angle is in radians
          let angle: f32 = 0.436;
          let arrow_head_length = 20.0;

          // Calculating the tip of the triangle that touches the node (position + (direction * (radius / length)))
          draw_line(
            from_point.x + (direction.x * (-(self.radius as f32) / direction.length())),
            from_point.y + (direction.y * (-(self.radius as f32) / direction.length())),
            arrow_head_location.x,
            arrow_head_location.y,
            1.0,
            Color::from_rgba(0, 255, 255, 255)
          );

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
            Vec2 {
              x: arrow_head_location.x + ((arrow_head_length / direction.length()) * (((from_point.x - to_point.x) * angle.cos()) - ((from_point.y - to_point.y) * angle.sin()))),
              y: arrow_head_location.y + ((arrow_head_length / direction.length()) * (((from_point.y - to_point.y) * angle.cos()) + ((from_point.x - to_point.x) * angle.sin()))),
            },
            Color::from_rgba(0, 255, 255, 255)
          );

          // Right arrow head wing
          draw_triangle(
            arrow_head_location,
            helper_point,
            Vec2 {
              x: arrow_head_location.x + ((arrow_head_length / direction.length()) * (((from_point.x - to_point.x) * angle.cos()) + ((from_point.y - to_point.y) * angle.sin()))),
              y: arrow_head_location.y + ((arrow_head_length / direction.length()) * (((from_point.y - to_point.y) * angle.cos()) - ((from_point.x - to_point.x) * angle.sin()))),
            },
            Color::from_rgba(0, 255, 255, 255)
          );
        }

        (_, _) => ()
      }
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
          let position = Vec2 {
            x: (1.0/3.0)*from_point.x + (2.0/3.0)*to_point.x,
            y: (1.0/3.0)*from_point.y + (2.0/3.0)*to_point.y
          };

          let text_center = get_text_center(length.to_string().as_str(), None, 20, 1.0, 0.0);
          let text_dimensions = measure_text(length.to_string().as_str(), None, 20, 1.0);

          draw_pill(
            position.x - text_dimensions.width.div(2.0),
            position.y - text_dimensions.height.div(2.0) - self.padding as f32,
            text_dimensions.width,
            text_dimensions.height + self.padding.mul(2) as f32,
            Color::from_rgba(0, 255, 255, 255)
          );

          draw_text(
            length.to_string().as_str(),
            position.x - text_center.x,
            position.y - text_center.y,
            20.0,
            BLACK
          );
        }

        (_, _) => ()
      }
    }
  }

  pub fn paint_label(&self, text: &str, position: &Vec2)
  {
    let text_center = get_text_center(text, None, 20, 1.0, 0.0);

    draw_text(text, position.x - text_center.x, position.y - text_center.y - 23.0, 20.0, GREEN);
  }

  pub fn paint_graph(&mut self)
  {
    // Paints lines
    if !self.lines.is_empty()
    {
      self.paint_lines();
      self.paint_line_lengths();
    }

    // Paints points
    if !self.points.is_empty()
    {
      self.paint_points();
    }

    // Paints start label
    if let Some(start_id) = self.start
    {
      if let Some(start_point_position) = self.points.get(&start_id)
      {
        self.paint_label("Start", start_point_position);
      }
    }

    // Paints end label
    if let Some(end_id) = self.end
    {
      if let Some(end_point_position) = self.points.get(&end_id)
      {
        self.paint_label("End", end_point_position);
      }
    }
  }
}

#[derive(Hash)]
struct Line
{
  from: u8,
  to: u8,
}

impl PartialEq for Line
{
  fn eq(&self, other: &Self) -> bool
  {
    return self.from == other.from && self.to == other.to;
  }
}

impl Eq for Line {}
