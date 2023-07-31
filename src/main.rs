mod graph;
mod utils;

use egui_macroquad::{
  draw,
  egui::{epaint::Shadow, Align2, Grid, Rounding, Slider, Vec2, Visuals, Window},
  ui,
};
use graph::*;
use macroquad::prelude::*;
use std::ops::Div;

fn window_configuration() -> Conf {
  return Conf {
    window_title: "Graph Visualiser".to_string(),
    window_width: 1290,
    window_height: 720,
    fullscreen: false,
    window_resizable: false,
    ..Conf::default()
  };
}

// FIX: path from 6 to 18 is wrong, there exist a much shorter one (point 10 seems to cause that somehow)
#[macroquad::main(window_configuration)]
async fn main() {
  let mut graph = Graph::new();
  let ui_width = 200;
  let mut mode = Mode::Move;

  let mut bg_colour = [0.25, 0., 0.5];

  loop {
    clear_background(Color::from_vec(Vec4::new(bg_colour[0], bg_colour[1], bg_colour[2], 1.)));

    // Delete or backspace clears the graph of all points and lines
    if is_key_pressed(KeyCode::Backspace) || is_key_pressed(KeyCode::Delete) {
      graph.clear();
    }

    // --- INPUT ---
    if utils::is_point_in_rectangle(
      mouse_position().0,
      mouse_position().1,
      graph.get_radius() as f32,
      graph.get_radius() as f32,
      screen_width() - (ui_width as f32 + (2.0 * graph.get_radius() as f32)),
      screen_height() - (2.0 * graph.get_radius() as f32),
    ) {
      handle_mouse_input(&mode, &mut graph);
    }

    // --- GUI ---
    // TODO: style the GUI
    ui(|egui_context| {
      // Disabling all shadows
      egui_context.set_visuals(Visuals {
        window_shadow: Shadow::NONE,
        window_rounding: Rounding {
          nw: 10.,
          ne: 0.,
          sw: 10.,
          se: 0.,
        },
        ..Default::default()
      });

      // egui ❤ macroquad
      Window::new("Rust Graph Visualiser")
        .anchor(Align2::RIGHT_TOP, Vec2::new(0., 10.))
        .constrain(true)
        .collapsible(false)
        .movable(false)
        .resizable(false)
        .fixed_size(Vec2::new(200., 0.))
        .show(egui_context, |ui|
        { ui.label("Select a mode:");
          ui.horizontal(|ui|
          { ui.selectable_value(&mut mode, Mode::Move, "Move");
            ui.selectable_value(&mut mode, Mode::Line, "Line");
            ui.selectable_value(&mut mode, Mode::Point, "Point");
            ui.selectable_value(&mut mode, Mode::Path, "Path");
          });

          ui.separator();

          // The newlines are a hack to make all text fill up the same amount of vertical space
          match (&mode, graph.selected_point_id)
          { (Mode::Move, _) => ui.label("• Left click on a point to select it.\n• Hold left click to move it around."),
            (Mode::Line, None) => ui.label("• Left click on a point to select it."),
            (Mode::Line, Some(_)) => ui.label("• Left click on another point to create a new line.\n• Right click on another point to delete an existing line."),
            (Mode::Point, _) => ui.label("• Left click somewhere to create a point.\n• Right click on a point to delete it."),
            (Mode::Path, _) => ui.label("• Left click on a point to set the start.\n• Right click on a point to set the end.")
          };

          match &mode
          { Mode::Line =>
            {
              ui.separator();
              ui.label("Line length:");
              ui.add(Slider::new(&mut graph.line_length, 1..=255).logarithmic(true));
            }
            Mode::Path =>
            { ui.separator();
              ui.add_enabled_ui(graph.start.is_some() && graph.end.is_some(), |ui| {
                  if ui.button("Find shortest path").clicked() {
                    graph.find_shortest_path();
                  }
              });
              ui.horizontal(|ui|
              { ui.label("Pick the color of the path:");
                ui.color_edit_button_rgb(&mut graph.path_color);
              });
            }
            _ => ()
          }

          ui.separator();

          ui.add_space(match (&mode, graph.selected_point_id) {
            (Mode::Move, _) => 215.,
            (Mode::Line, None) => 182.,
            (Mode::Line, Some(_)) => 140.,
            (Mode::Point, _) => 201.,
            (Mode::Path, _) => 136.
          });

          ui.separator();

          ui.label("Add in a pre-made graph:");
          ui.horizontal(|ui| {
            if ui.button("Small").clicked() {
              graph.insert_small_graph();
            }
            if ui.button("Medium").clicked() {
              graph.insert_medium_graph();
            }
            if ui.button("Large").clicked() {
              graph.insert_large_graph();
            }
            if ui.button("Clear").clicked() {
              graph.clear();
            }
          });

          ui.separator();

          ui.horizontal(|ui| {
            ui.label("Angle:");
            ui.add_enabled_ui(false, |ui| {
              ui.drag_angle(&mut graph.angle);
            });
          });

          ui.horizontal(|ui| {
            ui.add(Slider::new(&mut graph.angle, 0.261..=0.785));
            if ui.button("Reset").clicked() {
              graph.angle = 0.436;
            }
          });

          ui.label("Wing size:");
          ui.horizontal(|ui| {
            ui.add(Slider::new(&mut graph.arrow_head_length, 1.0..=60.0));
            if ui.button("Reset").clicked() {
              graph.arrow_head_length = 20.0;
            }
          });

          ui.label("Base point:");
          ui.horizontal(|ui| {
            ui.add(Slider::new(&mut graph.base_point, 1.0..=50.0));
            if ui.button("Reset").clicked() {
              graph.base_point = 15.;
            }
          });

          ui.separator();

          ui.label("Radius:");
          ui.horizontal(|ui| {
            ui.add(Slider::new(&mut graph.radius, 7..=20));
            if ui.button("Reset").clicked() {
              graph.radius = 13;
            }
          });

          ui.label("Path thickness:");
          ui.horizontal(|ui| {
            ui.add(Slider::new(&mut graph.path_thickness, 1.5..=5.0));
            if ui.button("Reset").clicked() {
              graph.path_thickness = 2.0;
            }
          });

          ui.separator();

          Grid::new("colours")
            .num_columns(2)
            .striped(false)
            .show(ui, |ui|
            { ui.label("Point colour:");
              ui.color_edit_button_rgb(&mut graph.point_color);
              ui.end_row();

              ui.label("Line colour:");
              ui.color_edit_button_rgb(&mut graph.line_color);
              ui.end_row();

              ui.label("Background colour:");
              ui.color_edit_button_rgb(&mut bg_colour);
              ui.end_row();
            });

          ui.separator();

          ui.horizontal(|ui| {
            ui.label("v.0.2.1");
            ui.separator();
            ui.label(format!("FPS:{}", get_fps()));
          });
        });
    });

    // ! dbg
    if is_key_pressed(KeyCode::P) {
      println!("Graph data:");
      graph.print_graph_data();
      println!("Path data:");
      graph.print_path();
    }

    graph.paint_graph();

    draw();

    next_frame().await;
  }
}

