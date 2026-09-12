use egui::{Color32, Pos2, Stroke, Ui};
use mathcanvas_graph::sampler3d::Sampler3D;

pub struct Graph3DView {
    grid_cache: Vec<Vec<(f32, f32, f32)>>,
}

impl Default for Graph3DView {
    fn default() -> Self {
        Self {
            grid_cache: Sampler3D::sample_surface(30, (-10.0, 10.0), (-10.0, 10.0)),
        }
    }
}

impl Graph3DView {
    pub fn ui(&mut self, ui: &mut Ui) {
        ui.heading("3D Graph (Isometric)");
        ui.separator();

        let (response, painter) = ui.allocate_painter(ui.available_size(), egui::Sense::hover());
        let rect = response.rect;
        painter.rect_filled(rect, 0.0, Color32::from_rgb(30, 30, 30)); // Dark background for 3D

        let center = (rect.center().x, rect.center().y);
        let scale = 15.0;

        let projected2d = Sampler3D::project_isometric(&self.grid_cache, scale, center);

        // Draw horizontal lines
        for row in &projected2d {
            let mut line_points = Vec::new();
            for &(px, py) in row {
                line_points.push(Pos2::new(px, py));
            }
            if line_points.len() > 1 {
                painter.add(egui::Shape::line(
                    line_points,
                    Stroke::new(1.0_f32, Color32::from_rgb(100, 200, 255)),
                ));
            }
        }

        // Draw vertical lines
        let size = projected2d.len();
        if size > 0 {
            for j in 0..size {
                let mut line_points = Vec::new();
                for i in 0..size {
                    line_points.push(Pos2::new(projected2d[i][j].0, projected2d[i][j].1));
                }
                if line_points.len() > 1 {
                    painter.add(egui::Shape::line(
                        line_points,
                        Stroke::new(1.0_f32, Color32::from_rgb(255, 100, 200)),
                    ));
                }
            }
        }
    }
}
