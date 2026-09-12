use egui::{vec2, Color32, Rect, Stroke, Ui};
use mathcanvas_core::document::{BlockContent, Document};

pub struct NotebookView {
    pub document: Document,
}

impl Default for NotebookView {
    fn default() -> Self {
        Self {
            document: Document::new("Untitled".into()),
        }
    }
}

impl NotebookView {
    pub fn ui(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.heading(&self.document.metadata.title);
            if ui.button("Save (mock)").clicked() {
                // TODO: Save to JSON
            }
        });
        ui.separator();

        let (response, painter) =
            ui.allocate_painter(ui.available_size(), egui::Sense::click_and_drag());
        let rect = response.rect;

        // Draw notebook background
        painter.rect_filled(rect, 0.0, Color32::from_rgb(250, 250, 250));

        // Draw blocks
        if let Some(page) = self.document.pages.first() {
            for block in &page.blocks {
                let block_rect = Rect::from_min_size(
                    rect.min + vec2(block.position.0, block.position.1),
                    vec2(block.size.0, block.size.1),
                );

                painter.rect_stroke(block_rect, 2.0, Stroke::new(1.0_f32, Color32::GRAY));

                match &block.content {
                    BlockContent::Text(t) => {
                        painter.text(
                            block_rect.min + vec2(5.0, 5.0),
                            egui::Align2::LEFT_TOP,
                            t,
                            egui::FontId::proportional(14.0),
                            Color32::BLACK,
                        );
                    }
                    BlockContent::MathExpression(expr) => {
                        painter.text(
                            block_rect.min + vec2(5.0, 5.0),
                            egui::Align2::LEFT_TOP,
                            expr,
                            egui::FontId::monospace(14.0),
                            Color32::BLUE,
                        );
                    }
                    _ => {}
                }
            }
        }
    }
}
