use macroquad::prelude::*;

pub fn window_config() -> Conf
{
  return Conf
  {
    window_title: "Graph Visualiser".to_string(),
    window_width: 1290,
    window_height: 720,
    fullscreen: false,
    window_resizable: false,
    ..Default::default()
  };
}
