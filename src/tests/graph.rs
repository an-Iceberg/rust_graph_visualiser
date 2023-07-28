use super::Graph;
use super::Node;
use crate::graph::Line;
use macroquad::prelude::IVec2;
use rand::*;
use std::{
  collections::{BTreeMap, HashMap},
  ops::Mul,
};

fn ivec2_random_coordinates(radius: i32) -> IVec2 {
  return IVec2 {
    x: thread_rng().gen_range(radius..(1290 - 200 - radius)),
    y: thread_rng().gen_range(radius..(720 - radius)),
  };
}

fn graph(amount_of_points: u8) -> Graph {
  let mut graph = Graph::new();
  for _i in 1..=amount_of_points {
    graph.add_point(ivec2_random_coordinates(
      graph
        .radius
        .into(),
    ));
  }

  return graph;
}

#[test]
fn add_some_points() {
  // Creating a graph
  let mut is_graph = Graph::new();
  for _i in 1..=3 {
    is_graph.add_point(ivec2_random_coordinates(
      is_graph
        .radius
        .into(),
    ));
  }

  // Creating the values it should have
  let mut should_ids: Vec<u8> = Vec::new();
  for id in 1..=3 {
    should_ids.push(id);
  }

  // Comparing the two for equality
  for (is_id, should_id) in is_graph
    .points
    .keys()
    .zip(should_ids.iter())
  {
    assert_eq!(*is_id, *should_id);
  }
}

#[test]
fn add_many_points() {
  // Creating the graph
  let mut is_graph = Graph::new();
  for _i in 1..=50 {
    is_graph.add_point(ivec2_random_coordinates(
      is_graph
        .get_radius()
        .into(),
    ))
  }

  // Creating the data that should be in the graph
  let mut should_ids: Vec<u8> = Vec::new();
  for id in 1..=50 {
    should_ids.push(id);
  }

  // Comparing for equality
  for (is_id, should_id) in is_graph
    .points
    .keys()
    .zip(should_ids.iter())
  {
    assert_eq!(*is_id, *should_id);
  }
}

#[test]
fn max_amount_of_points() {
  // Creating graph and "adding" 1_000 points to it
  let mut is_graph = Graph::new();
  for _i in 0..1_000 {
    is_graph.add_point(ivec2_random_coordinates(
      is_graph
        .radius
        .into(),
    ));
  }

  // The graph should still only have 100 points
  assert_eq!(
    is_graph
      .points
      .len(),
    100
  );
}

#[test]
fn remove_points() {
  // Creating a graph
  let mut is_graph = graph(10);

  // Removing every second point
  for id in 1..=5 {
    is_graph.remove_point(id * 2);
  }

  // Creating the ids the resulting graph should have
  let mut should_ids: Vec<u8> = Vec::new();
  for id in 1..=5 {
    should_ids.push(id.mul(2 as u8) - 1);
  }

  // Comparing for equality
  for (is_id, should_id) in is_graph
    .points
    .keys()
    .zip(should_ids.iter())
  {
    assert_eq!(*is_id, *should_id);
  }
}

