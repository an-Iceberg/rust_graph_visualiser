use super::DijkstraGraph;
use rand::*;

const RADIUS: f32 = 13.;

fn random_x(radius: f32) -> f32
{ return thread_rng().gen_range(radius..(1290. - 200. - radius)); }

fn random_y(radius: f32) -> f32
{ return thread_rng().gen_range(radius..(720. - radius)); }

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
  let graph = generate_random_points_graph(3);
  assert_eq!(graph.size(), 3);
}

#[test]
fn add_many_points()
{
  let graph = generate_random_points_graph(50);
  assert_eq!(graph.size(), 50);
}

#[test]
fn max_amount_of_points()
{
  // Creating graph and "adding" 1_000 points to it
  let graph = generate_random_points_graph(255);
  // The graph should still only have 100 points
  assert_eq!(graph.size(), 100);
}

#[test]
fn remove_points()
{
  // Creating a graph
  let mut graph = generate_random_points_graph(10);
  // Removing every second point
  for id in 0..=4
  { graph.remove_point(id * 2); }
  assert_eq!(graph.size(), 5);
}

#[test]
fn shortest_path_small()
{
  // First case
  {
    let mut graph = DijkstraGraph::new();
    graph.insert_small_graph();
    graph.set_start(2);
    graph.set_end(0);
    graph.find_shortest_path();

    let should_path_1 = vec![2, 3, 4, 0];
    let should_path_2 = vec![2, 5, 7, 0];

    match graph.get_path()
    {
      Some(path) => assert!(path == should_path_1 || path == should_path_2),
      None => panic!("A path should have been found"),
    }
  }

  // Second case
  {
    let mut graph = DijkstraGraph::new();

    graph.append_point(783., 102.);
    graph.append_point(412., 295.);
    graph.append_point(680., 308.);
    graph.append_point(509., 459.);
    graph.append_point(330., 603.);
    graph.append_point(160., 442.);
    graph.append_point(174., 196.);
    graph.append_point(411., 78.);
    graph.append_point(1003., 239.);

    graph.add_line(3, 4, 2);
    graph.add_line(2, 3, 3);
    graph.add_line(1, 5, 3);
    graph.add_line(0, 8, 7);
    graph.add_line(3, 1, 1);
    graph.add_line(8, 2, 1);
    graph.add_line(5, 1, 3);
    graph.add_line(6, 7, 2);
    graph.add_line(1, 3, 1);
    graph.add_line(1, 7, 3);
    graph.add_line(1, 6, 5);
    graph.add_line(1, 0, 1);
    graph.add_line(4, 5, 2);
    graph.add_line(0, 1, 1);
    graph.add_line(2, 8, 1);
    graph.add_line(3, 2, 3);
    graph.add_line(0, 7, 1);
    graph.add_line(7, 0, 1);
    graph.add_line(5, 6, 2);
    graph.add_line(7, 6, 2);
    graph.add_line(7, 1, 3);
    graph.add_line(1, 2, 1);
    graph.add_line(6, 1, 5);
    graph.add_line(8, 0, 7);
    graph.add_line(2, 1, 1);
    graph.add_line(4, 3, 2);
    graph.add_line(5, 4, 2);
    graph.add_line(6, 5, 2);

    let should_path = vec![6, 7, 0, 1, 2, 8];

    graph.set_start(6);
    graph.set_end(8);

    graph.find_shortest_path();

    match graph.get_path()
    {
      Some(path) => assert!(path == should_path),
      None => panic!("A path should have been found"),
    }
  }
}

#[test]
fn shortest_path_small_a()
{
  let mut graph = DijkstraGraph::new();
  graph.insert_small_graph();
  graph.set_start(2);
  graph.set_end(4);
  graph.find_shortest_path();

  let should_path = vec![2, 3, 4];

  match graph.get_path()
  {
    Some(path) => assert!(path == should_path),
    None => panic!("A path should have been found"),
  }
}

#[test]
fn find_shortest_path_multiple_calls()
{
  let mut graph = DijkstraGraph::new();
  graph.insert_small_graph();
  graph.set_start(2);
  graph.set_end(0);
  graph.find_shortest_path();
  graph.set_start(2);
  graph.set_end(4);
  graph.find_shortest_path();
  graph.find_shortest_path();
  graph.find_shortest_path();

  assert!(graph.get_path().is_some());
}

#[test]
fn shortest_path_medium()
{
  let mut graph = DijkstraGraph::new();
  graph.insert_medium_graph();

  // First end
  {
    let should_path_1 = vec![3, 9, 8, 4, 0];
    let should_path_2 = vec![3, 10, 7, 4, 0];
    let should_path_3 = vec![3, 11, 8, 4, 0];

    graph.set_start(3);
    graph.set_end(0);

    graph.find_shortest_path();

    match graph.get_path()
    {
      Some(path) => assert!(path == should_path_1 || path == should_path_2 || path == should_path_3),
      None => panic!("A path should have been found"),
    }
  }

  // Second end
  {
    let should_path_1 = vec![3, 9, 8, 4, 1];
    let should_path_2 = vec![3, 10, 7, 4, 1];
    let should_path_3 = vec![3, 10, 7, 1];
    let should_path_4 = vec![3, 11, 7, 4, 1];
    let should_path_5 = vec![3, 11, 7, 1];

    graph.set_start(3);
    graph.set_end(1);

    graph.find_shortest_path();

    match graph.get_path()
    {
      Some(path) => assert!(path == should_path_1 || path == should_path_2 || path == should_path_3 || path == should_path_4 || path == should_path_5),
      None => panic!("A path should have been found"),
    }
  }

  // Third end
  {
    let should_path_1 = vec![3, 10, 6, 5, 2];
    let should_path_2 = vec![3, 11, 6, 5, 2];

    graph.set_start(3);
    graph.set_end(2);

    graph.find_shortest_path();

    match graph.get_path()
    {
      Some(path) => assert!(path == should_path_1 || path == should_path_2),
      None => panic!("A path should have been found"),
    }
  }
}

