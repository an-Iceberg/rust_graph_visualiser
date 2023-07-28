mod graph;
mod utils;

use std::ops::Div;

use graph::*;
use macroquad::{hash, prelude::*, ui::root_ui};

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

#[macroquad::main(window_configuration)]
async fn main() {
  let mut graph = Graph::new();
  let ui_width = 200;
  let mut line_length = 1.0;
  let mut mode = 0;

  setup(&mut graph);

  loop {
    clear_background(Color::from_rgba(20, 0, 40, 255));

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
      handle_mouse_input(mode, &mut graph, line_length);
    }

    // --- GUI ---
    // TODO: replace existing GUI with egui
    // TODO: extract GUI into separate component (if possible)
    // TODO: style the GUI
    root_ui().window(
      hash!(),
      Vec2 {
        x: screen_width() - ui_width as f32,
        y: 0.0,
      },
      Vec2 {
        x: ui_width as f32,
        y: screen_height(),
      },
      |ui| {
        ui.combo_box(hash!(), "Mode", &["Move", "Point", "Line", "Path"], &mut mode);

        match mode {
          // Move
          0 => {
            ui.label(None, "Left click: move a point");
          },

          // Point
          1 => {
            ui.label(None, "Left click: create point");
            ui.label(None, "Right click: delete point");
          },

          // Line
          2 => {
            ui.drag(hash!(), "Line Length", Some((1.0, 100.0)), &mut line_length);

            if graph.selected_point_id == None {
              ui.label(None, "Left click: select a point");
            } else {
              ui.label(None, "Left click: select another");
              ui.label(None, "  point to make a line");
              ui.label(None, "Right click: select another");
              ui.label(None, "  point to delete a line");
            }
          },

          // Path
          3 => {
            if graph.start != None && graph.end != None {
              if ui.button(None, "Find shortest path") {
                graph.find_shortest_path();
              }
            }

            ui.label(None, "Left click: set start");
            ui.label(None, "Right click: set end");
          },

          _ => (),
        }

        // TODO: Add user instructions

        // TODO: Add small graph button
        // TODO: Add medium graph button
        // TODO: Add large graph button

        // TODO: Adjustments for point radius, arrowhead, path thickness/color
        // TODO: FPS
      },
    );

    // ! dbg
    if is_key_pressed(KeyCode::P) {
      println!("Graph data:");
      graph.print_graph_data();
      println!("Path data:");
      graph.print_path();
    }
    // ! dbg
    if is_key_pressed(KeyCode::S) {
      graph.insert_small_graph();
    }
    // ! dbg
    if is_key_pressed(KeyCode::M) {
      graph.insert_medium_graph();
    }
    // ! dbg
    if is_key_pressed(KeyCode::L) {
      // FIX: path from 6 to 18 is wrong, there exist a much shorter one
      graph.insert_large_graph();
    }

    graph.paint_graph();

    next_frame().await;
  }
}

fn handle_mouse_input(mode: usize, graph: &mut Graph, line_length: f32) {
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
    (0, true, _, _, false, Some(hovered_point_id), _) => {
      graph.selected_point_id = Some(hovered_point_id);
    },

    // Move a point around
    (0, _, true, _, false, _, Some(selected_point_id)) => {
      graph.set_point_coordinates(
        selected_point_id,
        IVec2 {
          x: mouse_position().0 as i32,
          y: mouse_position().1 as i32,
        },
      );
    },

    // Releasing the selected point
    (0, _, _, true, false, _, _) => {
      graph.selected_point_id = None;
    },

    // --- POINT ---

    // Create a point
    (1, true, _, _, false, None, None) => {
      graph.add_point(IVec2 {
        x: mouse_position().0 as i32,
        y: mouse_position().1 as i32,
      });
    },

    // Remove a point
    (1, false, _, _, true, Some(hovered_point_id), _) => {
      graph.remove_point(hovered_point_id);
    },

    // --- LINE ---

    // Select a point to draw a line from
    (2, true, _, _, false, Some(hovered_point_id), None) => {
      graph.selected_point_id = Some(hovered_point_id);
    },

    // Unset the selected point if no other point is clicked on
    (2, true, _, _, _, None, Some(_)) | (2, _, _, _, true, None, Some(_)) => {
      graph.selected_point_id = None;
    },

    // Select a point to draw the line to
    (2, true, _, _, false, Some(hovered_point_id), Some(selected_point_id)) => {
      graph.add_line(selected_point_id, hovered_point_id, line_length as u16);
      graph.selected_point_id = None;
    },

    // Deletes the selected line
    (2, false, _, _, true, Some(hovered_point_id), Some(selected_point_id)) => {
      graph.remove_line(selected_point_id, hovered_point_id);
      graph.selected_point_id = None;
    },

    // --- PATH ---

    // Select a start point with left click
    (3, true, _, _, false, Some(hovered_point_id), None) => {
      graph.start = Some(hovered_point_id);
      graph.clear_path();
    },

    // Unsetting the start point
    (3, true, _, _, false, None, None) => {
      graph.start = None;
      graph.clear_path();
    },

    // Select an end point with right click
    (3, false, _, _, true, Some(hovered_point_id), None) => {
      graph.end = Some(hovered_point_id);
      graph.clear_path();
    },

    // Unsetting the end point
    (3, false, _, _, true, None, None) => {
      graph.end = None;
      graph.clear_path();
    },

    (_, _, _, _, _, _, _) => (),
  }
}

fn setup(graph: &mut Graph) {
  graph.add_point(IVec2 { x: 233, y: 73 });
  graph.add_point(IVec2 { x: 76, y: 234 });
  graph.add_point(IVec2 { x: 72, y: 150 });
  graph.add_point(IVec2 { x: 29, y: 495 });
  graph.add_point(IVec2 { x: 120, y: 684 });
  graph.add_point(IVec2 { x: 290, y: 521 });
  graph.add_point(IVec2 { x: 380, y: 537 });
  graph.add_point(IVec2 { x: 414, y: 127 });
  graph.add_point(IVec2 { x: 584, y: 88 });
  graph.add_point(IVec2 { x: 535, y: 514 });
  graph.add_point(IVec2 { x: 650, y: 616 });
  graph.add_point(IVec2 { x: 857, y: 529 });
  graph.add_point(IVec2 { x: 250, y: 309 });
  graph.add_point(IVec2 { x: 502, y: 343 });
  graph.add_point(IVec2 { x: 752, y: 251 });

  graph.add_line(3, 12, 15);
  graph.add_line(10, 5, 30);
  graph.add_line(9, 1, 43);
  graph.add_line(2, 14, 67);
  graph.add_line(15, 11, 93);
  graph.add_line(6, 13, 9);
  graph.add_line(13, 6, 24);
  graph.add_line(7, 14, 33);
  graph.add_line(9, 15, 47);
}

pub fn draw_pill(x: f32, y: f32, width: f32, height: f32, color: Color) {
  draw_rectangle(x, y, width, height, color);
  draw_circle(x, y + height.div(2.0), height.div(2.0), color);
  draw_circle(x + width, y + height.div(2.0), height.div(2.0), color);
}

enum Mode {
  Move,
  Point,
  Line,
  Path,
}
