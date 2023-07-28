use crate::{draw_pill, utils};
use itertools::Itertools;
use macroquad::{
  prelude::{mouse_position, Color, IVec2, BLACK, GREEN, MAGENTA, ORANGE, YELLOW},
  shapes::{draw_circle, draw_circle_lines, draw_line, draw_triangle},
  text::{draw_text, get_text_center, measure_text},
};
use std::{
  collections::{BTreeMap, HashMap},
  fmt::Display,
  ops::{Div, Mul},
};

/// ### Graph
///
/// It contains nodes and edges connecting those nodes.
pub(crate) struct Graph {
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
  points: BTreeMap<u8, Node>,

  /// Contains all data for the lines
  ///
  /// Key: Line (2 ids)
  ///
  /// Value: line length
  lines: HashMap<Line, u16>,

  /// The path is a vector of all the point ids that the graph traverses
  ///
  /// The 0th element is the start, the last element is the end
  path: Option<Vec<u8>>,
}

impl Default for Graph {
  fn default() -> Self {
    return Graph {
      start: None,
      end: None,
      hovered_point_id: None,
      selected_point_id: None,
      has_hovered_point_been_checked: false,
      max_amount_of_points: 100,
      radius: 13,
      padding: 3,
      points: BTreeMap::<u8, Node>::new(),
      lines: HashMap::<Line, u16>::new(),
      path: None,
    };
  }
}

impl Graph {
  pub fn new() -> Graph {
    return Graph { ..Graph::default() };
  }

  pub fn points_amount(&self) -> usize {
    return self
      .points
      .len();
  }

  pub fn add_point(&mut self, coordinates: IVec2) {
    // Limiting the amount of available points to 100
    if self.points_amount() >= self.max_amount_of_points as usize {
      return;
    }

    let mut smallest_missing_id = 1;

    // Finding the smallest missing point id
    for (point_id, _) in self
      .points
      .iter()
    {
      // Incrementing the missing id until it doesn't match a given point id
      if *point_id == smallest_missing_id {
        smallest_missing_id += 1;
        continue;
      }
    }

    self
      .points
      .insert(smallest_missing_id, Node::new(coordinates, smallest_missing_id));

    self.clear_path();
  }

  pub fn remove_point(&mut self, id: u8) {
    // Deleting all lines associated with this point
    self
      .lines
      .retain(|line, _| {
        return line.from != id && line.to != id;
      });

    self
      .points
      .remove(&id);

    self.clear_path();
  }

  pub fn set_point_coordinates(&mut self, point_id: u8, new_position: IVec2) {
    if let Some(node) = self
      .points
      .get_mut(&point_id)
    {
      node.position = new_position;
    }
  }

  /// Adds a line; if it already exists, the length gets updated
  pub fn add_line(&mut self, from_id: u8, to_id: u8, line_length: u16) {
    let new_line = Line {
      from: from_id,
      to: to_id,
    };

    match self
      .lines
      .get_mut(&new_line)
    {
      Some(length) => *length = line_length,
      None => {
        _ = self
          .lines
          .insert(new_line, line_length)
      },
    }

    self.clear_path();
  }

  pub fn remove_line(&mut self, from_id: u8, to_id: u8) {
    let line = Line {
      from: from_id,
      to: to_id,
    };

    self
      .lines
      .remove(&line);

    self.clear_path();
  }

  pub fn find_hovered_point(&mut self) -> Option<u8> {
    self.has_hovered_point_been_checked = true;

    for (id, node) in self
      .points
      .iter()
    {
      if utils::is_point_in_circle(
        mouse_position().0,
        mouse_position().1,
        node
          .position
          .x as f32,
        node
          .position
          .y as f32,
        self.radius as f32,
      ) {
        self.hovered_point_id = Some(*id);
        return Some(*id);
      }
    }

    return None;
  }

  pub fn get_hovered_point_id(&mut self) -> Option<u8> {
    if !self.has_hovered_point_been_checked {
      self.has_hovered_point_been_checked = true;
      return self.find_hovered_point();
    }

    return self.hovered_point_id;
  }

  pub fn get_radius(&self) -> u8 {
    return self.radius;
  }

  /// Every time the graph gets changed, the path gets cleared b/c the graph might have been changed
  /// in a way that would change the shortest path from `start` to `end`
  pub fn clear_path(&mut self) {
    self.path = None;
    self
      .points
      .iter_mut()
      .for_each(|(id, node)| {
        node.parent = *id;
        node.visited = false;
        node.distance = u32::MAX;
      });
  }

