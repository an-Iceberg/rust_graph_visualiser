use std::{collections::{HashMap, BTreeMap}, ops::{Mul, Div}, fmt::Display};
use macroquad::{prelude::{Vec2, ORANGE, BLACK, mouse_position, MAGENTA, YELLOW, GREEN, Color}, shapes::{draw_circle, draw_circle_lines, draw_line, draw_triangle, draw_rectangle}, text::{get_text_center, draw_text, measure_text}};

use crate::{utils, draw_pill};

/// ### Graph
///
/// It contains a lot
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

  /// Contains all data for the points
  ///
  /// Key: point id
  ///
  /// Value: point position
  points: BTreeMap<u8, Vec2>,

  /// Contains all data for the lines
  ///
  /// Key: Line (2 ids)
  ///
  ///  Value: line length
  lines: HashMap<Line, u16>,

  /// The path is a vector of all the point ids that the graph traverses
  ///
  /// The 0th element is the start, the last element is the end
  path: Option<Vec<u8>>
}

impl Default for Graph
{
  fn default() -> Self
  {
    return Graph {
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
}

impl Graph
{
  pub fn new() -> Graph
  {
    return Graph {
      ..Graph::default()
    };
  }

  pub fn points_amount(&self) -> usize
  {
    return self.points.len();
  }

  pub fn add_point(&mut self, coordinates: Vec2)
  {
    // Limiting the amount of available points to 100
    if self.points_amount() >= self.max_amount_of_points as usize
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
    if self.start.is_none() || self.end.is_none()
    {
      return;
    }

    // TODO: implement Dijkstra's shortest path algorithm

    todo!();
  }

  // TODO: somehow paint the path

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

    // TODO: consider replacing this with Option::inspect
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

    // TODO: draw pill of contrasting colour around the text
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

    // TODO: consider replacing this with Option::inspect
    // Paints start label
    if let Some(start_id) = self.start
    {
      if let Some(start_point_position) = self.points.get(&start_id)
      {
        self.paint_label("Start", start_point_position);
      }
    }

    // TODO: consider replacing this with Option::inspect
    // Paints end label
    if let Some(end_id) = self.end
    {
      if let Some(end_point_position) = self.points.get(&end_id)
      {
        self.paint_label("End", end_point_position);
      }
    }
  }

  pub fn print_graph_data(&self)
  {
    println!("Points:");
    self.points
      .iter()
      .for_each(|point|
      {
        println!("{}: {}", point.0, point.1);
      });

    println!("Lines:");
    self.lines
      .iter()
      .for_each(|line|
      {
        println!("{}: {}", line.0, line.1);
      });

    match self.start
    {
      Some(id) => println!("Start: {}", id),
      None => println!("Start: None")
    }

    match self.end
    {
      Some(id) => println!("End: {}", id),
      None => println!("End: None")
    }
  }

  /// Replaces the current graph with a small one
  pub fn insert_small_graph(&mut self)
  {
    self.clear();

    self.points = BTreeMap::from([
      (1, Vec2 { x: 942.0, y: 355.0 }),
      (2, Vec2 { x: 720.0, y: 208.0 }),
      (3, Vec2 { x: 198.0, y: 342.0 }),
      (4, Vec2 { x: 463.0, y: 507.0 }),
      (5, Vec2 { x: 735.0, y: 513.0 }),
      (6, Vec2 { x: 458.0, y: 346.0 }),
      (7, Vec2 { x: 468.0, y: 202.0 }),
      (8, Vec2 { x: 721.0, y: 360.0 }),
    ]);

    self.lines = HashMap::<Line, u16>::from([
      (Line { from: 4, to: 5 }, 3),
      (Line { from: 3, to: 6 }, 5),
      (Line { from: 6, to: 8 }, 4),
      (Line { from: 7, to: 2 }, 5),
      (Line { from: 2, to: 1 }, 5),
      (Line { from: 6, to: 2 }, 7),
      (Line { from: 4, to: 8 }, 5),
      (Line { from: 8, to: 1 }, 4),
      (Line { from: 3, to: 7 }, 4),
      (Line { from: 3, to: 4 }, 7),
      (Line { from: 7, to: 8 }, 6),
      (Line { from: 6, to: 5 }, 8),
      (Line { from: 5, to: 1 }, 3),
    ]);
  }

  /// Replaces the current graph with a medium-sized one
  pub fn insert_medium_graph(&mut self)
  {
    self.clear();

    self.points = BTreeMap::<u8, Vec2>::from([
      (1, Vec2 { x: 959.0, y: 211.0 }),
      (2, Vec2 { x: 967.0, y: 394.0 }),
      (3, Vec2 { x: 946.0, y: 532.0 }),
      (4, Vec2 { x: 144.0, y: 377.0 }),
      (5, Vec2 { x: 775.0, y: 295.0 }),
      (6, Vec2 { x: 734.0, y: 523.0 }),
      (7, Vec2 { x: 559.0, y: 493.0 }),
      (8, Vec2 { x: 570.0, y: 361.0 }),
      (9, Vec2 { x: 569.0, y: 200.0 }),
      (10, Vec2 { x: 353.0, y: 206.0 }),
      (11, Vec2 { x: 355.0, y: 350.0 }),
      (12, Vec2 { x: 342.0, y: 488.0 }),
    ]);

    self.lines = HashMap::<Line, u16>::from([
      (Line { from: 11, to: 7 }, 4),
      (Line { from: 8, to: 2 }, 5),
      (Line { from: 4, to: 10 }, 4),
      (Line { from: 12, to: 7 }, 4),
      (Line { from: 4, to: 12 }, 6),
      (Line { from: 8, to: 6 }, 4),
      (Line { from: 6, to: 3 }, 20),
      (Line { from: 8, to: 5 }, 3),
      (Line { from: 12, to: 8 }, 2),
      (Line { from: 9, to: 5 }, 3),
      (Line { from: 11, to: 8 }, 3),
      (Line { from: 4, to: 11 }, 5),
      (Line { from: 5, to: 1 }, 1),
      (Line { from: 9, to: 1 }, 5),
      (Line { from: 10, to: 9 }, 4),
      (Line { from: 7, to: 6 }, 7),
      (Line { from: 5, to: 2 }, 2),
    ]);
  }

  pub fn insert_large_graph(&mut self)
  {
    self.clear();

    self.points = BTreeMap::<u8, Vec2>::from([
      (1, Vec2 { x: 595.0, y: 640.0 }),
      (2, Vec2 { x: 864.0, y: 300.0 }),
      (3, Vec2 { x: 550.0, y: 369.0 }),
      (4, Vec2 { x: 280.0, y: 606.0 }),
      (5, Vec2 { x: 748.0, y: 127.0 }),
      (6, Vec2 { x: 177.0, y: 71.0 }),
      (7, Vec2 { x: 467.0, y: 84.0 }),
      (8, Vec2 { x: 260.0, y: 431.0 }),
      (9, Vec2 { x: 928.0, y: 642.0 }),
      (10, Vec2 { x: 466.0, y: 181.0 }),
      (11, Vec2 { x: 433.0, y: 27.0 }),
      (12, Vec2 { x: 667.0, y: 52.0 }),
      (13, Vec2 { x: 847.0, y: 75.0 }),
      (14, Vec2 { x: 734.0, y: 270.0 }),
      (15, Vec2 { x: 931.0, y: 233.0 }),
      (16, Vec2 { x: 904.0, y: 389.0 }),
      (17, Vec2 { x: 423.0, y: 467.0 }),
      (18, Vec2 { x: 445.0, y: 551.0 }),
      (19, Vec2 { x: 691.0, y: 559.0 }),
    ]);

    self.lines = HashMap::<Line, u16>::from([
      (Line { from: 12, to: 13 }, 1),
      (Line { from: 6, to: 8 }, 12),
      (Line { from: 14, to: 3 }, 1),
      (Line { from: 16, to: 9 }, 10),
      (Line { from: 15, to: 9 }, 14),
      (Line { from: 2, to: 19 }, 9),
      (Line { from: 18, to: 19 }, 3),
      (Line { from: 17, to: 18 }, 2),
      (Line { from: 8, to: 4 }, 1),
      (Line { from: 1, to: 9 }, 1),
      (Line { from: 7, to: 5 }, 1),
      (Line { from: 16, to: 3 }, 2),
      (Line { from: 3, to: 8 }, 1),
      (Line { from: 3, to: 17 }, 3),
      (Line { from: 15, to: 16 }, 1),
      (Line { from: 5, to: 14 }, 3),
      (Line { from: 10, to: 3 }, 8),
      (Line { from: 13, to: 2 }, 2),
      (Line { from: 12, to: 5 }, 2),
      (Line { from: 11, to: 12 }, 1),
      (Line { from: 6, to: 11 }, 2),
      (Line { from: 10, to: 5 }, 3),
      (Line { from: 5, to: 2 }, 1),
      (Line { from: 16, to: 17 }, 5),
      (Line { from: 6, to: 7 }, 1),
      (Line { from: 18, to: 1 }, 1),
      (Line { from: 6, to: 10 }, 2),
      (Line { from: 2, to: 3 }, 1),
      (Line { from: 19, to: 9 }, 4),
      (Line { from: 17, to: 4 }, 2),
      (Line { from: 13, to: 15 }, 1),
      (Line { from: 4, to: 1 }, 1),
    ]);
  }
}

/// ### The line struct
///
/// It contains two ids: one is the source and the other is the target of the line
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

impl Display for Line
{
  fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
  {
    return write!(formatter, "({}, {})", self.from, self.to);
  }
}

// Tests

#[cfg(test)]
mod tests
{
  use std::{ops::Mul, collections::{BTreeMap, HashMap}};
  use macroquad::prelude::Vec2;
  use rand::*;
  use crate::graph::Line;
  use super::Graph;

