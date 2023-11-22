use super::DijkstraGraph;
use super::DijkstraNode;
use crate::graph::Edge;
use macroquad::prelude::IVec2;
use rand::*;
use std::{
  collections::{BTreeMap, HashMap},
  ops::Mul,
};

const RADIUS: f32 = 13_f32;

fn random_x(radius: f32) -> f32
{ return thread_rng().gen_range(radius..(1290_f32 - 200_f32 - radius)); }

fn random_y(radius: f32) -> f32
{ return thread_rng().gen_range(radius..(720_f32 - radius)); }

fn generate_random_points_graph(amount_of_points: u8) -> DijkstraGraph
{
  let mut graph = DijkstraGraph::new();
  for _ in 1..=amount_of_points
  { graph.append_point(random_x(RADIUS), random_y(RADIUS)); }
  return graph;
}

#[test]
fn add_some_points()
{
  // Creating a graph
  let mut is_graph = DijkstraGraph::new();
  for _i in 1..=3 {
    is_graph.add_point(vec2_random_coordinates(
      is_graph.radius.into(),
    ));
  }

  // Creating the values it should have
  let mut should_ids: Vec<u8> = Vec::new();
  for id in 1..=3 { should_ids.push(id); }

  // Comparing the two for equality
  for (is_id, should_id) in is_graph.points.keys().zip(should_ids.iter())
  { assert_eq!(*is_id, *should_id); }
}

#[test]
fn add_many_points()
{
  // Creating the graph
  let mut is_graph = DijkstraGraph::new();
  for _i in 1..=50 {
    is_graph.add_point(vec2_random_coordinates(is_graph.get_radius().into()))
  }

  // Creating the data that should be in the graph
  let mut should_ids: Vec<u8> = Vec::new();
  for id in 1..=50 { should_ids.push(id); }

  // Comparing for equality
  for (is_id, should_id) in is_graph.points.keys().zip(should_ids.iter())
  { assert_eq!(*is_id, *should_id); }
}

#[test]
fn max_amount_of_points()
{
  // Creating graph and "adding" 1_000 points to it
  let mut is_graph = DijkstraGraph::new();
  for _i in 0..1_000 {
    is_graph.add_point(vec2_random_coordinates(
      is_graph.radius.into(),
    ));
  }

  // The graph should still only have 100 points
  assert_eq!(is_graph.points.len(),100);
}

#[test]
fn remove_points()
{
  // Creating a graph
  let mut is_graph = generate_random_points_graph(10);

  // Removing every second point
  for id in 1..=5
  { is_graph.remove_point(id * 2); }

  // Creating the ids the resulting graph should have
  let mut should_ids: Vec<u8> = Vec::new();
  for id in 1..=5
  { should_ids.push(id.mul(2 as u8) - 1); }

  // Comparing for equality
  for (is_id, should_id) in is_graph.points.keys().zip(should_ids.iter())
  { assert_eq!(*is_id, *should_id); }
}

#[test]
fn shortest_path_small()
{
  // First case
  {
    let mut graph = DijkstraGraph::new();
    graph.insert_small_graph_a();
    assert!(graph.find_shortest_path(3, 1));

    // Shortest paths are either [3, 4, 5, 1] or [3, 6, 8, 1]
    let should_path_1: Vec<u8> = vec![3, 4, 5, 1];
    let should_path_2: Vec<u8> = vec![3, 6, 8, 1];

    match graph.get_path()
    {
      Some(path) =>
      {
        path.iter()
          .zip(should_path_1.iter())
          .zip(should_path_2.iter())
          .for_each(|((path_id, should_id_1), should_id_2)|
          { assert!(*path_id == *should_id_1 || *path_id == *should_id_2); });
      },
      None => panic!("A path should have been found"),
    }
  }

  // Second case
  {
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

    let should_path = vec![7, 8, 1, 2, 3, 9];

    assert!(graph.find_shortest_path(7, 9));

    match graph.get_path()
    {
      Some(path) =>
      {
        path.iter()
          .zip(should_path.iter())
          .for_each(|(path_id, should_id)|
          { assert_eq!(*path_id, *should_id); });
      },
      None => panic!("A path should have been found"),
    }
  }
}

