use std::collections::{HashMap, BTreeMap};
use macroquad::prelude::Vec2;

pub struct Graph
{
  points: BTreeMap<u8, Vec2>,
  // TODO: figure out a good data structure for a line. It needs to store 'from' id, 'to' id and 'length'
  // IDEA: hashset, key is a point id, value is all other point ids it points to (but the line lengths would need to be added somehow)
  lines: HashMap<Line, u16>
}

impl Graph
{
  pub fn new() -> Graph
  {
    return Graph{points: BTreeMap::<u8, Vec2>::new(), lines: HashMap::<Line, u16>::new()};
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
}

// Naive implementation of a line
struct Line
{
  from: u8,
  to: u8,
}