  fn vec2_random_coordinates(radius: f32) -> Vec2
  {
    return Vec2 {
      x: thread_rng().gen_range(radius..(1290.0 - 200.0 - radius)),
      y: thread_rng().gen_range(radius..(720.0 - radius))
    };
  }

  fn graph(amount_of_points: u8) -> Graph
  {
    let mut graph = Graph::new();
    for _i in 1..=amount_of_points
    {
      graph.add_point(vec2_random_coordinates(graph.radius as f32));
    }

    return graph;
  }

  #[test]
  fn add_some_points()
  {
    // Creating a graph
    let mut is_graph = Graph::new();
    for _i in 1..=3
    {
      is_graph.add_point(vec2_random_coordinates(is_graph.radius as f32));
    }

    // Creating the values it should have
    let mut should_ids: Vec<u8> = Vec::new();
    for id in 1..=3
    {
      should_ids.push(id);
    }

    // Comparing the two for equality
    for (is_id, should_id) in is_graph.points.keys().zip(should_ids.iter())
    {
      assert_eq!(*is_id, *should_id);
    }
  }

  #[test]
  fn add_many_points()
  {
    // Creating the graph
    let mut is_graph = Graph::new();
    for _i in 1..=50
    {
      is_graph.add_point(vec2_random_coordinates(is_graph.get_radius() as f32))
    }

    // Creating the data that should be in the graph
    let mut should_ids: Vec<u8> = Vec::new();
    for id in 1..=50
    {
      should_ids.push(id);
    }

    // Comparing for equality
    for (is_id, should_id) in is_graph.points.keys().zip(should_ids.iter())
    {
      assert_eq!(*is_id, *should_id);
    }
  }

