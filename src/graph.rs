use std::cmp::Ordering;

use crate::utils::is_point_in_circle;

// TODO: consider using a Vec<u8> to store the points
/// ### Dijkstra Graph
///
/// Data is stored in an adjacency list as an array.
pub(crate) struct DijkstraGraph
{
  /// The actual graph data is stored here.
  ///
  /// Since we only allow 100 nodes and we identify them based on their id we can use the properties
  /// of an array to our advantage.
  points: [Option<DijkstraNode>; 100],

  start: Option<usize>,
  end: Option<usize>
}

impl Default for DijkstraGraph
{
  fn default() -> Self
  {
    return DijkstraGraph
    {
      points:
      [
        None, None, None, None, None, None, None, None, None, None,
        None, None, None, None, None, None, None, None, None, None,
        None, None, None, None, None, None, None, None, None, None,
        None, None, None, None, None, None, None, None, None, None,
        None, None, None, None, None, None, None, None, None, None,
        None, None, None, None, None, None, None, None, None, None,
        None, None, None, None, None, None, None, None, None, None,
        None, None, None, None, None, None, None, None, None, None,
        None, None, None, None, None, None, None, None, None, None,
        None, None, None, None, None, None, None, None, None, None,
      ],
      start: None,
      end: None,
    };
  }
}

impl DijkstraGraph
{
  pub(crate) fn new() -> DijkstraGraph
  { return DijkstraGraph { ..DijkstraGraph::default() }; }

  pub(crate) fn clear(&mut self)
  {
    self.points = self.points.iter()
      .map(|_| None)
      .collect::<Vec<_>>()
      .try_into()
      .unwrap();

    self.start = None;
    self.end = None;
  }

  pub(crate) fn clear_path(&mut self)
  {
    for option in self.points.iter_mut()
    {
      let Some(point) = option.as_mut() else { continue; };
      point.parent = None;
      point.visited = false;
    }
  }

  /// Returns the amount of nodes in the graph
  pub(crate) fn size(&self) -> usize
  {
    let mut size = 0;
    self.points.iter().for_each(|node| { if node.is_some() { size += 1; } });
    return size;
  }

  pub(crate) fn add_point(&mut self, id: usize, x: f32, y: f32)
  {
    if id > 100 { return; }
    if self.points[id].is_none() { self.points[id] = Some(DijkstraNode::new(x, y)); }
  }

  /// Inserts a node at the first missing instance of the array
  pub(crate) fn append_point(&mut self, x: f32, y: f32)
  {
    for (index, node_option) in self.points.iter().enumerate()
    {
      if node_option.is_none()
      {
        self.points[index] = Some(DijkstraNode::new(x, y));
        return;
      }
    }
  }

  pub(crate) fn remove_point(&mut self, id: usize)
  {
    if id > 100 { return; }
    self.points[id] = None;
  }

  /// Adds a line; if it already exists, the length gets updated
  pub(crate) fn add_line(&mut self, from: usize, to: usize, distance: u16)
  {
    if from > 100 || to > 100 { return; }

    let Some(point) = self.points[from].as_mut() else { return; };

    // Avoids duplicate edges
    for edge in point.edges.iter_mut()
    {
      if edge.destination == to
      {
        edge.distance = distance;
        return;
      }
    }

    point.edges.push(Edge { destination: to, distance });
  }

  pub(crate) fn remove_line(&mut self, from: usize, to: usize)
  {
    if from > 100 || to > 100 { return; }

    let Some(from_point) = self.points[from].as_mut() else { return; };
    from_point.edges.remove(to);
  }

  pub(crate) fn get(&self, id: usize) -> &Option<DijkstraNode>
  { return &self.points[id]; }

  pub(crate) fn get_mut(&mut self, id: usize) -> &mut Option<DijkstraNode>
  { return &mut self.points[id]; }

  pub(crate) fn start(&self) -> Option<usize>
  { return self.start; }

  pub(crate) fn set_start(&mut self, start: usize)
  {
    if start > 100 { return; }
    self.start = Some(start);
  }

  pub(crate) fn clear_start(&mut self)
  { self.start = None; }

  pub(crate) fn end(&self) -> Option<usize>
  { return self.end; }

  pub(crate) fn set_end(&mut self, end: usize)
  {
    if end > 100 { return; }
    self.end = Some(end);
  }

  pub(crate) fn clear_end(&mut self)
  { self.end = None; }

