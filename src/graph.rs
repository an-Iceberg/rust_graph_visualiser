use std::collections::{HashMap, BTreeMap};
use macroquad::prelude::Vec2;

pub struct Graph
{
  start: Option<u8>,
  end: Option<u8>,
  // Key: point id, Value: point position
  points: BTreeMap<u8, Vec2>,
  // Key: Line (2 ids), Value: line length
  lines: HashMap<Line, u16>
}

impl Graph
{
  pub fn new() -> Graph
  {
    return Graph{start: None, end: None, points: BTreeMap::<u8, Vec2>::new(), lines: HashMap::<Line, u16>::new()};
  }

  pub fn add_point(&mut self, coordinates: Vec2)
  {
    let mut smallest_missing_id = 0;

    // TODO: tests
    // Finding the smallest missing point id
    for (point_id, _) in self.points.iter()
    {
      // The next point id should be larger by 1 from the current id
      if *point_id == smallest_missing_id + 1
      {
        smallest_missing_id = *point_id;
      }
      // The next point's id is a larger step away than 1, therefore a point id is missing here
      else
      {
        smallest_missing_id += 1;
        break;
      }
    }

    self.points.insert(smallest_missing_id, coordinates);
  }

  pub fn remove_point(&mut self, id: u8)
  {
    self.points.remove(&id);
  }

  pub fn add_line(&mut self)
  {
    todo!();
  }

  pub fn remove_line(&mut self)
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
}

// Naive implementation of a line
struct Line
{
  from: u8,
  to: u8,
}
