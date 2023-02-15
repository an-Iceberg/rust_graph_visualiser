use std::collections::{HashMap, BTreeMap};
use macroquad::{prelude::{Vec2, ORANGE, BLACK, mouse_position, MAGENTA, YELLOW}, shapes::{draw_circle, draw_circle_lines}, text::{get_text_center, draw_text}};

use crate::utils;

pub struct Graph
{
  start: Option<u8>,
  end: Option<u8>,

  // This is the id of the point that the mouse is currently hovering over
  hovered_point_id: Option<u8>,

  // This is the id of the point the mouse is currently hovering over and mouse 1 is pressed
  selected_point_id: Option<u8>,

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
      selected_point_id: None,
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

  pub fn get_hovered_point_id(&mut self) -> Option<u8>
  {
    if !self.has_hovered_point_been_checked
    {
      self.has_hovered_point_been_checked = true;
      return self.find_hovered_point();
    }

    return self.hovered_point_id;
  }

  pub fn set_selected_point_id(&mut self, id: Option<u8>)
  {
    self.selected_point_id = id;
  }

  pub fn get_selected_point_id(&self) -> Option<u8>
  {
    return self.selected_point_id;
  }

  pub fn get_radius(&self) -> u8
  {
    return self.radius;
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
    //self.selected_point_id = None;
    self.has_hovered_point_been_checked = false;
  }

  pub fn paint_lines(&self)
  {
    todo!();
  }

  pub fn paint_graph(&mut self)
  {
    //self.paint_lines();
    self.paint_points();
  }
}

struct Line
{
  from: u8,
  to: u8,
}
