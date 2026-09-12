use egui::{Color32, Pos2, Stroke, Ui};
use mathcanvas_core::parser::Parser;
use mathcanvas_graph::adaptive_sampler::AdaptiveSampler;

pub struct Graph2DView {
    expression_input: String,
    points: Vec<(f64, f64)>,
}

impl Default for Graph2DView {
    fn default() -> Self {
        Self {
            expression_input: "sin(x)".to_string(),
            points: Vec::new(),
        }
    }
}

impl Graph2DView {
    pub fn ui(&mut self, ui: &mut Ui) {
        ui.heading("2D Graph");
        ui.horizontal(|ui| {
            ui.label("y = ");
            let response = ui.add(
                egui::TextEdit::singleline(&mut self.expression_input)
                    .hint_text("Enter expression e.g., x^2"),
            );

            if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter))
                || ui.button("Graph").clicked()
            {
                self.update_graph();
            }
        });

        ui.separator();

        let (response, painter) = ui.allocate_painter(ui.available_size(), egui::Sense::hover());
        let rect = response.rect;

        // Draw background
        painter.rect_filled(rect, 0.0, Color32::from_rgb(240, 240, 240));

        // Coordinate system
        let origin = rect.center();
        let scale = 50.0; // pixels per unit

        // Draw axes
        painter.line_segment(
            [
                Pos2::new(rect.left(), origin.y),
                Pos2::new(rect.right(), origin.y),
            ],
            Stroke::new(1.0_f32, Color32::BLACK),
        );
        painter.line_segment(
            [
                Pos2::new(origin.x, rect.top()),
                Pos2::new(origin.x, rect.bottom()),
            ],
            Stroke::new(1.0_f32, Color32::BLACK),
        );

        // Draw the graph
        if !self.points.is_empty() {
            let mut line_points = Vec::new();
            for &(x, y) in &self.points {
                let px = origin.x + (x as f32) * scale;
                let py = origin.y - (y as f32) * scale;
                if rect.contains(Pos2::new(px, py)) {
                    line_points.push(Pos2::new(px, py));
                }
            }
            if line_points.len() > 1 {
                painter.add(egui::Shape::line(
                    line_points,
                    Stroke::new(2.0_f32, Color32::BLUE),
                ));
            }
        }
    }

    fn update_graph(&mut self) {
        if let Ok(ast) = Parser::parse(&self.expression_input) {
            self.points = AdaptiveSampler::sample(&ast, -10.0, 10.0, 500);
        }
    }
}