  pub fn clear(&mut self) {
    self.clear_path();
    self
      .lines
      .clear();
    self
      .points
      .clear();
    self.start = None;
    self.end = None;
    self.hovered_point_id = None;
    self.selected_point_id = None;
    self.has_hovered_point_been_checked = false;
  }

  /// Finds the shortest path from the start to the end point using dijkstra's shortest path algorithm
  pub fn find_shortest_path(&mut self) {
    let Some(start) = self.start else { return; };
    let Some(end) = self.end else { return; };

    // TODO: don't use unwrap()
    // Inserting the start node so the list won't be empty
    let mut untested_nodes = Vec::<u8>::new();
    untested_nodes.push(start);
    self
      .points
      .get_mut(&start)
      .unwrap()
      .distance = 0;

    // --- DIJKSTRA'S SHORTEST PATH ALGORITHM ---

    // TOFIX: large graph doesn't give shortest path from 6 to 18
    while !untested_nodes.is_empty() {
      // Remove all visited nodes
      untested_nodes.retain(|id| {
        return match self
          .points
          .get(id)
        {
          Some(node) => !node.visited,
          None => false,
        };
      });

      if untested_nodes.is_empty() {
        break;
      }

      let current_node_id = untested_nodes
        .first()
        .unwrap();
      let current_node_copy = self
        .points
        .get_mut(current_node_id)
        .unwrap()
        .clone();

      // Set the current node to visited
      self
        .points
        .get_mut(current_node_id)
        .unwrap()
        .visited = true;

      let mut new_untested_nodes = Vec::<u8>::new();

      // Test the neighbours of the current node
      for (line, line_length) in self
        .lines
        .iter()
      {
        // Only process the neighbours of the current node
        if line.from != *current_node_id {
          continue;
        }

        let neighbour = self
          .points
          .get_mut(&line.to)
          .unwrap();

        new_untested_nodes.push(line.to);

        if current_node_copy.distance + (*line_length as u32) < neighbour.distance {
          neighbour.parent = *current_node_id;
          neighbour.distance = current_node_copy.distance + (*line_length as u32);
        }
      }

      // Add all found untested neighbours to the untested nodes
      untested_nodes.append(&mut new_untested_nodes);
    }

    // --- Dijkstra's algorithm is over ---

    // Extracting the path data from the graph

    self.path = Some(Vec::<u8>::new());

    let path = self
      .path
      .as_mut()
      .unwrap();

    let mut current_node = end;

    for _ in 0..self
      .points
      .len()
    {
      let Some(next_node) = self
        .points
        .get(&current_node) else { return; };
      let next_node = next_node.parent;

      if next_node == current_node {
        self.path = None;
        return;
      }

      // Push the current node onto the path and go to the next node
      path.push(current_node);

      current_node = next_node;

      // Pushing the start onto the path and exiting the loop
      if current_node == start {
        path.push(start);
        break;
      }
    }

    path.reverse();
  }

  // !dbg
  pub fn print_path(&self) {
    if let Some(path) = &self.path {
      println!("{:?}", path);
    }
  }

  // TODO: print arrowhead over the path
  // TODO: use functional pattern if possible
  pub fn paint_path(&self) {
    if let Some(path) = &self.path {
      for (from, to) in path
        .into_iter()
        .tuple_windows::<(&u8, &u8)>()
      {
        let Some(from_point) = self.points.get(from) else { continue; };
        let Some(to_point) = self.points.get(to) else { continue; };

        draw_line(
          from_point
            .position
            .x as f32,
          from_point
            .position
            .y as f32,
          to_point
            .position
            .x as f32,
          to_point
            .position
            .y as f32,
          2.0,
          GREEN,
        );
      }
    }
  }

