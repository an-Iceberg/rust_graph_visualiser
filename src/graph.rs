use crate::utils;

// TODO: add iterator over points
// TODO: add iterator bfs
// TODO: add iterator dfs
// TODO: consider using a Vec<u8> to store the points
// TODO: Extract painting the graph thru macroquad to a different module
// TODO: Some fields in this struct might be better stored in the main module
/// ### Graph
///
/// It contains nodes and edges connecting those nodes.
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
  pub fn insert_small_graph_a(&mut self)
  {
    self.clear();

    self.add_point(1, 942_f32, 355_f32);
    self.add_point(2, 720_f32, 208_f32);
    self.add_point(3, 198_f32, 342_f32);
    self.add_point(4, 463_f32, 507_f32);
    self.add_point(5, 735_f32, 513_f32);
    self.add_point(6, 458_f32, 346_f32);
    self.add_point(7, 468_f32, 202_f32);
    self.add_point(8, 721_f32, 360_f32);

    self.add_line(4, 5, 3);
    self.add_line(3, 6, 5);
    self.add_line(6, 8, 4);
    self.add_line(7, 2, 5);
    self.add_line(2, 1, 5);
    self.add_line(6, 2, 7);
    self.add_line(4, 8, 5);
    self.add_line(8, 1, 4);
    self.add_line(3, 7, 4);
    self.add_line(3, 4, 7);
    self.add_line(7, 8, 6);
    self.add_line(6, 5, 8);
    self.add_line(5, 1, 3);
  }

  /// Replaces the current graph with a small one
  pub fn insert_small_graph_b(&mut self)
  {
    self.clear();

    let mut graph = DijkstraGraph {
      points: BTreeMap::<u8, DijkstraNode>::from([
        (1, DijkstraNode::new(IVec2 { x: 783, y: 102 }, 1)),
        (2, DijkstraNode::new(IVec2 { x: 412, y: 295 }, 2)),
        (3, DijkstraNode::new(IVec2 { x: 680, y: 308 }, 3)),
        (4, DijkstraNode::new(IVec2 { x: 509, y: 459 }, 4)),
        (5, DijkstraNode::new(IVec2 { x: 330, y: 603 }, 5)),
        (6, DijkstraNode::new(IVec2 { x: 160, y: 442 }, 6)),
        (7, DijkstraNode::new(IVec2 { x: 174, y: 196 }, 7)),
        (8, DijkstraNode::new(IVec2 { x: 411, y: 78 }, 8)),
        (9, DijkstraNode::new(IVec2 { x: 1003, y: 239 }, 9)),
      ]),
      lines: HashMap::<Edge, u16>::from([
        (Edge { from: 4, to: 5 }, 2),
        (Edge { from: 3, to: 4 }, 3),
        (Edge { from: 2, to: 6 }, 3),
        (Edge { from: 1, to: 9 }, 7),
        (Edge { from: 4, to: 2 }, 1),
        (Edge { from: 9, to: 3 }, 1),
        (Edge { from: 6, to: 2 }, 3),
        (Edge { from: 7, to: 8 }, 2),
        (Edge { from: 2, to: 4 }, 1),
        (Edge { from: 2, to: 8 }, 3),
        (Edge { from: 2, to: 7 }, 5),
        (Edge { from: 2, to: 1 }, 1),
        (Edge { from: 5, to: 6 }, 2),
        (Edge { from: 1, to: 2 }, 1),
        (Edge { from: 3, to: 9 }, 1),
        (Edge { from: 4, to: 3 }, 3),
        (Edge { from: 1, to: 8 }, 1),
        (Edge { from: 8, to: 1 }, 1),
        (Edge { from: 6, to: 7 }, 2),
        (Edge { from: 8, to: 7 }, 2),
        (Edge { from: 8, to: 2 }, 3),
        (Edge { from: 2, to: 3 }, 1),
        (Edge { from: 7, to: 2 }, 5),
        (Edge { from: 9, to: 1 }, 7),
        (Edge { from: 3, to: 2 }, 1),
        (Edge { from: 5, to: 4 }, 2),
        (Edge { from: 6, to: 5 }, 2),
        (Edge { from: 7, to: 6 }, 2),
      ]),
      start: Some(7),
      end: Some(9),
      ..DijkstraGraph::default()
    };
  }

  /// Replaces the current graph with a medium-sized one
  pub fn insert_medium_graph(&mut self)
  {
    self.clear();

    self.add_point(1, 959_f32, 211_f32);
    self.add_point(2, 967_f32, 394_f32);
    self.add_point(3, 946_f32, 532_f32);
    self.add_point(4, 144_f32, 377_f32);
    self.add_point(5, 775_f32, 295_f32);
    self.add_point(6, 734_f32, 523_f32);
    self.add_point(7, 559_f32, 493_f32);
    self.add_point(8, 570_f32, 361_f32);
    self.add_point(9, 569_f32, 200_f32);
    self.add_point(10, 353_f32, 206_f32);
    self.add_point(11, 355_f32, 350_f32);
    self.add_point(12, 342_f32, 488_f32);

    self.add_line(11, 7, 4);
    self.add_line(8, 2, 5);
    self.add_line(4, 10, 4);
    self.add_line(12, 7, 4);
    self.add_line(4, 12, 6);
    self.add_line(6, 3, 20);
    self.add_line(8, 5, 3);
    self.add_line(12, 8, 3);
    self.add_line(9, 5, 3);
    self.add_line(11, 8, 3);
    self.add_line(4, 11, 5);
    self.add_line(5, 1, 1);
    self.add_line(9, 1, 5);
    self.add_line(10, 9, 4);
    self.add_line(7, 6, 7);
    self.add_line(5, 2, 2);
  }

  /// Replaces the current graph with a medium-sized one
  pub fn insert_medium_graph(&mut self)
  {
    self.clear();

  let mut graph = DijkstraGraph {
    points: BTreeMap::<u8, DijkstraNode>::from([
      (1, DijkstraNode::new(IVec2 { x: 970, y: 108 }, 1)),
      (2, DijkstraNode::new(IVec2 { x: 991, y: 340 }, 2)),
      (3, DijkstraNode::new(IVec2 { x: 1023, y: 580 }, 3)),
      (4, DijkstraNode::new(IVec2 { x: 509, y: 459 }, 4)),
      (5, DijkstraNode::new(IVec2 { x: 750, y: 537 }, 5)),
      (6, DijkstraNode::new(IVec2 { x: 747, y: 262 }, 6)),
      (7, DijkstraNode::new(IVec2 { x: 535, y: 237 }, 7)),
      (8, DijkstraNode::new(IVec2 { x: 497, y: 433 }, 8)),
      (9, DijkstraNode::new(IVec2 { x: 352, y: 379 }, 9)),
      (10, DijkstraNode::new(IVec2 { x: 308, y: 266 }, 10)),
      (16, DijkstraNode::new(IVec2 { x: 163, y: 205 }, 11)),
      (17, DijkstraNode::new(IVec2 { x: 149, y: 346 }, 12)),
      (18, DijkstraNode::new(IVec2 { x: 620, y: 550 }, 13)),
    ]),
    lines: HashMap::<Edge, u16>::from([
      (Edge { from: 5, to: 4 }, 2),
      (Edge { from: 18, to: 5 }, 7),
      (Edge { from: 6, to: 1 }, 6),
      (Edge { from: 8, to: 18 }, 6),
      (Edge { from: 9, to: 8 }, 8),
      (Edge { from: 4, to: 2 }, 5),
      (Edge { from: 6, to: 4 }, 9),
      (Edge { from: 4, to: 3 }, 4),
      (Edge { from: 17, to: 10 }, 8),
      (Edge { from: 10, to: 7 }, 12),
      (Edge { from: 16, to: 10 }, 7),
      (Edge { from: 8, to: 6 }, 4),
      (Edge { from: 10, to: 9 }, 11),
      (Edge { from: 17, to: 9 }, 4),
      (Edge { from: 7, to: 6 }, 5),
    ]),
    start: Some(10),
    end: Some(4),
    ..DijkstraGraph::default()
  };
  }

  pub fn insert_large_graph(&mut self)
  {
    self.clear();

    self.add_point(1, 595_f32, 640_f32);
    self.add_point(2, 864_f32, 300_f32);
    self.add_point(3, 550_f32, 369_f32);
    self.add_point(4, 280_f32, 606_f32);
    self.add_point(5, 748_f32, 127_f32);
    self.add_point(6, 177_f32, 71_f32);
    self.add_point(7, 467_f32, 84_f32);
    self.add_point(8, 260_f32, 431_f32);
    self.add_point(9, 928_f32, 642_f32);
    self.add_point(10, 466_f32, 181_f32);
    self.add_point(11, 433_f32, 27_f32);
    self.add_point(12, 667_f32, 52_f32);
    self.add_point(13, 847_f32, 75_f32);
    self.add_point(14, 734_f32, 270_f32);
    self.add_point(15, 931_f32, 233_f32);
    self.add_point(16, 904_f32, 389_f32);
    self.add_point(17, 423_f32, 467_f32);
    self.add_point(18, 445_f32, 551_f32);
    self.add_point(19, 691_f32, 559_f32);

    self.add_line(12, 13, 1);
    self.add_line(6, 8, 12);
    self.add_line(14, 3, 1);
    self.add_line(16, 9, 10);
    self.add_line(15, 9, 14);
    self.add_line(2, 19, 9);
    self.add_line(18, 19, 3);
    self.add_line(17, 18, 2);
    self.add_line(8, 4, 1);
    self.add_line(1, 9, 1);
    self.add_line(7, 5, 1);
    self.add_line(16, 3, 2);
    self.add_line(3, 8, 1);
    self.add_line(3, 17, 3);
    self.add_line(15, 16, 1);
    self.add_line(5, 14, 3);
    self.add_line(10, 3, 8);
    self.add_line(13, 2, 2);
    self.add_line(12, 5, 2);
    self.add_line(11, 12, 1);
    self.add_line(6, 11, 2);
    self.add_line(10, 5, 3);
    self.add_line(5, 2, 1);
    self.add_line(16, 17, 5);
    self.add_line(6, 7, 1);
    self.add_line(18, 1, 1);
    self.add_line(6, 10, 2);
    self.add_line(2, 3, 1);
    self.add_line(19, 9, 4);
    self.add_line(17, 4, 2);
    self.add_line(13, 15, 1);
    self.add_line(4, 1, 1);
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
