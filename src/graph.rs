use std::collections::{HashMap, BTreeMap};
use macroquad::{prelude::{Vec2, ORANGE, BLACK, mouse_position, MAGENTA}, shapes::{draw_circle, draw_circle_lines}, text::{get_text_center, draw_text}};

use crate::utils;

pub struct Graph
{
  start: Option<u8>,
  end: Option<u8>,
  hovered_point_id: Option<u8>,
  has_hovered_point_been_checked: bool,
  max_amount_of_points: u16,
  radius: u8,
  // Key: point id, Value: point position
  points: BTreeMap<u8, Vec2>,
  // Key: Line (2 ids), Value: line length
  lines: HashMap<Line, u16>
}

impl Graph
{
  pub fn new() -> Graph
  {
    return Graph{
      start: None,
      end: None,
      hovered_point_id: None,
      has_hovered_point_been_checked: false,
      max_amount_of_points: 100,
      radius: 13,
      points: BTreeMap::<u8, Vec2>::new(),
      lines: HashMap::<Line, u16>::new()
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

    // TODO: tests
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
    self.points.remove(&id);
  }

  pub fn add_line(&mut self, from_id: u8, to_id: u8)
  {
    todo!();
  }

  pub fn remove_line(&mut self, from_id: u8, to_id: u8)
  {
    todo!();
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

  pub fn clear(&mut self)
  {
    self.points.clear();
    self.lines.clear();
  }

  pub fn set_start(&mut self, id: u8)
  {
    todo!();
  }

  pub fn set_end(&mut self, id: u8)
  {
    todo!();
  }

  pub fn clear_start(&mut self)
  {
    self.start = None;
  }

  pub fn clear_end(&mut self)
  {
    self.end = None;
  }

  pub fn paint_points(&mut self)
  {
    // Painting all points and centering the text
    for (point, coordinates) in self.points.iter()
    {
      draw_circle(coordinates.x, coordinates.y, self.radius as f32, ORANGE);
      let text_center = get_text_center(point.to_string().as_str(), None, 20, 1.0, 0.0);
      draw_text(point.to_string().as_str(), coordinates.x - text_center.x, coordinates.y - text_center.y, 20.0, BLACK);
    }

    // Painting an outline for the hovered point (if it exists)
    if !self.has_hovered_point_been_checked
    {
      self.find_hovered_point();

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
    }

    // Reset the hovered point id
    self.hovered_point_id = None;
    self.has_hovered_point_been_checked = false;
  }

  pub fn paint_lines(&self)
  {
    todo!();
  }

  pub fn paint_graph(&mut self)
  {
    self.paint_points();
    //self.paint_lines();
  }
}

struct Line
{
  from: u8,
  to: u8,
}