  pub fn paint_points(&mut self) {
    // Painting all points and centering the text
    for (id, node) in self
      .points
      .iter()
    {
      draw_circle(
        node
          .position
          .x as f32,
        node
          .position
          .y as f32,
        self.radius as f32,
        if self.selected_point_id == Some(*id) { YELLOW } else { ORANGE },
      );

      let text_center = get_text_center(
        id.to_string()
          .as_str(),
        None,
        20,
        1.0,
        0.0,
      );

      draw_text(
        id.to_string()
          .as_str(),
        node
          .position
          .x as f32
          - text_center.x,
        node
          .position
          .y as f32
          - text_center.y,
        20.0,
        BLACK,
      );
    }

    // Checking for the hovered point id (if it hasn't been done already)
    if !self.has_hovered_point_been_checked {
      self.find_hovered_point();
    }

    // TODO: consider replacing this with Option::inspect
    // Painting an outline for the hovered point (if it exists)
    if let Some(hovered_point_id) = self.hovered_point_id {
      if let Some(node) = self
        .points
        .get(&hovered_point_id)
      {
        draw_circle_lines(
          node
            .position
            .x as f32,
          node
            .position
            .y as f32,
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

  pub fn paint_lines(&self) {
    for (line, _) in self
      .lines
      .iter()
    {
      match (
        self
          .points
          .get(&line.from),
        self
          .points
          .get(&line.to),
      ) {
        (Some(from_point), Some(to_point)) => {
          let direction = IVec2 {
            x: from_point
              .position
              .x
              - to_point
                .position
                .x,
            y: from_point
              .position
              .y
              - to_point
                .position
                .y,
          };

          let arrow_head_location = IVec2 {
            x: to_point
              .position
              .x
              + (direction.x as f32
                * ((self.radius + 2) as f32
                  / direction
                    .as_vec2()
                    .length())) as i32,
            y: to_point
              .position
              .y
              + (direction.y as f32
                * ((self.radius + 2) as f32
                  / direction
                    .as_vec2()
                    .length())) as i32,
          };

          // This point is at the base of the arrow head that "connects" it to the line
          let helper_point = IVec2 {
            x: to_point
              .position
              .x
              + (direction.x as f32
                * ((self.radius + 15) as f32
                  / direction
                    .as_vec2()
                    .length())) as i32,
            y: to_point
              .position
              .y
              + (direction.y as f32
                * ((self.radius + 15) as f32
                  / direction
                    .as_vec2()
                    .length())) as i32,
          };

          // The angle is in radians
          let angle: f32 = 0.436;
          let arrow_head_length = 20.0;

          // Calculating the tip of the triangle that touches the node (position + (direction * (radius / length)))
          draw_line(
            from_point
              .position
              .x as f32
              + (direction.x as f32
                * (-(self.radius as f32)
                  / direction
                    .as_vec2()
                    .length())),
            from_point
              .position
              .y as f32
              + (direction.y as f32
                * (-(self.radius as f32)
                  / direction
                    .as_vec2()
                    .length())),
            arrow_head_location.x as f32,
            arrow_head_location.y as f32,
            1.0,
            Color::from_rgba(0, 255, 255, 255),
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
            arrow_head_location.as_vec2(),
            helper_point.as_vec2(),
            IVec2 {
              x: arrow_head_location.x
                + ((arrow_head_length
                  / direction
                    .as_vec2()
                    .length())
                  * (((from_point
                    .position
                    .x
                    - to_point
                      .position
                      .x) as f32
                    * angle.cos())
                    - ((from_point
                      .position
                      .y
                      - to_point
                        .position
                        .y) as f32
                      * angle.sin()))) as i32,
              y: arrow_head_location.y
                + ((arrow_head_length
                  / direction
                    .as_vec2()
                    .length())
                  * (((from_point
                    .position
                    .y
                    - to_point
                      .position
                      .y) as f32
                    * angle.cos())
                    + ((from_point
                      .position
                      .x
                      - to_point
                        .position
                        .x) as f32
                      * angle.sin()))) as i32,
            }
            .as_vec2(),
            Color::from_rgba(0, 255, 255, 255),
          );

          // Right arrow head wing
          draw_triangle(
            arrow_head_location.as_vec2(),
            helper_point.as_vec2(),
            IVec2 {
              x: arrow_head_location.x
                + ((arrow_head_length
                  / direction
                    .as_vec2()
                    .length())
                  * (((from_point
                    .position
                    .x
                    - to_point
                      .position
                      .x) as f32
                    * angle.cos())
                    + ((from_point
                      .position
                      .y
                      - to_point
                        .position
                        .y) as f32
                      * angle.sin()))) as i32,
              y: arrow_head_location.y
                + ((arrow_head_length
                  / direction
                    .as_vec2()
                    .length())
                  * (((from_point
                    .position
                    .y
                    - to_point
                      .position
                      .y) as f32
                    * angle.cos())
                    - ((from_point
                      .position
                      .x
                      - to_point
                        .position
                        .x) as f32
                      * angle.sin()))) as i32,
            }
            .as_vec2(),
            Color::from_rgba(0, 255, 255, 255),
          );
        },

        (_, _) => (),
      }
    }
  }

  pub fn paint_line_lengths(&self) {
    for (line, length) in self
      .lines
      .iter()
    {
      match (
        self
          .points
          .get(&line.from),
        self
          .points
          .get(&line.to),
      ) {
        (Some(from_point), Some(to_point)) => {
          let position = IVec2 {
            x: ((1.0 / 3.0)
              * from_point
                .position
                .x as f32
              + (2.0 / 3.0)
                * to_point
                  .position
                  .x as f32) as i32,
            y: ((1.0 / 3.0)
              * from_point
                .position
                .y as f32
              + (2.0 / 3.0)
                * to_point
                  .position
                  .y as f32) as i32,
          };

          let text_center = get_text_center(
            length
              .to_string()
              .as_str(),
            None,
            20,
            1.0,
            0.0,
          );

          let text_dimensions = measure_text(
            length
              .to_string()
              .as_str(),
            None,
            20,
            1.0,
          );

          draw_pill(
            position.x as f32
              - text_dimensions
                .width
                .div(2.0),
            position.y as f32
              - text_dimensions
                .height
                .div(2.0)
              - self.padding as f32,
            text_dimensions.width,
            text_dimensions.height
              + self
                .padding
                .mul(2) as f32,
            Color::from_rgba(0, 255, 255, 255),
          );

          draw_text(
            length
              .to_string()
              .as_str(),
            position.x as f32 - text_center.x,
            position.y as f32 - text_center.y,
            20.0,
            BLACK,
          );
        },
        (_, _) => (),
      }
    }
  }

  /// The `position` is the center of the point over which the label is painted.
  pub fn paint_label(&self, text: &str, position: &IVec2) {
    let text_center = get_text_center(text, None, 20, 1.0, 0.0);

    let text_dimensions = measure_text(text, None, 20, 1.0);

    // A 2 pixel gap between the label and the point is hard-coded
    draw_pill(
      position.x as f32
        - text_dimensions
          .width
          .div(2.0),
      position.y as f32
        - text_dimensions.height
        - self.radius as f32
        - self
          .padding
          .mul(2) as f32
        - 2.0,
      text_dimensions.width,
      text_dimensions.height
        + self
          .padding
          .mul(2) as f32,
      GREEN,
    );

    draw_text(
      text,
      position.x as f32 - text_center.x,
      position.y as f32
        - text_center.y as f32
        - self.radius as f32
        - text_dimensions
          .height
          .div(2.0)
        - self.padding as f32
        - 2.0,
      20.0,
      Color::from_rgba(20, 0, 40, 255),
    );
  }

  pub fn paint_graph(&mut self) {
    // Paints lines
    if !self
      .lines
      .is_empty()
    {
      self.paint_lines();
      self.paint_path();
      self.paint_line_lengths();
    }

    // Paints points
    if !self
      .points
      .is_empty()
    {
      self.paint_points();
    }

    // TODO: consider replacing this with Option::inspect
    // Paints start label
    if let Some(start_id) = self.start {
      if let Some(start_point) = self
        .points
        .get(&start_id)
      {
        self.paint_label("Start", &start_point.position);
      }
    }

    // TODO: consider replacing this with Option::inspect
    // Paints end label
    if let Some(end_id) = self.end {
      if let Some(end_point) = self
        .points
        .get(&end_id)
      {
        self.paint_label("End", &end_point.position);
      }
    }
  }

  pub fn print_graph_data(&self) {
    println!("Points:");
    self
      .points
      .iter()
      .for_each(|point| {
        println!("{}: {:?}", point.0, point.1);
      });

    println!("Lines:");
    self
      .lines
      .iter()
      .for_each(|line| {
        println!("{}: {}", line.0, line.1);
      });

    match self.start {
      Some(id) => println!("Start: {}", id),
      None => println!("Start: None"),
    }

    match self.end {
      Some(id) => println!("End: {}", id),
      None => println!("End: None"),
    }
  }

  /// Replaces the current graph with a small one
  pub fn insert_small_graph(&mut self) {
    self.clear();

    self.points = BTreeMap::from([
      (1, Node::new(IVec2 { x: 942, y: 355 }, 1)),
      (2, Node::new(IVec2 { x: 720, y: 208 }, 2)),
      (3, Node::new(IVec2 { x: 198, y: 342 }, 3)),
      (4, Node::new(IVec2 { x: 463, y: 507 }, 4)),
      (5, Node::new(IVec2 { x: 735, y: 513 }, 5)),
      (6, Node::new(IVec2 { x: 458, y: 346 }, 6)),
      (7, Node::new(IVec2 { x: 468, y: 202 }, 7)),
      (8, Node::new(IVec2 { x: 721, y: 360 }, 8)),
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
  pub fn insert_medium_graph(&mut self) {
    self.clear();

    self.points = BTreeMap::<u8, Node>::from([
      (1, Node::new(IVec2 { x: 959, y: 211 }, 1)),
      (2, Node::new(IVec2 { x: 967, y: 394 }, 2)),
      (3, Node::new(IVec2 { x: 946, y: 532 }, 3)),
      (4, Node::new(IVec2 { x: 144, y: 377 }, 4)),
      (5, Node::new(IVec2 { x: 775, y: 295 }, 5)),
      (6, Node::new(IVec2 { x: 734, y: 523 }, 6)),
      (7, Node::new(IVec2 { x: 559, y: 493 }, 7)),
      (8, Node::new(IVec2 { x: 570, y: 361 }, 8)),
      (9, Node::new(IVec2 { x: 569, y: 200 }, 9)),
      (10, Node::new(IVec2 { x: 353, y: 206 }, 10)),
      (11, Node::new(IVec2 { x: 355, y: 350 }, 11)),
      (12, Node::new(IVec2 { x: 342, y: 488 }, 12)),
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

  pub fn insert_large_graph(&mut self) {
    self.clear();

    self.points = BTreeMap::<u8, Node>::from([
      (1, Node::new(IVec2 { x: 595, y: 640 }, 1)),
      (2, Node::new(IVec2 { x: 864, y: 300 }, 2)),
      (3, Node::new(IVec2 { x: 550, y: 369 }, 3)),
      (4, Node::new(IVec2 { x: 280, y: 606 }, 4)),
      (5, Node::new(IVec2 { x: 748, y: 127 }, 5)),
      (6, Node::new(IVec2 { x: 177, y: 71 }, 6)),
      (7, Node::new(IVec2 { x: 467, y: 84 }, 7)),
      (8, Node::new(IVec2 { x: 260, y: 431 }, 8)),
      (9, Node::new(IVec2 { x: 928, y: 642 }, 9)),
      (10, Node::new(IVec2 { x: 466, y: 181 }, 10)),
      (11, Node::new(IVec2 { x: 433, y: 27 }, 11)),
      (12, Node::new(IVec2 { x: 667, y: 52 }, 12)),
      (13, Node::new(IVec2 { x: 847, y: 75 }, 13)),
      (14, Node::new(IVec2 { x: 734, y: 270 }, 14)),
      (15, Node::new(IVec2 { x: 931, y: 233 }, 15)),
      (16, Node::new(IVec2 { x: 904, y: 389 }, 16)),
      (17, Node::new(IVec2 { x: 423, y: 467 }, 17)),
      (18, Node::new(IVec2 { x: 445, y: 551 }, 18)),
      (19, Node::new(IVec2 { x: 691, y: 559 }, 19)),
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
struct Line {
  from: u8,
  to: u8,
}

impl PartialEq for Line {
  fn eq(&self, other: &Self) -> bool {
    return self.from == other.from && self.to == other.to;
  }
}

impl Eq for Line {}

impl Display for Line {
  fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    return write!(formatter, "({}, {})", self.from, self.to);
  }
}

/// ### The Node struct
///
/// It contains additional fields needed for dijkstra's shortest path algorithm
#[derive(Debug, Clone, Copy)]
struct Node {
  position: IVec2,
  visited: bool,
  parent: u8,
  distance: u32,
}

impl Node {
  pub fn new(position: IVec2, parent: u8) -> Self {
    return Node {
      position,
      visited: false,
      parent,
      distance: u32::MAX,
    };
  }
}

// Tests

#[path = "./tests/graph.rs"]
#[cfg(test)]
mod graph;