#[test]
fn shortest_path_large_a()
{
  let mut graph = DijkstraGraph::new();
  graph.insert_large_graph();

  let should_path = vec![5, 6, 4, 1, 2, 7, 3, 0, 8];

  graph.set_start(5);
  graph.set_end(8);

  graph.find_shortest_path();

  match graph.get_path()
  {
    Some(path) => assert!(path == should_path),
    None => panic!("A path should have been found"),
  }
}

#[test]
fn shortest_path_large_b()
{
  let mut graph = DijkstraGraph::new();
  graph.insert_large_graph();

  let should_path1 = vec![6, 4, 1, 2, 16, 17, 18];
  let should_path2 = vec![6, 4, 1, 18];

  graph.set_start(6);
  graph.set_end(18);

  graph.find_shortest_path();

  match graph.get_path()
  {
    Some(path) => assert!(path == should_path1 || path == should_path2),
    None => panic!("A path should have been found"),
  }
}

#[test]
fn start_and_end_are_within_graph()
{
  let mut graph = DijkstraGraph::new();

  graph.add_point(1, 970., 108.);
  graph.add_point(2, 991., 340.);
  graph.add_point(3, 1023., 580.);
  graph.add_point(4, 509., 459.);
  graph.add_point(5, 750., 537.);
  graph.add_point(6, 747., 262.);
  graph.add_point(7, 535., 237.);
  graph.add_point(8, 497., 433.);
  graph.add_point(9, 352., 379.);
  graph.add_point(10, 308., 266.);
  graph.add_point(16, 163., 205.);
  graph.add_point(17, 149., 346.);
  graph.add_point(18, 620., 550.);

  graph.add_line(5, 4, 2);
  graph.add_line(18, 5, 7);
  graph.add_line(6, 1, 6);
  graph.add_line(8, 18, 6);
  graph.add_line(9, 8, 8);
  graph.add_line(4, 2, 5);
  graph.add_line(6, 4, 9);
  graph.add_line(4, 3, 4);
  graph.add_line(17, 10, 8);
  graph.add_line(10, 7, 12);
  graph.add_line(16, 10, 7);
  graph.add_line(8, 6, 4);
  graph.add_line(10, 9, 11);
  graph.add_line(17, 9, 4);
  graph.add_line(7, 6, 5);

  let should_path = vec![10, 7, 6, 4];

  graph.set_start(10);
  graph.set_end(4);

  graph.find_shortest_path();

  match graph.get_path()
  {
    Some(path) => assert!(path == should_path),
    None => panic!("A path should have been found"),
  }
}

#[test]
fn no_possible_path()
{
  let mut graph = DijkstraGraph::new();
  graph.insert_small_graph();

  graph.set_start(0);
  graph.set_end(2);

  graph.find_shortest_path();
  assert!(graph.get_path().is_none());
}

#[test]
fn disconnected_graph()
{
  let mut graph = DijkstraGraph::new();

  graph.append_point(888., 135.);
  graph.append_point(595., 138.);
  graph.append_point(267., 120.);
  graph.append_point(230., 347.);
  graph.append_point(553., 379.);
  graph.append_point(905., 390.);
  graph.append_point(895., 649.);
  graph.append_point(479., 634.);
  graph.append_point(187., 607.);

  graph.add_line(8, 7, 20);
  graph.add_line(2, 1, 20);
  graph.add_line(0, 5, 20);
  graph.add_line(5, 6, 20);
  graph.add_line(2, 3, 20);
  graph.add_line(7, 6, 20);
  graph.add_line(2, 4, 20);

  graph.set_start(2);
  graph.set_end(0);

  graph.find_shortest_path();

  assert!(graph.get_path().is_none());
}

#[test]
fn cyclical_valid_path()
{
  let mut graph = DijkstraGraph::new();

  graph.append_point(899., 490.);
  graph.append_point(941., 618.);
  graph.append_point(710., 621.);
  graph.append_point(777., 390.);
  graph.append_point(698., 200.);
  graph.append_point(497., 185.);
  graph.append_point(379., 367.);
  graph.append_point(556., 541.);
  graph.append_point(403., 574.);
  graph.append_point(207., 434.);
  graph.append_point(238., 257.);
  graph.append_point(554., 41.);

  graph.add_line(6, 10, 1);
  graph.add_line(5, 11, 1);
  graph.add_line(6, 5, 1);
  graph.add_line(4, 3, 1);
  graph.add_line(5, 4, 1);
  graph.add_line(7, 6, 1);
  graph.add_line(3, 7, 1);
  graph.add_line(3, 0, 1);
  graph.add_line(7, 2, 1);
  graph.add_line(6, 9, 1);
  graph.add_line(0, 1, 1);
  graph.add_line(7, 8, 1);

  let should_path = vec![3, 7, 6, 5, 4];

  graph.set_start(3);
  graph.set_end(4);

  graph.find_shortest_path();

  match graph.get_path()
  {
    Some(path) => assert!(path == should_path),
    None => panic!("A path should have been found"),
  }
}
