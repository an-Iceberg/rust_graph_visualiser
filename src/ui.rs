use crate::{graph::DijkstraGraph, utils::Mode};
use egui_macroquad::{
  egui::{epaint::Shadow, Align2, Grid, Rounding, Slider, Vec2, Visuals, Window},
  ui,
};
use macroquad::time::get_fps;

pub(crate) fn paint_ui(mode: &mut Mode, graph: &mut DijkstraGraph, mut bg_color: &mut [f32; 3]) {
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
          { ui.selectable_value(mode, Mode::Move, "Move");
            ui.selectable_value(mode, Mode::Line, "Line");
            ui.selectable_value(mode, Mode::Point, "Point");
            ui.selectable_value(mode, Mode::Path, "Path");
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
              graph.insert_small_graph_a();
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
              ui.color_edit_button_rgb(&mut bg_color);
              ui.end_row();
            });

          ui.separator();

          ui.horizontal(|ui| {
            ui.label("v1.0.0");
            ui.separator();
            ui.label(format!("FPS:{}", get_fps()));
          });
        });
  });
}