  #[test]
  fn max_amount_of_points()
  {
    // Creating graph and "adding" 1_000 points to it
    let mut is_graph = Graph::new();
    for _i in 0..1_000
    {
      is_graph.add_point(vec2_random_coordinates(is_graph.radius as f32));
    }

    // The graph should still only have 100 points
    assert_eq!(is_graph.points.len(), 100);
  }

  #[test]
  fn remove_points()
  {
    // Creating a graph
    let mut is_graph = graph(10);

    // Removing every second point
    for id in 1..=5
    {
      is_graph.remove_point(id * 2);
    }

    // Creating the ids the resulting graph should have
    let mut should_ids: Vec<u8> = Vec::new();
    for id in 1..=5
    {
      should_ids.push(id.mul(2 as u8) - 1);
    }

    // Comparing for equality
    for (is_id, should_id) in is_graph.points.keys().zip(should_ids.iter())
    {
      assert_eq!(*is_id, *should_id);
    }
  }

  #[test]
  fn shortest_path_small()
  {
    // First case
    {
      let mut graph = Graph {
        points: BTreeMap::<u8, Vec2>::from([
          (1, Vec2 { x: 942.0, y: 355.0 }),
          (2, Vec2 { x: 720.0, y: 208.0 }),
          (3, Vec2 { x: 198.0, y: 342.0 }),
          (4, Vec2 { x: 463.0, y: 507.0 }),
          (5, Vec2 { x: 735.0, y: 513.0 }),
          (6, Vec2 { x: 458.0, y: 346.0 }),
          (7, Vec2 { x: 468.0, y: 202.0 }),
          (8, Vec2 { x: 721.0, y: 360.0 }),
        ]),
        lines: HashMap::<Line, u16>::from([
          (Line { from: 4, to: 5 }, 3),
          (Line { from: 3, to: 6 }, 5),
          (Line { from: 6, to: 8 }, 4),
          (Line { from: 7, to: 2 }, 5),
          (Line { from: 2, to: 1 }, 5),
          (Line { from: 6, to: 2 }, 7),
          (Line { from: 4, to: 8 }, 5),
          (Line { from: 8, to: 1 }, 4),
          (Line { from: 3, to: 7 }, 4),
          (Line { from: 3, to: 4 }, 7),
          (Line { from: 7, to: 8 }, 6),
          (Line { from: 6, to: 5 }, 8),
          (Line { from: 5, to: 1 }, 3),
        ]),
        start: Some(3),
        end: Some(1),
        ..Graph::default()
      };

      // Shortest paths are either [3, 4, 5, 1] or [3, 6, 8, 1]
      let should_path_1: Vec<u8> = vec![3, 4, 5, 1];
      let should_path_2: Vec<u8> = vec![3, 6, 8, 1];

      graph.find_shortest_path();

      match graph.path
      {
        Some(path) =>
        {
          path
          .iter()
          .zip(should_path_1.iter())
          .zip(should_path_2.iter())
          .for_each(|((path_id, should_id_1), should_id_2)|
          {
            assert!(*path_id == *should_id_1 || *path_id == *should_id_2);
          });
        }
        None => panic!("A path should have been found")
      }
    }

    // Second case
    {
      let mut graph = Graph {
        points: BTreeMap::<u8, Vec2>::from([
          (1, Vec2 { x: 783.0, y: 102.0 }),
          (2, Vec2 { x: 412.0, y: 295.0 }),
          (3, Vec2 { x: 680.0, y: 308.0 }),
          (4, Vec2 { x: 509.0, y: 459.0 }),
          (5, Vec2 { x: 330.0, y: 603.0 }),
          (6, Vec2 { x: 160.0, y: 442.0 }),
          (7, Vec2 { x: 174.0, y: 196.0 }),
          (8, Vec2 { x: 411.0, y: 78.0 }),
          (9, Vec2 { x: 1003.0, y: 239.0 }),
        ]),
        lines: HashMap::<Line, u16>::from([
          (Line { from: 4, to: 5 }, 2),
          (Line { from: 3, to: 4 }, 3),
          (Line { from: 2, to: 6 }, 3),
          (Line { from: 1, to: 9 }, 7),
          (Line { from: 4, to: 2 }, 1),
          (Line { from: 9, to: 3 }, 1),
          (Line { from: 6, to: 2 }, 3),
          (Line { from: 7, to: 8 }, 2),
          (Line { from: 2, to: 4 }, 1),
          (Line { from: 2, to: 8 }, 3),
          (Line { from: 2, to: 7 }, 5),
          (Line { from: 2, to: 1 }, 1),
          (Line { from: 5, to: 6 }, 2),
          (Line { from: 1, to: 2 }, 1),
          (Line { from: 3, to: 9 }, 1),
          (Line { from: 4, to: 3 }, 3),
          (Line { from: 1, to: 8 }, 1),
          (Line { from: 8, to: 1 }, 1),
          (Line { from: 6, to: 7 }, 2),
          (Line { from: 8, to: 7 }, 2),
          (Line { from: 8, to: 2 }, 3),
          (Line { from: 2, to: 3 }, 1),
          (Line { from: 7, to: 2 }, 5),
          (Line { from: 9, to: 1 }, 7),
          (Line { from: 3, to: 2 }, 1),
          (Line { from: 5, to: 4 }, 2),
          (Line { from: 6, to: 5 }, 2),
          (Line { from: 7, to: 6 }, 2),
        ]),
        start: Some(7),
        end: Some(9),
        ..Graph::default()
      };

      let should_path = vec![7, 8, 1, 2, 3, 9];

      graph.find_shortest_path();

      match graph.path
      {
        Some(path) =>
        {
          path
          .iter()
          .zip(should_path.iter())
          .for_each(|(path_id, should_id)|
          {
            assert_eq!(*path_id, *should_id);
          });
        }
        None => panic!("A path should have been found")
      }
    }
  }