#[test]
fn shortest_path_medium()
{
  let mut graph = DijkstraGraph::new();
  // start: Some(4),

  // First end
  {
    let should_path_1: Vec<u8> = vec![4, 10, 9, 5, 1];
    let should_path_2: Vec<u8> = vec![4, 11, 8, 5, 1];
    let should_path_3: Vec<u8> = vec![4, 12, 9, 5, 1];

    graph.find_shortest_path(4, 1);

    match graph.path
    {
      Some(ref path) =>
      {
        path.iter()
          .zip(should_path_1.iter())
          .zip(should_path_2.iter())
          .zip(should_path_3.iter())
          .for_each(|(((path_id, should_id_1), should_id_2), should_id_3)|
          { assert!(*path_id == *should_id_1 || *path_id == *should_id_2 || *path_id == *should_id_3); });
      },
      None => panic!("A path should have been found"),
    }
  }

  // Second end
  {
    let should_path_1: Vec<u8> = vec![4, 10, 9, 5, 2];
    let should_path_2: Vec<u8> = vec![4, 11, 8, 5, 2];
    let should_path_3: Vec<u8> = vec![4, 11, 8, 2];
    let should_path_4: Vec<u8> = vec![4, 12, 8, 5, 2];
    let should_path_5: Vec<u8> = vec![4, 12, 8, 2];

    graph.find_shortest_path(4, 2);

    match graph.path {
      Some(ref path) => {
        path
          .iter()
          .zip(should_path_1.iter())
          .zip(should_path_2.iter())
          .zip(should_path_3.iter())
          .zip(should_path_4.iter())
          .zip(should_path_5.iter())
          .for_each(|(((((path_id, should_id_1), should_id_2), should_id_3), should_id_4), should_id_5)| {
            assert!(
              *path_id == *should_id_1
                || *path_id == *should_id_2
                || *path_id == *should_id_3
                || *path_id == *should_id_4
                || *path_id == *should_id_5
            );
          });
      },
      None => panic!("A path should have been found"),
    }
  }

  // Third end
  {
    let should_path_1: Vec<u8> = vec![4, 11, 8, 6, 3];
    let should_path_2: Vec<u8> = vec![4, 12, 8, 6, 3];

    graph.find_shortest_path(4, 3);

    match graph.path
    {
      Some(ref path) =>
      {
        path.iter()
          .zip(should_path_1.iter())
          .zip(should_path_2.iter())
          .for_each(|((path_id, should_id_1), should_id_2)|
          { assert!(*path_id == *should_id_1 || *path_id == *should_id_2); });
      },
      None => panic!("A path should have been found"),
    }
  }
}

#[test]
fn shortest_path_large()
{
  let mut graph = DijkstraGraph::new();

  let should_path = vec![6, 7, 5, 2, 3, 8, 4, 1, 9];

  graph.find_shortest_path(6, 9);

  match graph.path
  {
    Some(path) =>
    {
      path.iter()
        .zip(should_path.iter())
        .for_each(|(path_id, should_id)|
        { assert_eq!(*path_id, *should_id); });
    },
    None => panic!("A path should have been found"),
  }
}

#[test]
fn start_and_end_are_within_graph() {
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

  let should_path = vec![10, 7, 6, 4];

  graph.find_shortest_path(10, 4);

  match graph.path
  {
    Some(path) =>
    {
      path.iter()
        .zip(should_path.iter())
        .for_each(|(path_id, should_id)|
        { assert_eq!(*path_id, *should_id); });
    },
    None => panic!("A path should have been found"),
  }
}

#[test]
fn no_possible_path()
{
  let mut graph = DijkstraGraph::new();
  graph.insert_small_graph_a();

  assert!(!graph.find_shortest_path(1, 3));
  assert!(graph.get_path().is_none());
}

