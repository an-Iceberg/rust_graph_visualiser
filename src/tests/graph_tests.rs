use super::DijkstraGraph;
use rand::*;

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
  let mut graph = generate_random_points_graph(3);
  assert_eq!(graph.size(), 3);
}

#[test]
fn add_many_points()
{
  let mut graph = generate_random_points_graph(50);
  assert_eq!(graph.size(), 50);
}

#[test]
fn max_amount_of_points()
{
  // Creating graph and "adding" 1_000 points to it
  let mut graph = generate_random_points_graph(1_000);
  // The graph should still only have 100 points
  assert_eq!(graph.size(), 101);
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
    assert!(graph.find_shortest_path(2, 0));

    let should_path_1 = vec![2, 3, 4, 0];
    let should_path_2 = vec![2, 5, 7, 0];

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
    let mut graph = DijkstraGraph::new();

    graph.append_point(783_f32, 102_f32);
    graph.append_point(412_f32, 295_f32);
    graph.append_point(680_f32, 308_f32);
    graph.append_point(509_f32, 459_f32);
    graph.append_point(330_f32, 603_f32);
    graph.append_point(160_f32, 442_f32);
    graph.append_point(174_f32, 196_f32);
    graph.append_point(411_f32, 78_f32);
    graph.append_point(1003_f32, 239_f32);

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

    assert!(graph.find_shortest_path(6, 8));

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
    let should_path_1 = vec![3, 9, 8, 4, 0];
    let should_path_2 = vec![3, 10, 7, 4, 0];
    let should_path_3 = vec![3, 11, 8, 4, 0];

    graph.find_shortest_path(3, 0);

    match graph.get_path()
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
    let should_path_1 = vec![3, 9, 8, 4, 1];
    let should_path_2 = vec![3, 10, 7, 4, 1];
    let should_path_3 = vec![3, 10, 7, 1];
    let should_path_4 = vec![3, 11, 7, 4, 1];
    let should_path_5 = vec![3, 11, 7, 1];

    graph.find_shortest_path(3, 1);

    match graph.get_path()
    {
      Some(ref path) =>
      {
        path.iter()
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
    let should_path_1 = vec![3, 10, 7, 5, 2];
    let should_path_2 = vec![3, 11, 7, 5, 2];

    graph.find_shortest_path(3, 2);

    match graph.get_path()
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

// TODO: decrease all indices in add_line by 1
#[test]
fn shortest_path_large()
{
  let mut graph = DijkstraGraph::new();
  graph.insert_large_graph();

  let should_path = vec![5, 6, 4, 1, 2, 7, 3, 0, 8];

  graph.find_shortest_path(5, 8);

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

#[test]
fn start_and_end_are_within_graph()
{
  let mut graph = DijkstraGraph::new();

  graph.add_point(1, 970_f32, 108_f32);
  graph.add_point(2, 991_f32, 340_f32);
  graph.add_point(3, 1023_f32, 580_f32);
  graph.add_point(4, 509_f32, 459_f32);
  graph.add_point(5, 750_f32, 537_f32);
  graph.add_point(6, 747_f32, 262_f32);
  graph.add_point(7, 535_f32, 237_f32);
  graph.add_point(8, 497_f32, 433_f32);
  graph.add_point(9, 352_f32, 379_f32);
  graph.add_point(10, 308_f32, 266_f32);
  graph.add_point(16, 163_f32, 205_f32);
  graph.add_point(17, 149_f32, 346_f32);
  graph.add_point(18, 620_f32, 550_f32);

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

  graph.find_shortest_path(10, 4);

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

#[test]
fn no_possible_path()
{
  let mut graph = DijkstraGraph::new();
  graph.insert_small_graph();

  assert!(!graph.find_shortest_path(0, 2));
  assert!(graph.get_path().is_none());
}

#[test]
fn disconnected_graph()
{
  let mut graph = DijkstraGraph::new();

  graph.append_point(888_f32, 135_f32);
  graph.append_point(595_f32, 138_f32);
  graph.append_point(267_f32, 120_f32);
  graph.append_point(230_f32, 347_f32);
  graph.append_point(553_f32, 379_f32);
  graph.append_point(905_f32, 390_f32);
  graph.append_point(895_f32, 649_f32);
  graph.append_point(479_f32, 634_f32);
  graph.append_point(187_f32, 607_f32);

  graph.add_line(8, 7, 20);
  graph.add_line(2, 1, 20);
  graph.add_line(0, 5, 20);
  graph.add_line(5, 6, 20);
  graph.add_line(2, 3, 20);
  graph.add_line(7, 6, 20);
  graph.add_line(2, 4, 20);

  graph.find_shortest_path(2, 0);

  assert!(graph.get_path().is_none());
}

// TODO: decrease all indices in add_line by 1
#[test]
fn cyclical_valid_path()
{
  let mut graph = DijkstraGraph::new();

  graph.append_point(899_f32, 490_f32);
  graph.append_point(941_f32, 618_f32);
  graph.append_point(710_f32, 621_f32);
  graph.append_point(777_f32, 390_f32);
  graph.append_point(698_f32, 200_f32);
  graph.append_point(497_f32, 185_f32);
  graph.append_point(379_f32, 367_f32);
  graph.append_point(556_f32, 541_f32);
  graph.append_point(403_f32, 574_f32);
  graph.append_point(207_f32, 434_f32);
  graph.append_point(238_f32, 257_f32);
  graph.append_point(554_f32, 41_f32);

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

  graph.find_shortest_path(3, 4);

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