  #[test]
  fn shortest_path_medium()
  {
    let mut graph = Graph {
      points: BTreeMap::<u8, Vec2>::from([
        (1, Vec2 { x: 959.0, y: 211.0 }),
        (2, Vec2 { x: 967.0, y: 394.0 }),
        (3, Vec2 { x: 946.0, y: 532.0 }),
        (4, Vec2 { x: 144.0, y: 377.0 }),
        (5, Vec2 { x: 775.0, y: 295.0 }),
        (6, Vec2 { x: 734.0, y: 523.0 }),
        (7, Vec2 { x: 559.0, y: 493.0 }),
        (8, Vec2 { x: 570.0, y: 361.0 }),
        (9, Vec2 { x: 569.0, y: 200.0 }),
        (10, Vec2 { x: 353.0, y: 206.0 }),
        (11, Vec2 { x: 355.0, y: 350.0 }),
        (12, Vec2 { x: 342.0, y: 488.0 }),
      ]),
      lines: HashMap::<Line, u16>::from([
        (Line { from: 11, to: 7 }, 4),
        (Line { from: 8, to: 2 }, 5),
        (Line { from: 4, to: 10 }, 4),
        (Line { from: 12, to: 7 }, 4),
        (Line { from: 4, to: 12 }, 6),
        (Line { from: 8, to: 6 }, 4),
        (Line { from: 6, to: 3 }, 20),
        (Line { from: 8, to: 5 }, 3),
        (Line { from: 12, to: 8 }, 2),
        (Line { from: 9, to: 5 }, 3),
        (Line { from: 11, to: 8 }, 3),
        (Line { from: 4, to: 11 }, 5),
        (Line { from: 5, to: 1 }, 1),
        (Line { from: 9, to: 1 }, 5),
        (Line { from: 10, to: 9 }, 4),
        (Line { from: 7, to: 6 }, 7),
        (Line { from: 5, to: 2 }, 2),
      ]),
      start: Some(4),
      end: None,
      ..Graph::default()
    };

    // First end
    {
      let should_path_1: Vec<u8> = vec![4, 10, 9, 5, 1];
      let should_path_2: Vec<u8> = vec![4, 11, 8, 5, 1];
      let should_path_3: Vec<u8> = vec![4, 12, 9, 5, 1];

      graph.end = Some(1);
      graph.find_shortest_path();

      match graph.path
      {
        Some(ref path) =>
        {
          path
          .iter()
          .zip(should_path_1.iter())
          .zip(should_path_2.iter())
          .zip(should_path_3.iter())
          .for_each(|(((path_id, should_id_1), should_id_2), should_id_3)|
          {
            assert!(*path_id == *should_id_1 || *path_id == *should_id_2 || *path_id == *should_id_3);
          });
        }
        None => panic!("A path should have been found")
      }
    }

    // Second end
    {
      let should_path_1: Vec<u8> = vec![4, 10, 9, 5, 2];
      let should_path_2: Vec<u8> = vec![4, 11, 8, 5, 2];
      let should_path_3: Vec<u8> = vec![4, 11, 8, 2];
      let should_path_4: Vec<u8> = vec![4, 12, 8, 5, 2];
      let should_path_5: Vec<u8> = vec![4, 12, 8, 2];

      graph.end = Some(2);
      graph.find_shortest_path();

      match graph.path
      {
        Some(ref path) =>
        {
          path
          .iter()
          .zip(should_path_1.iter())
          .zip(should_path_2.iter())
          .zip(should_path_3.iter())
          .zip(should_path_4.iter())
          .zip(should_path_5.iter())
          .for_each(|(((((path_id, should_id_1), should_id_2), should_id_3), should_id_4), should_id_5)|
          {
            assert!(*path_id == *should_id_1 || *path_id == *should_id_2 || *path_id == *should_id_3 || *path_id == *should_id_4 || *path_id == *should_id_5);
          });
        }
        None => panic!("A path should have been found")
      }
    }

    // Third end
    {
      let should_path_1: Vec<u8> = vec![4, 11, 8, 6, 3];
      let should_path_2: Vec<u8> = vec![4, 12, 8, 6, 3];

      graph.end = Some(3);
      graph.find_shortest_path();

      match graph.path
      {
        Some(ref path) =>
        {
          path
          .iter()
          .zip(should_path_1.iter())
          .zip(should_path_2.iter())
          .for_each(|((path_id, should_id_1), should_id_2)|
          {
            assert!(*path_id == *should_id_1 || *path_id == *should_id_2);
          });
        }
        None => panic!("A path should have been found")
      }
    }
  }

