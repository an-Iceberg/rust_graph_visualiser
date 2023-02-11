#[cfg(test)]

#[path ="../src/utils.rs"]
mod utils;

#[test]
pub fn circle_tests()
{
  // TODO: more tests
  assert!(utils::is_point_in_circle(240.0, 78.0, 233.0, 73.0, 13.0));
  assert!(!utils::is_point_in_circle(752.0, 251.0, 725.0, 270.0, 13.0));
}

#[test]
pub fn rectangle_tests()
{
  // TODO: more tests
  todo!();
}