  /// Returns true if the shortest path has been found
  pub(crate) fn find_shortest_path(&mut self)
  {
    if self.start.is_none() || self.end.is_none() || (self.start.is_none() && self.end.is_none()) { return; }
    if self.points[self.start.unwrap()].is_none() { self.start = None; return; }
    if self.points[self.end.unwrap()].is_none() { self.end = None; return; }

    self.clear_path();

    self.points[self.start.unwrap()].as_mut().unwrap().distance = Some(0);
    self.points[self.start.unwrap()].as_mut().unwrap().parent = Some(self.start.unwrap());

    let mut unvisited_points = vec![];
    unvisited_points.push(self.start.unwrap());
    let mut current_id;
    let mut path_length: u16; // TODO: this

    // --- DIJKSTRA'S SHORTEST PATH ALGORITHM ---
    while !unvisited_points.is_empty()
    {
      /*
      unvisited_points.sort_by(|a, b|
      {
        // It is basically impossible, that unvisited_points contains ids that aren't in the graph
        let distance_a = self.points[*a].as_ref().unwrap().distance;
        let distance_b = self.points[*b].as_ref().unwrap().distance;

        // Distance = None serves as the infinite value
        match (distance_a, distance_b)
        {
          (None, None) => return Ordering::Equal,
          (Some(_), None) => return Ordering::Less,
          (None, Some(_)) => return Ordering::Greater,
          (Some(dist_a), Some(dist_b)) => return dist_a.cmp(&dist_b),
        };
      });
      */

      /*
      // Removing all points that have been marked as visited
      unvisited_points.retain(|id| !self.points[*id].as_ref().unwrap().visited);
      if unvisited_points.is_empty() { break; }
      */

      current_id = *unvisited_points.first().unwrap();
      unvisited_points.remove(0);

      if self.points[current_id].is_none() { continue; }

      let current_point = self.points[current_id].as_mut().unwrap();
      current_point.visited = true;
      let current_point_distance = current_point.distance.clone().unwrap();
      let edges = current_point.edges.clone();

      // drop(current_point);

      for edge in edges
      {
        let Some(neighbour) = self.points[edge.destination].as_mut() else { return; };

        if !neighbour.visited
        { unvisited_points.push(edge.destination); }

        let possibly_lower_goal = current_point_distance + edge.distance;

        if neighbour.distance.is_none() || neighbour.distance.unwrap() > possibly_lower_goal
        {
          neighbour.distance = Some(possibly_lower_goal);
          neighbour.parent = Some(current_id);
        }
      }
    }
  }

  pub(crate) fn get_path(&self) -> Option<Vec<usize>>
  {
    if self.start.is_none() || self.end.is_none() || (self.start.is_none() && self.end.is_none()) { return None; }

    let mut current_node = self.end.unwrap();
    let mut path = vec![current_node];

    for _ in 0..self.points.len()
    {
      if self.points[current_node].is_none() { return None; }
      if self.points[current_node].as_ref().unwrap().parent.is_none() { return None; }

      current_node = self.points[current_node].as_ref().unwrap().parent.unwrap();

      path.push(current_node);

      if current_node == self.start.unwrap() { break; }
    }

    path.reverse();

    return Some(path);
  }

  pub(crate) fn points(&self) -> &[Option<DijkstraNode>; 100]
  { return &self.points; }

  pub(crate) fn lines(&self) -> Vec<(usize, &DijkstraNode, u16, usize, &DijkstraNode)>
  {
    let mut lines = vec![];

    self.points.iter().enumerate()
      .filter(|(_, option)| option.is_some())
      .map(|(from_id, option)| (from_id, option.as_ref().unwrap()))
      .for_each(|(from_id, from_point)|
      {
        // It would take 2× as many lines of code to implement this in a functional style
        from_point.edges.iter()
          .for_each(|edge|
          {
            let Some(Some(to_point)) = self.points.get(edge.destination) else { return; };

            lines.push((from_id, from_point, edge.distance, edge.destination, to_point));
          });
      });

    return lines;
  }

  pub(crate) fn find_hovered_point(&mut self, mouse_x: f32, mouse_y: f32, radius: f32) -> Option<usize>
  {
    let mut point_id = None;

    self.points.iter().enumerate()
      .filter(|(_, option)| option.is_some())
      .map(|(index, option)| (index, option.as_ref().unwrap()))
      .for_each(|(index, point)|
      {
        if is_point_in_circle(mouse_x, mouse_y, point.x, point.y, radius)
        {
          point_id = Some(index);
        }
      });

    return point_id;
  }

  // !dbg
  pub(crate) fn print_graph_data(&self)
  {
    println!("Graph data:");
    self.lines().iter()
      .for_each(|(from_id, _, distance, to_id, _)|
      {
        println!("  {}--{}-->{}", from_id, distance, to_id);
      });

    match self.start
    {
      Some(id) => println!("Start: {}", id),
      None => println!("Start: None"),
    }

    match self.end
    {
      Some(id) => println!("End: {}", id),
      None => println!("End: None"),
    }

    match self.get_path()
    {
      Some(path) => println!("Path: {:?}", path),
      None => println!("Path: None")
    }
  }