  #[test]
  fn shortest_path_large()
  {
    let mut graph = Graph {
      points: BTreeMap::<u8, Vec2>::from([
        (1, Vec2 { x: 595.0, y: 640.0 }),
        (2, Vec2 { x: 864.0, y: 300.0 }),
        (3, Vec2 { x: 550.0, y: 369.0 }),
        (4, Vec2 { x: 280.0, y: 606.0 }),
        (5, Vec2 { x: 748.0, y: 127.0 }),
        (6, Vec2 { x: 177.0, y: 71.0 }),
        (7, Vec2 { x: 467.0, y: 84.0 }),
        (8, Vec2 { x: 260.0, y: 431.0 }),
        (9, Vec2 { x: 928.0, y: 642.0 }),
        (10, Vec2 { x: 466.0, y: 181.0 }),
        (11, Vec2 { x: 433.0, y: 27.0 }),
        (12, Vec2 { x: 667.0, y: 52.0 }),
        (13, Vec2 { x: 847.0, y: 75.0 }),
        (14, Vec2 { x: 734.0, y: 270.0 }),
        (15, Vec2 { x: 931.0, y: 233.0 }),
        (16, Vec2 { x: 904.0, y: 389.0 }),
        (17, Vec2 { x: 423.0, y: 467.0 }),
        (18, Vec2 { x: 445.0, y: 551.0 }),
        (19, Vec2 { x: 691.0, y: 559.0 }),
      ]),
      lines: HashMap::<Line, u16>::from([
        (Line { from: 12, to: 13 }, 1),
        (Line { from: 6, to: 8 }, 12),
        (Line { from: 14, to: 3 }, 1),
        (Line { from: 16, to: 9 }, 10),
        (Line { from: 15, to: 9 }, 14),
        (Line { from: 2, to: 19 }, 9),
        (Line { from: 18, to: 19 }, 3),
        (Line { from: 17, to: 18 }, 2),
        (Line { from: 8, to: 4 }, 1),
        (Line { from: 1, to: 9 }, 1),
        (Line { from: 7, to: 5 }, 1),
        (Line { from: 16, to: 3 }, 2),
        (Line { from: 3, to: 8 }, 1),
        (Line { from: 3, to: 17 }, 3),
        (Line { from: 15, to: 16 }, 1),
        (Line { from: 5, to: 14 }, 3),
        (Line { from: 10, to: 3 }, 8),
        (Line { from: 13, to: 2 }, 2),
        (Line { from: 12, to: 5 }, 2),
        (Line { from: 11, to: 12 }, 1),
        (Line { from: 6, to: 11 }, 2),
        (Line { from: 10, to: 5 }, 3),
        (Line { from: 5, to: 2 }, 1),
        (Line { from: 16, to: 17 }, 5),
        (Line { from: 6, to: 7 }, 1),
        (Line { from: 18, to: 1 }, 1),
        (Line { from: 6, to: 10 }, 2),
        (Line { from: 2, to: 3 }, 1),
        (Line { from: 19, to: 9 }, 4),
        (Line { from: 17, to: 4 }, 2),
        (Line { from: 13, to: 15 }, 1),
        (Line { from: 4, to: 1 }, 1),
      ]),
      start: Some(6),
      end: Some(9),
      ..Graph::default()
    };

    let should_path = vec![6, 7, 5, 2, 3, 8, 4, 1, 9];

    graph.find_shortest_path();

    match graph.path
    {
      Some(path) =>
      {
        path
        .iter()
        .zip(should_path.iter())
        .for_each(|(path_id, should_id)|
        {
          assert_eq!(*path_id, *should_id);
        });
      }
      None => panic!("A path should have been found")
    }
  }

