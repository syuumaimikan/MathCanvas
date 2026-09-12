use egui::{vec2, Color32, Pos2, Rect, Stroke, Ui};

pub struct CanvasView {
    strokes: Vec<Vec<Pos2>>,
    current_stroke: Vec<Pos2>,
}

impl Default for CanvasView {
    fn default() -> Self {
        Self {
            strokes: Vec::new(),
            current_stroke: Vec::new(),
        }
    }
}

impl CanvasView {
    pub fn ui(&mut self, ui: &mut Ui) {
        ui.heading("Handwriting Canvas");
        ui.horizontal(|ui| {
            if ui.button("Clear").clicked() {
                self.strokes.clear();
                self.current_stroke.clear();
            }
            if ui.button("Mock OCR (Recognize)").clicked() {
                // Mock OCR functionality
                if !self.strokes.is_empty() {
                    ui.ctx().memory_mut(|mem| {
                        // Normally we would parse strokes to text.
                    });
                }
            }
        });
        ui.separator();

        let (response, painter) = ui.allocate_painter(ui.available_size(), egui::Sense::drag());
        let rect = response.rect;

        painter.rect_filled(rect, 0.0, Color32::from_rgb(255, 255, 240)); // Light yellow canvas

        // Draw grid
        let grid_step = 40.0;
        let mut x = rect.left();
        while x < rect.right() {
            painter.line_segment(
                [Pos2::new(x, rect.top()), Pos2::new(x, rect.bottom())],
                Stroke::new(1.0_f32, Color32::from_rgb(230, 230, 230)),
            );
            x += grid_step;
        }
        let mut y = rect.top();
        while y < rect.bottom() {
            painter.line_segment(
                [Pos2::new(rect.left(), y), Pos2::new(rect.right(), y)],
                Stroke::new(1.0_f32, Color32::from_rgb(230, 230, 230)),
            );
            y += grid_step;
        }

        // Handle input
        if response.dragged() {
            if let Some(pos) = response.interact_pointer_pos() {
                self.current_stroke.push(pos);
            }
        } else if response.drag_released() {
            if !self.current_stroke.is_empty() {
                self.strokes.push(self.current_stroke.clone());
                self.current_stroke.clear();
            }
        }

        // Draw saved strokes
        for stroke in &self.strokes {
            if stroke.len() > 1 {
                painter.add(egui::Shape::line(
                    stroke.clone(),
                    Stroke::new(3.0_f32, Color32::BLACK),
                ));
            }
        }

        // Draw current stroke
        if self.current_stroke.len() > 1 {
            painter.add(egui::Shape::line(
                self.current_stroke.clone(),
                Stroke::new(3.0_f32, Color32::BLACK),
            ));
        }
    }
}
