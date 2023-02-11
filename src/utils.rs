pub fn is_point_in_circle(point_x: f32, point_y: f32, circle_x: f32, circle_y: f32, circle_radius: f32) -> bool
{
  return (circle_x - point_x).powf(2.0) + (circle_y - point_y).powf(2.0) <= circle_radius.powf(2.0);
}

pub fn is_point_in_rectangle(point_x: f32, point_y: f32, rectangle_x: f32, rectangle_y: f32, rectangle_width: f32, rectangle_height: f32) -> bool
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