  #[test]
  fn start_and_end_are_within_graph()
  {
    let mut graph = Graph {
      points: BTreeMap::<u8, Vec2>::from([
        (1, Vec2 { x: 970.0, y: 108.0 }),
        (2, Vec2 { x: 991.0, y: 340.0 }),
        (3, Vec2 { x: 1023.0, y: 580.0 }),
        (4, Vec2 { x: 509.0, y: 459.0 }),
        (5, Vec2 { x: 750.0, y: 537.0 }),
        (6, Vec2 { x: 747.0, y: 262.0 }),
        (7, Vec2 { x: 535.0, y: 237.0 }),
        (8, Vec2 { x: 497.0, y: 433.0 }),
        (9, Vec2 { x: 352.0, y: 379.0 }),
        (10, Vec2 { x: 308.0, y: 266.0 }),
        (16, Vec2 { x: 163.0, y: 205.0 }),
        (17, Vec2 { x: 149.0, y: 346.0 }),
        (18, Vec2 { x: 620.0, y: 550.0 }),
      ]),
      lines: HashMap::<Line, u16>::from([
        (Line { from: 5, to: 4 }, 2),
        (Line { from: 18, to: 5 }, 7),
        (Line { from: 6, to: 1 }, 6),
        (Line { from: 8, to: 18 }, 6),
        (Line { from: 9, to: 8 }, 8),
        (Line { from: 4, to: 2 }, 5),
        (Line { from: 6, to: 4 }, 9),
        (Line { from: 4, to: 3 }, 4),
        (Line { from: 17, to: 10 }, 8),
        (Line { from: 10, to: 7 }, 12),
        (Line { from: 16, to: 10 }, 7),
        (Line { from: 8, to: 6 }, 4),
        (Line { from: 10, to: 9 }, 11),
        (Line { from: 17, to: 9 }, 4),
        (Line { from: 7, to: 6 }, 5),
      ]),
      start: Some(10),
      end: Some(4),
      ..Graph::default()
    };

    let should_path = vec![10, 7, 6, 4];

    graph.find_shortest_path();

    match graph.path
    {
      Some(path) =>
      {
        path
        .iter()
        .zip(should_path.iter())
        .for_each(|(path_id, should_id)|
        {
          assert_eq!(*path_id, *should_id);
        });
      }
      None => panic!("A path should have been found")
    }
  }

