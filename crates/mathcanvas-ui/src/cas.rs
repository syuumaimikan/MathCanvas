use egui::{TextEdit, Ui};
use mathcanvas_cas::display::format_expression;
use mathcanvas_cas::simplifier::Simplifier;
use mathcanvas_core::parser::Parser;

pub struct CASView {
    expression_input: String,
    history: Vec<(String, String)>, // (Original, Simplified)
}

impl Default for CASView {
    fn default() -> Self {
        Self {
            expression_input: String::new(),
            history: Vec::new(),
        }
    }
}

impl CASView {
    pub fn ui(&mut self, ui: &mut Ui) {
        ui.heading("Computer Algebra System");
        ui.separator();

        egui::ScrollArea::vertical()
            .max_height(300.0)
            .show(ui, |ui| {
                for (expr, res) in &self.history {
                    ui.label(format!("Simplify: {} -> {}", expr, res));
                }
            });

        ui.separator();

        let response = ui.add(
            TextEdit::singleline(&mut self.expression_input)
                .hint_text("Enter expression e.g., x + x + 0")
                .desired_width(f32::INFINITY),
        );

        if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
            self.simplify();
            response.request_focus();
        }

        if ui.button("Simplify").clicked() {
            self.simplify();
        }
    }

    fn simplify(&mut self) {
        if self.expression_input.trim().is_empty() {
            return;
        }

        let input = self.expression_input.clone();
        match Parser::parse(&input) {
            Ok(ast) => {
                let simplified = Simplifier::simplify(&ast);
                let result_str = format_expression(&simplified);
                self.history.push((input, result_str));
            }
            Err(e) => {
                self.history.push((input, format!("Error: {}", e)));
            }
        }
        self.expression_input.clear();
    }
}
