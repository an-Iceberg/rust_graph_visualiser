pub(crate) fn is_point_in_circle(point_x: f32, point_y: f32, circle_x: f32, circle_y: f32, circle_radius: f32) -> bool
{
  return (circle_x - point_x).powf(2.0) + (circle_y - point_y).powf(2.0) <= circle_radius.powf(2.0);
}

pub(crate) fn is_point_in_rectangle(point_x: f32, point_y: f32, rectangle_x: f32, rectangle_y: f32, rectangle_width: f32, rectangle_height: f32) -> bool
{
  if
    point_x < rectangle_x ||
    point_y < rectangle_y ||
    point_x > (rectangle_x + rectangle_width) ||
    point_y > (rectangle_y + rectangle_height)
  {
    return false;
  }

  return true;
}

#[cfg(test)]
mod tests
{
  use super::*;

  #[test]
  pub fn circle_tests()
  {
    assert!(is_point_in_circle(240.0, 78.0, 233.0, 73.0, 13.0));
    assert!(!is_point_in_circle(752.0, 251.0, 725.0, 270.0, 13.0));
    assert!(!is_point_in_circle(81.0, 24.0, 40.0, 40.0, 20.0));
    assert!(is_point_in_circle(25.0, 29.0, 40.0, 40.0, 20.0));
    assert!(!is_point_in_circle(189.0, 176.0, 143.0, 136.0, 57.0));
    assert!(is_point_in_circle(192.0, 87.0, 143.0, 136.0, 75.0));
    assert!(!is_point_in_circle(177.0, 243.0, 153.0, 250.0, 4.0));
    assert!(is_point_in_circle(201.0, 251.0, 200.0, 250.0, 4.0));
    assert!(!is_point_in_circle(1020.0, 315.0, 873.0, 310.0, 48.0));
    assert!(is_point_in_circle(606.0, 411.0, 615.0, 404.0, 22.0));
  }

  #[test]
  pub fn rectangle_tests()
  {
    assert!(!is_point_in_rectangle(747.0, 313.0, 600.0, 357.0, 100.0, 100.0));
    assert!(!is_point_in_rectangle(433.0, 226.0, 448.0, 170.0, 38.0, 100.0));
    assert!(is_point_in_rectangle(382.0, 274.0, 213.0, 165.0, 324.0, 238.0));
    assert!(!is_point_in_rectangle(664.0, 180.0, 563.0, 211.0, 62.0, 67.0));
    assert!(!is_point_in_rectangle(601.0, 182.0, 587.0, 389.0, 31.0, 92.0));
    assert!(!is_point_in_rectangle(821.0, 85.0, 509.0, 53.0, 276.0, 67.0));
    assert!(!is_point_in_rectangle(229.0, 328.0, 77.0, 139.0, 143.0, 181.0));
    assert!(!is_point_in_rectangle(450.0, 618.0, 383.0, 384.0, 191.0, 134.0));
    assert!(!is_point_in_rectangle(729.0, 685.0, 813.0, 418.0, 96.0, 248.0));
    assert!(!is_point_in_rectangle(305.0, 617.0, 355.0, 588.0, 96.0, 86.0));
    assert!(is_point_in_rectangle(637.0, 297.0, 419.0, 68.0, 248.0, 248.0));
    assert!(is_point_in_rectangle(803.0, 429.0, 781.0, 404.0, 210.0, 134.0));
    assert!(is_point_in_rectangle(135.0, 88.0, 104.0, 55.0, 162.0, 305.0));
  }
}