  #[test]
  fn no_possible_path()
  {
    let mut graph = Graph::new();
    graph.insert_small_graph();
    graph.start = Some(1);
    graph.end = Some(3);

    graph.find_shortest_path();

    assert!(graph.path.is_none());
  }

  #[test]
  fn disconnected_graph()
  {
    let mut graph = Graph {
      points: BTreeMap::<u8, Vec2>::from([
        (1, Vec2 { x: 888.0, y: 135.0 }),
        (2, Vec2 { x: 595.0, y: 138.0 }),
        (3, Vec2 { x: 267.0, y: 120.0 }),
        (4, Vec2 { x: 230.0, y: 347.0 }),
        (5, Vec2 { x: 553.0, y: 379.0 }),
        (6, Vec2 { x: 905.0, y: 390.0 }),
        (7, Vec2 { x: 895.0, y: 649.0 }),
        (8, Vec2 { x: 479.0, y: 634.0 }),
        (8, Vec2 { x: 187.0, y: 607.0 }),
      ]),
      lines: HashMap::<Line, u16>::from([
        (Line { from: 9, to: 8 }, 20),
        (Line { from: 3, to: 2 }, 20),
        (Line { from: 1, to: 6 }, 20),
        (Line { from: 6, to: 7 }, 20),
        (Line { from: 3, to: 4 }, 20),
        (Line { from: 8, to: 7 }, 20),
        (Line { from: 3, to: 5 }, 20),
      ]),
      start: Some(3),
      end: Some(1),
      ..Graph::default()
    };
    graph.start = Some(3);
    graph.end = Some(7);
    graph.find_shortest_path();

    assert!(graph.path.is_none());
  }