fn handle_mouse_input(mode: &Mode, graph: &mut Graph) {
  match (
    mode,
    is_mouse_button_pressed(MouseButton::Left),
    is_mouse_button_down(MouseButton::Left),
    is_mouse_button_released(MouseButton::Left),
    is_mouse_button_pressed(MouseButton::Right),
    graph.get_hovered_point_id(),
    graph.selected_point_id,
  ) {
    // --- MOVE ---

    // Select a point to be moved around
    (Mode::Move, true, _, _, false, Some(hovered_point_id), _) => {
      graph.selected_point_id = Some(hovered_point_id);
    },

    // Move a point around
    (Mode::Move, _, true, _, false, _, Some(selected_point_id)) => {
      graph.set_point_coordinates(
        selected_point_id,
        IVec2 {
          x: mouse_position().0 as i32,
          y: mouse_position().1 as i32,
        },
      );
    },

    // Releasing the selected point
    (Mode::Move, _, _, true, false, _, _) => {
      graph.selected_point_id = None;
    },

    // --- POINT ---

    // Create a point
    (Mode::Point, true, _, _, false, None, None) => {
      graph.add_point(IVec2 {
        x: mouse_position().0 as i32,
        y: mouse_position().1 as i32,
      });
    },

    // Remove a point
    (Mode::Point, false, _, _, true, Some(hovered_point_id), _) => {
      graph.remove_point(hovered_point_id);
    },

    // --- LINE ---

    // Select a point to draw a line from
    (Mode::Line, true, _, _, false, Some(hovered_point_id), None) => {
      graph.selected_point_id = Some(hovered_point_id);
    },

    // Unset the selected point if no other point is clicked on
    (Mode::Line, true, _, _, _, None, Some(_)) | (Mode::Line, _, _, _, true, None, Some(_)) => {
      graph.selected_point_id = None;
    },

    // Select a point to draw the line to
    (Mode::Line, true, _, _, false, Some(hovered_point_id), Some(selected_point_id)) => {
      graph.add_line(selected_point_id, hovered_point_id);
      graph.selected_point_id = None;
    },

    // Deletes the selected line
    (Mode::Line, false, _, _, true, Some(hovered_point_id), Some(selected_point_id)) => {
      graph.remove_line(selected_point_id, hovered_point_id);
      graph.selected_point_id = None;
    },

    // --- PATH ---

    // Select a start point with left click
    (Mode::Path, true, _, _, false, Some(hovered_point_id), None) => {
      graph.start = Some(hovered_point_id);
      graph.clear_path();
    },

    // Unsetting the start point
    (Mode::Path, true, _, _, false, None, None) => {
      graph.start = None;
      graph.clear_path();
    },

    // Select an end point with right click
    (Mode::Path, false, _, _, true, Some(hovered_point_id), None) => {
      graph.end = Some(hovered_point_id);
      graph.clear_path();
    },

    // Unsetting the end point
    (Mode::Path, false, _, _, true, None, None) => {
      graph.end = None;
      graph.clear_path();
    },

    (_, _, _, _, _, _, _) => (),
  }
}

pub fn draw_pill(x: f32, y: f32, width: f32, height: f32, color: Color) {
  draw_rectangle(x, y, width, height, color);
  draw_circle(x, y + height.div(2.0), height.div(2.0), color);
  draw_circle(x + width, y + height.div(2.0), height.div(2.0), color);
}

#[derive(PartialEq, Eq)]
enum Mode {
  Move,
  Point,
  Line,
  Path,
}