#[test]
fn shortest_path_small() {
  // First case
  {
    let mut graph = Graph {
      points: BTreeMap::<u8, Node>::from([
        (1, Node::new(IVec2 { x: 942, y: 355 }, 0)),
        (2, Node::new(IVec2 { x: 720, y: 208 }, 0)),
        (3, Node::new(IVec2 { x: 198, y: 342 }, 0)),
        (4, Node::new(IVec2 { x: 463, y: 507 }, 0)),
        (5, Node::new(IVec2 { x: 735, y: 513 }, 0)),
        (6, Node::new(IVec2 { x: 458, y: 346 }, 0)),
        (7, Node::new(IVec2 { x: 468, y: 202 }, 0)),
        (8, Node::new(IVec2 { x: 721, y: 360 }, 0)),
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

    match graph.path {
      Some(path) => {
        path
          .iter()
          .zip(should_path_1.iter())
          .zip(should_path_2.iter())
          .for_each(|((path_id, should_id_1), should_id_2)| {
            assert!(*path_id == *should_id_1 || *path_id == *should_id_2);
          });
      },
      None => panic!("A path should have been found"),
    }
  }

  // Second case
  {
    let mut graph = Graph {
      points: BTreeMap::<u8, Node>::from([
        (1, Node::new(IVec2 { x: 783, y: 102 }, 1)),
        (2, Node::new(IVec2 { x: 412, y: 295 }, 2)),
        (3, Node::new(IVec2 { x: 680, y: 308 }, 3)),
        (4, Node::new(IVec2 { x: 509, y: 459 }, 4)),
        (5, Node::new(IVec2 { x: 330, y: 603 }, 5)),
        (6, Node::new(IVec2 { x: 160, y: 442 }, 6)),
        (7, Node::new(IVec2 { x: 174, y: 196 }, 7)),
        (8, Node::new(IVec2 { x: 411, y: 78 }, 8)),
        (9, Node::new(IVec2 { x: 1003, y: 239 }, 9)),
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

    match graph.path {
      Some(path) => {
        path
          .iter()
          .zip(should_path.iter())
          .for_each(|(path_id, should_id)| {
            assert_eq!(*path_id, *should_id);
          });
      },
      None => panic!("A path should have been found"),
    }
  }
}

#[test]
fn shortest_path_medium() {
  let mut graph = Graph {
    points: BTreeMap::<u8, Node>::from([
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

    match graph.path {
      Some(ref path) => {
        path
          .iter()
          .zip(should_path_1.iter())
          .zip(should_path_2.iter())
          .zip(should_path_3.iter())
          .for_each(|(((path_id, should_id_1), should_id_2), should_id_3)| {
            assert!(*path_id == *should_id_1 || *path_id == *should_id_2 || *path_id == *should_id_3);
          });
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

    graph.end = Some(2);
    graph.find_shortest_path();

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

    graph.end = Some(3);
    graph.find_shortest_path();

    match graph.path {
      Some(ref path) => {
        path
          .iter()
          .zip(should_path_1.iter())
          .zip(should_path_2.iter())
          .for_each(|((path_id, should_id_1), should_id_2)| {
            assert!(*path_id == *should_id_1 || *path_id == *should_id_2);
          });
      },
      None => panic!("A path should have been found"),
    }
  }
}

#[test]
fn shortest_path_large() {
  let mut graph = Graph {
    points: BTreeMap::<u8, Node>::from([
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

  match graph.path {
    Some(path) => {
      path
        .iter()
        .zip(should_path.iter())
        .for_each(|(path_id, should_id)| {
          assert_eq!(*path_id, *should_id);
        });
    },
    None => panic!("A path should have been found"),
  }
}

#[test]
fn start_and_end_are_within_graph() {
  let mut graph = Graph {
    points: BTreeMap::<u8, Node>::from([
      (1, Node::new(IVec2 { x: 970, y: 108 }, 1)),
      (2, Node::new(IVec2 { x: 991, y: 340 }, 2)),
      (3, Node::new(IVec2 { x: 1023, y: 580 }, 3)),
      (4, Node::new(IVec2 { x: 509, y: 459 }, 4)),
      (5, Node::new(IVec2 { x: 750, y: 537 }, 5)),
      (6, Node::new(IVec2 { x: 747, y: 262 }, 6)),
      (7, Node::new(IVec2 { x: 535, y: 237 }, 7)),
      (8, Node::new(IVec2 { x: 497, y: 433 }, 8)),
      (9, Node::new(IVec2 { x: 352, y: 379 }, 9)),
      (10, Node::new(IVec2 { x: 308, y: 266 }, 10)),
      (16, Node::new(IVec2 { x: 163, y: 205 }, 11)),
      (17, Node::new(IVec2 { x: 149, y: 346 }, 12)),
      (18, Node::new(IVec2 { x: 620, y: 550 }, 13)),
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

  match graph.path {
    Some(path) => {
      path
        .iter()
        .zip(should_path.iter())
        .for_each(|(path_id, should_id)| {
          assert_eq!(*path_id, *should_id);
        });
    },
    None => panic!("A path should have been found"),
  }
}

#[test]
fn no_possible_path() {
  let mut graph = Graph::new();
  graph.insert_small_graph();
  graph.start = Some(1);
  graph.end = Some(3);

  graph.find_shortest_path();

  assert!(graph
    .path
    .is_none());
}

#[test]
fn disconnected_graph() {
  let mut graph = Graph {
    points: BTreeMap::<u8, Node>::from([
      (1, Node::new(IVec2 { x: 888, y: 135 }, 1)),
      (2, Node::new(IVec2 { x: 595, y: 138 }, 2)),
      (3, Node::new(IVec2 { x: 267, y: 120 }, 3)),
      (4, Node::new(IVec2 { x: 230, y: 347 }, 4)),
      (5, Node::new(IVec2 { x: 553, y: 379 }, 5)),
      (6, Node::new(IVec2 { x: 905, y: 390 }, 6)),
      (7, Node::new(IVec2 { x: 895, y: 649 }, 7)),
      (8, Node::new(IVec2 { x: 479, y: 634 }, 8)),
      (9, Node::new(IVec2 { x: 187, y: 607 }, 9)),
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

  assert!(graph
    .path
    .is_none());
}

#[test]
fn cyclical_valid_path() {
  let mut graph = Graph {
    points: BTreeMap::<u8, Node>::from([
      (1, Node::new(IVec2 { x: 899, y: 490 }, 1)),
      (2, Node::new(IVec2 { x: 941, y: 618 }, 2)),
      (3, Node::new(IVec2 { x: 710, y: 621 }, 3)),
      (4, Node::new(IVec2 { x: 777, y: 390 }, 4)),
      (5, Node::new(IVec2 { x: 698, y: 200 }, 5)),
      (6, Node::new(IVec2 { x: 497, y: 185 }, 6)),
      (7, Node::new(IVec2 { x: 379, y: 367 }, 7)),
      (8, Node::new(IVec2 { x: 556, y: 541 }, 8)),
      (9, Node::new(IVec2 { x: 403, y: 574 }, 9)),
      (10, Node::new(IVec2 { x: 207, y: 434 }, 10)),
      (11, Node::new(IVec2 { x: 238, y: 257 }, 11)),
      (12, Node::new(IVec2 { x: 554, y: 41 }, 12)),
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

  match graph.path {
    Some(path) => {
      path
        .iter()
        .zip(should_path.iter())
        .for_each(|(path_id, should_id)| {
          assert_eq!(*path_id, *should_id);
        });
    },
    None => panic!("A path should have been found"),
  }
}