  #[test]
  fn cyclical_valid_path()
  {
    let mut graph = Graph {
      points: BTreeMap::<u8, Vec2>::from([
        (1, Vec2 { x: 899.0, y: 490.0 }),
        (2, Vec2 { x: 941.0, y: 618.0 }),
        (3, Vec2 { x: 710.0, y: 621.0 }),
        (4, Vec2 { x: 777.0, y: 390.0 }),
        (5, Vec2 { x: 698.0, y: 200.0 }),
        (6, Vec2 { x: 497.0, y: 185.0 }),
        (7, Vec2 { x: 379.0, y: 367.0 }),
        (8, Vec2 { x: 556.0, y: 541.0 }),
        (9, Vec2 { x: 403.0, y: 574.0 }),
        (10, Vec2 { x: 207.0, y: 434.0 }),
        (11, Vec2 { x: 238.0, y: 257.0 }),
        (12, Vec2 { x: 554.0, y: 41.0 }),
      ]),
      lines: HashMap::<Line, u16>::from([
        (Line { from: 7, to: 11 }, 1),
        (Line { from: 6, to: 12 }, 1),
        (Line { from: 7, to: 6 }, 1),
        (Line { from: 5, to: 4 }, 1),
        (Line { from: 6, to: 5 }, 1),
        (Line { from: 8, to: 7 }, 1),
        (Line { from: 4, to: 8 }, 1),
        (Line { from: 4, to: 1 }, 1),
        (Line { from: 8, to: 3 }, 1),
        (Line { from: 7, to: 10 }, 1),
        (Line { from: 1, to: 2 }, 1),
        (Line { from: 8, to: 9 }, 1),
      ]),
      start: Some(4),
      end: Some(5),
      ..Graph::default()
    };

    let should_path = vec![4, 8, 7, 6, 5];

    graph.find_shortest_path();

    match graph.path
    {
      Some(path) =>
      {
        path
        .iter()
        .zip(should_path.iter())
        .for_each(|(path_id, should_id)|
        {
          assert_eq!(*path_id, *should_id);
        });
      }
      None => panic!("A path should have been found")
    }
  }
}
