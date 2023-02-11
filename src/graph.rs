use std::collections::{HashMap, BTreeMap};
use macroquad::{prelude::{Vec2, ORANGE, BLACK}, shapes::draw_circle, text::{get_text_center, draw_text}};

pub struct Graph
{
  start: Option<u8>,
  end: Option<u8>,
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
    return Graph{start: None, end: None, radius: 13, points: BTreeMap::<u8, Vec2>::new(), lines: HashMap::<Line, u16>::new()};
  }

  pub fn add_point(&mut self, coordinates: Vec2)
  {
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

  pub fn paint_points(&self)
  {
    for (point, coordinates) in self.points.iter()
    {
      draw_circle(coordinates.x, coordinates.y, self.radius as f32, ORANGE);
      let text_center = get_text_center(point.to_string().as_str(), None, 20, 1.0, 0.0);
      draw_text(point.to_string().as_str(), coordinates.x - text_center.x, coordinates.y - text_center.y, 20.0, BLACK);
    }
  }

  pub fn paint_lines(&self)
  {
    todo!();
  }

  pub fn paint_graph(&self)
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