  /// Replaces the current graph with a small one
  pub(crate) fn insert_small_graph(&mut self)
  {
    self.clear();

    self.append_point(942_f32, 355_f32);
    self.append_point(720_f32, 208_f32);
    self.append_point(198_f32, 342_f32);
    self.append_point(463_f32, 507_f32);
    self.append_point(735_f32, 513_f32);
    self.append_point(458_f32, 346_f32);
    self.append_point(468_f32, 202_f32);
    self.append_point(721_f32, 360_f32);

    self.add_line(3, 4, 3);
    self.add_line(2, 5, 5);
    self.add_line(5, 7, 4);
    self.add_line(6, 1, 5);
    self.add_line(1, 0, 5);
    self.add_line(5, 1, 7);
    self.add_line(3, 7, 5);
    self.add_line(7, 0, 4);
    self.add_line(2, 6, 4);
    self.add_line(2, 3, 7);
    self.add_line(6, 7, 6);
    self.add_line(5, 4, 8);
    self.add_line(4, 0, 3);
  }

  /// Replaces the current graph with a medium-sized one
  pub(crate) fn insert_medium_graph(&mut self)
  {
    self.clear();

    self.append_point(959_f32, 211_f32);
    self.append_point(967_f32, 394_f32);
    self.append_point(946_f32, 532_f32);
    self.append_point(144_f32, 377_f32);
    self.append_point(775_f32, 295_f32);
    self.append_point(734_f32, 523_f32);
    self.append_point(559_f32, 493_f32);
    self.append_point(570_f32, 361_f32);
    self.append_point(569_f32, 200_f32);
    self.append_point(353_f32, 206_f32);
    self.append_point(355_f32, 350_f32);
    self.append_point(342_f32, 488_f32);

    self.add_line(10, 6, 4);
    self.add_line(7, 1, 5);
    self.add_line(3, 9, 4);
    self.add_line(11, 6, 4);
    self.add_line(3, 11, 6);
    self.add_line(5, 2, 20);
    self.add_line(7, 4, 3);
    self.add_line(11, 7, 3);
    self.add_line(8, 4, 3);
    self.add_line(10, 7, 3);
    self.add_line(3, 10, 5);
    self.add_line(4, 0, 1);
    self.add_line(8, 0, 5);
    self.add_line(9, 8, 4);
    self.add_line(6, 5, 7);
    self.add_line(4, 1, 2);
  }

  /// Replaces the current graph with a large one
  pub(crate) fn insert_large_graph(&mut self)
  {
    self.clear();

    self.append_point(595_f32, 640_f32);
    self.append_point(864_f32, 300_f32);
    self.append_point(550_f32, 369_f32);
    self.append_point(280_f32, 606_f32);
    self.append_point(748_f32, 127_f32);
    self.append_point(177_f32, 71_f32);
    self.append_point(467_f32, 84_f32);
    self.append_point(260_f32, 431_f32);
    self.append_point(928_f32, 642_f32);
    self.append_point(466_f32, 181_f32);
    self.append_point(433_f32, 27_f32);
    self.append_point(667_f32, 52_f32);
    self.append_point(847_f32, 75_f32);
    self.append_point(734_f32, 270_f32);
    self.append_point(931_f32, 233_f32);
    self.append_point(904_f32, 389_f32);
    self.append_point(423_f32, 467_f32);
    self.append_point(445_f32, 551_f32);
    self.append_point(691_f32, 559_f32);

    self.add_line(11, 12, 1);
    self.add_line(5, 7, 12);
    self.add_line(13, 2, 1);
    self.add_line(15, 8, 10);
    self.add_line(14, 8, 14);
    self.add_line(1, 18, 9);
    self.add_line(17, 18, 3);
    self.add_line(16, 17, 2);
    self.add_line(7, 3, 1);
    self.add_line(0, 8, 1);
    self.add_line(6, 4, 1);
    self.add_line(15, 2, 2);
    self.add_line(2, 7, 1);
    self.add_line(2, 16, 3);
    self.add_line(14, 15, 1);
    self.add_line(4, 13, 3);
    self.add_line(9, 2, 8);
    self.add_line(12, 1, 2);
    self.add_line(11, 4, 2);
    self.add_line(10, 11, 1);
    self.add_line(5, 10, 2);
    self.add_line(9, 4, 3);
    self.add_line(4, 1, 1);
    self.add_line(15, 16, 5);
    self.add_line(5, 6, 1);
    self.add_line(17, 0, 1);
    self.add_line(5, 9, 2);
    self.add_line(1, 2, 1);
    self.add_line(18, 8, 4);
    self.add_line(16, 3, 2);
    self.add_line(12, 14, 1);
    self.add_line(3, 0, 1);
  }
}

#[derive(Clone, Debug)]
pub(crate) struct DijkstraNode
{
  pub(crate) x: f32,
  pub(crate) y: f32,
  parent: Option<usize>,
  distance: Option<u16>,
  visited: bool,
  edges: Vec<Edge>,
}

impl DijkstraNode
{
  fn new(x: f32, y: f32) -> Self
  {
    DijkstraNode
    {
      x,
      y,
      parent: None,
      distance: None,
      visited: false,
      edges: vec![],
    }
  }
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct Edge
{
  destination: usize,
  distance: u16,
}

// Tests
#[path = "./tests/graph_tests.rs"]
#[cfg(test)]
mod graph_tests;
