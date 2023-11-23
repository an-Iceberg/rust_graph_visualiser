use crate::utils;

// TODO: add iterator over points
// TODO: add iterator bfs
// TODO: add iterator dfs
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
  graph: [Option<DijkstraNode>; 100],

  start: Option<usize>,
  end: Option<usize>
}

struct DijkstraNode
{
  position: Position,
  parent: Option<usize>,
  distance: Option<u16>,
  visited: bool,
  edges: Vec<Edge>,
}

struct Edge
{
  destination: usize,
  distance: u16,
}

struct Position
{
  x: f32,
  y: f32,
}

impl Default for DijkstraGraph
{
  fn default() -> Self
  {
    return DijkstraGraph
    {
      graph:
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
  pub fn new() -> DijkstraGraph
  { return DijkstraGraph { ..DijkstraGraph::default() }; }

  pub fn clear(&mut self)
  {
    for mut node in self.graph.iter_mut()
    { node = &mut None; }
    self.start = None;
    self.end = None;
  }

  pub fn clear_path(&mut self)
  {
    for mut node in self.graph.iter_mut()
    {
      if let Some(node) = node
      { node.parent = None; }
    }
  }

  /// Returns the amount of nodes in the graph
  pub fn size(&self) -> usize
  {
    let mut size = 0;
    self.graph.iter().for_each(|node| { if node.is_some() { size += 1; } });
    return size;
  }

  pub fn add_point(&mut self, id: usize, x: f32, y: f32)
  {
    if id > 100 { return; }
    if self.graph[id].is_none() { self.graph[id] = Some(DijkstraNode::new(x, y)); }
  }

  /// Inserts a node at the first missing instance of the array
  pub fn append_point(&mut self, x: f32, y: f32)
  {
    for (index, node_option) in self.graph.iter().enumerate()
    {
      if node_option.is_none()
      {
        self.graph[index] = Some(DijkstraNode::new(x, y));
        return;
      }
    }
  }

  pub fn remove_point(&mut self, id: usize)
  {
    if id > 100 { return; }
    self.graph[id] = None;
  }

  /// Adds a line; if it already exists, the length gets updated
  fn add_line(&mut self, from: usize, to: usize, distance: u16)
  {
    if from > 100 || to > 100 { return; }

    if let Some(node) = &mut self.graph[from]
    { node.edges.push(Edge { destination: to, distance }); }
  }

  fn remove_line(&mut self, from: usize, to: usize)
  {
    if from > 100 || to > 100 { return; }

    if let Some(node) = &mut self.graph[from]
    {
      if node.edges.get(to).is_some()
      { node.edges.remove(to); }
    }
  }

  fn update_node_position(&mut self, id: usize, x: f32, y: f32)
  {
    if id > 100 { return; }
    if let Some(node) = &mut self.graph[id] { node.update_position(x, y); }
  }

  fn update_line_distance(&mut self, from: usize, to: usize, distance: u16)
  {
    if from > 100 || to > 100 { return; }

    if let Some(node) = &mut self.graph[from]
    {
      if let Some(edge) = node.edges.get(to)
      { edge.update_distance(distance); }
    }
  }

  /// Returns true if the shortest path has been found
  fn find_shortest_path(&mut self, start: usize, end: usize) -> bool
  {
    if start > 100 || end > 100 { return false; }
    self.start = Some(start);
    self.end = Some(end);

    todo!();
    // --- DIJKSTRA'S SHORTEST PATH ALGORITHM ---
  }

  fn get_path(&self) -> Option<Vec<usize>>
  {
    if self.start.is_none() || self.end.is_none() { return None; }

    let mut current_node = self.start.unwrap();
    let mut path = vec![current_node];

    for _ in 0..self.graph.len()
    {
      if self.graph[current_node].is_none() { return None; }
      if self.graph[current_node].unwrap().parent.is_none() { return None; }

      current_node = self.graph[current_node].unwrap().parent.unwrap();

      path.push(current_node);

      if current_node == self.end.unwrap() { break; }
    }

    return Some(path);
  }

  fn get_points(&self)
  {
    todo!();
  }

  fn get_lines(&self)
  {
    todo!();
  }

  pub fn find_hovered_point(&mut self, mouse_x: f32, mouse_y: f32, radius: f32) -> Option<usize>
  {
    for (index, node_option) in self.graph.iter().enumerate()
    {
      if let Some(node) = node_option
      {
        if utils::is_point_in_circle(mouse_x, mouse_y, node.position.x, node.position.y, radius)
        { return Some(index); }
      }
    }

    return None;
  }

  // !dbg
  pub fn print_path(&self)
  {
    let Some(path) = self.get_path() else { return; };
    println!("{:?}", path);
  }

  // !dbg
  pub fn print_graph_data(&self)
  {
    println!("Points:");
    self.graph.iter().enumerate().for_each(|(index, node_option)|
    {
      if node_option.is_some()
      { print!("{} ", index); }
    });

    println!("Lines:");
    self.graph.iter().enumerate().for_each(|(index, node_option)|
    {
      if let Some(node) = node_option
      {
        node.edges.iter().for_each(|edge|
        { print!("{}->{} ", index, edge.destination); });
      }
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
  }

  /// Replaces the current graph with a small one
  pub fn insert_small_graph(&mut self)
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
  pub fn insert_medium_graph(&mut self)
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
  pub fn insert_large_graph(&mut self)
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

impl DijkstraNode
{
  fn new(x: f32, y: f32) -> Self
  {
    DijkstraNode
    {
      position: Position { x, y },
      parent: None,
      distance: None,
      visited: false,
      edges: vec![],
    }
  }

  pub fn update_position(&mut self, x: f32, y: f32)
  { self.position.update(x, y); }
}

impl Position
{
  pub fn get_x(&self) -> f32
  { return self.x; }

  pub fn get_y(&self) -> f32
  { return self.y; }

  pub fn update(&mut self, x: f32, y: f32)
  { self.x = x; self.y = y; }
}

impl Edge
{
  pub fn update_distance(&mut self, distance: u16)
  { self.distance = distance; }
}

// Tests
#[path = "./tests/graph_tests.rs"]
#[cfg(test)]
mod graph_tests;
