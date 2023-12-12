use crate::{graph::DijkstraGraph, Mode};
use egui_macroquad::{
  egui::{epaint::{Shadow, self}, Align2, Grid, Rounding, Slider, Vec2, Visuals, Window, color_picker::{color_picker_color32, Alpha}, Color32},
  ui,
};
use macroquad::time::get_fps;

// TODO: make option to switch to hexagons for points
// TODO: edit colour with hex values
// TODO: make colours editable

pub(crate) fn paint_ui(
  mode: &mut Mode,
  graph: &mut DijkstraGraph,
  radius: &mut f32,
  angle: &mut f32,
  arrow_head_length: &mut f32,
  path_thickness: &mut f32,
  base_point: &mut f32,
  selected_point_id: &mut Option<usize>,
  line_length: &mut u16,
  path_color: &u32,
  line_color: &u32,
  point_color: &u32,
  bg_color: &u32,
)
{
  ui(|egui_context| {
    // Disabling all shadows
    egui_context.set_visuals(Visuals
    {
      window_shadow: Shadow::NONE,
      window_rounding: Rounding
      {
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
      {
        ui.label("Select a mode:");
        ui.horizontal(|ui|
        {
          ui.selectable_value(mode, Mode::Move, "Move");
          ui.selectable_value(mode, Mode::Line, "Line");
          ui.selectable_value(mode, Mode::Point, "Point");
          ui.selectable_value(mode, Mode::Path, "Path");
        });

        ui.separator();

        // The newlines are a hack to make all text fill up the same amount of vertical space
        match (&mode, selected_point_id.is_some())
        {
          (Mode::Move, _) => ui.label("• Left click on a point to select it.\n• Hold left click to move it around."),
          (Mode::Line, false) => ui.label("• Left click on a point to select it."),
          (Mode::Line, true) => ui.label("• Left click on another point to create a new line.\n• Right click on another point to delete an existing line."),
          (Mode::Point, _) => ui.label("• Left click somewhere to create a point.\n• Right click on a point to delete it."),
          (Mode::Path, _) => ui.label("• Left click on a point to set the start.\n• Right click on a point to set the end.")
        };

        match &mode
        {
          Mode::Line =>
          {
            ui.separator();
            ui.label("Line length:");
            ui.add(Slider::new(line_length, 1..=255).logarithmic(true));
          }
          Mode::Path =>
          {
            ui.separator();
            ui.add_enabled_ui(graph.start().is_some() && graph.end().is_some(), |ui|
            {
              if ui.button("Find shortest path").clicked()
              { graph.find_shortest_path(); }
            });
            /*
            ui.horizontal(|ui|
            {
              ui.label("Pick the color of the path:");
              ui.color_edit_button_rgb(path_color);
            });
            */
          }
          _ => ()
        }

        ui.separator();

        ui.add_space(match (&mode, selected_point_id)
        {
          (Mode::Move, _) => 215.,
          (Mode::Line, None) => 182.,
          (Mode::Line, Some(_)) => 140.,
          (Mode::Point, _) => 201.,
          (Mode::Path, _) => 136.
        });

        ui.separator();

        ui.label("Add in a pre-made graph:");
        ui.horizontal(|ui|
        {
          if ui.button("Small").clicked()
          { graph.insert_small_graph(); }
          if ui.button("Medium").clicked()
          { graph.insert_medium_graph(); }
          if ui.button("Large").clicked()
          { graph.insert_large_graph(); }
          if ui.button("Clear").clicked()
          { graph.clear(); }
        });

        ui.separator();

        ui.horizontal(|ui|
        {
          ui.label("Angle:");
          ui.add_enabled_ui(false, |ui|
          { ui.drag_angle(angle); });
        });

        ui.horizontal(|ui|
        {
          ui.add(Slider::new(angle, 0.261..=0.785));
          if ui.button("Reset").clicked() { *angle = 0.436; }
        });

        ui.label("Wing size:");
        ui.horizontal(|ui|
        {
          ui.add(Slider::new(arrow_head_length, 1.0..=60.0));
          if ui.button("Reset").clicked() { *arrow_head_length = 20.; }
        });

        ui.label("Base point:");
        ui.horizontal(|ui|
        {
          ui.add(Slider::new(base_point, 1.0..=50.0));
          if ui.button("Reset").clicked() { *base_point = 15.; }
        });

        ui.separator();

        ui.label("Radius:");
        ui.horizontal(|ui|
        {
          ui.add(Slider::new(radius, 7.0..=20.0));
          if ui.button("Reset").clicked() { *radius = 13.; }
        });

        ui.label("Path thickness:");
        ui.horizontal(|ui|
        {
          ui.add(Slider::new(path_thickness, 1.0..=5.0));
          if ui.button("Reset").clicked() { *path_thickness = 2.0; }
        });

        /*
        ui.separator();

        Grid::new("colours")
          .num_columns(2)
          .striped(false)
          .show(ui, |ui|
          {
            ui.label("Point colour:");
            ui.color_edit_button_rgb(point_color);
            ui.end_row();

            ui.label("Line colour:");
            ui.color_edit_button_rgb(line_color);
            ui.end_row();

            ui.label("Background colour:");
            ui.color_edit_button_rgb(bg_color);
            ui.end_row();
          });
        */

        ui.separator();

        ui.horizontal(|ui|
        {
          ui.label("v1.0.0");
          ui.separator();
          ui.label(format!("FPS:{}", get_fps()));
        });
      });
  });
}