#[test]
fn disconnected_graph()
{
  let mut graph = DijkstraGraph {
    points: BTreeMap::<u8, DijkstraNode>::from([
      (1, DijkstraNode::new(IVec2 { x: 888, y: 135 }, 1)),
      (2, DijkstraNode::new(IVec2 { x: 595, y: 138 }, 2)),
      (3, DijkstraNode::new(IVec2 { x: 267, y: 120 }, 3)),
      (4, DijkstraNode::new(IVec2 { x: 230, y: 347 }, 4)),
      (5, DijkstraNode::new(IVec2 { x: 553, y: 379 }, 5)),
      (6, DijkstraNode::new(IVec2 { x: 905, y: 390 }, 6)),
      (7, DijkstraNode::new(IVec2 { x: 895, y: 649 }, 7)),
      (8, DijkstraNode::new(IVec2 { x: 479, y: 634 }, 8)),
      (9, DijkstraNode::new(IVec2 { x: 187, y: 607 }, 9)),
    ]),
    lines: HashMap::<Edge, u16>::from([
      (Edge { from: 9, to: 8 }, 20),
      (Edge { from: 3, to: 2 }, 20),
      (Edge { from: 1, to: 6 }, 20),
      (Edge { from: 6, to: 7 }, 20),
      (Edge { from: 3, to: 4 }, 20),
      (Edge { from: 8, to: 7 }, 20),
      (Edge { from: 3, to: 5 }, 20),
    ]),
    start: Some(3),
    end: Some(1),
    ..DijkstraGraph::default()
  };
  graph.find_shortest_path(3, 7);

  assert!(graph.path.is_none());
}

#[test]
fn cyclical_valid_path()
{
  let mut graph = DijkstraGraph {
    points: BTreeMap::<u8, DijkstraNode>::from([
      (1, DijkstraNode::new(IVec2 { x: 899, y: 490 }, 1)),
      (2, DijkstraNode::new(IVec2 { x: 941, y: 618 }, 2)),
      (3, DijkstraNode::new(IVec2 { x: 710, y: 621 }, 3)),
      (4, DijkstraNode::new(IVec2 { x: 777, y: 390 }, 4)),
      (5, DijkstraNode::new(IVec2 { x: 698, y: 200 }, 5)),
      (6, DijkstraNode::new(IVec2 { x: 497, y: 185 }, 6)),
      (7, DijkstraNode::new(IVec2 { x: 379, y: 367 }, 7)),
      (8, DijkstraNode::new(IVec2 { x: 556, y: 541 }, 8)),
      (9, DijkstraNode::new(IVec2 { x: 403, y: 574 }, 9)),
      (10, DijkstraNode::new(IVec2 { x: 207, y: 434 }, 10)),
      (11, DijkstraNode::new(IVec2 { x: 238, y: 257 }, 11)),
      (12, DijkstraNode::new(IVec2 { x: 554, y: 41 }, 12)),
    ]),
    lines: HashMap::<Edge, u16>::from([
      (Edge { from: 7, to: 11 }, 1),
      (Edge { from: 6, to: 12 }, 1),
      (Edge { from: 7, to: 6 }, 1),
      (Edge { from: 5, to: 4 }, 1),
      (Edge { from: 6, to: 5 }, 1),
      (Edge { from: 8, to: 7 }, 1),
      (Edge { from: 4, to: 8 }, 1),
      (Edge { from: 4, to: 1 }, 1),
      (Edge { from: 8, to: 3 }, 1),
      (Edge { from: 7, to: 10 }, 1),
      (Edge { from: 1, to: 2 }, 1),
      (Edge { from: 8, to: 9 }, 1),
    ]),
    ..DijkstraGraph::default()
  };

  let should_path = vec![4, 8, 7, 6, 5];

  graph.find_shortest_path(4, 5);

  match graph.path
  {
    Some(path) =>
    {
      path.iter()
        .zip(should_path.iter())
        .for_each(|(path_id, should_id)|
        { assert_eq!(*path_id, *should_id); });
    },
    None => panic!("A path should have been found"),
  }
}
