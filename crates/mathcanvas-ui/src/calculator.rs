use egui::{TextEdit, Ui};
use mathcanvas_core::evaluator::{evaluate, Environment};
use mathcanvas_core::parser::Parser;

pub struct CalculatorView {
    expression_input: String,
    history: Vec<(String, String)>, // (Expression, Result)
}

impl Default for CalculatorView {
    fn default() -> Self {
        Self {
            expression_input: String::new(),
            history: Vec::new(),
        }
    }
}

impl CalculatorView {
    pub fn ui(&mut self, ui: &mut Ui) {
        ui.heading("Calculator");
        ui.separator();

        egui::ScrollArea::vertical()
            .max_height(300.0)
            .show(ui, |ui| {
                for (expr, res) in &self.history {
                    ui.label(format!("{} = {}", expr, res));
                }
            });

        ui.separator();

        let response = ui.add(
            TextEdit::singleline(&mut self.expression_input)
                .hint_text("Enter expression e.g., 2 + 2 * sin(pi/2)")
                .desired_width(f32::INFINITY),
        );

        if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
            self.calculate();
            response.request_focus();
        }

        if ui.button("Calculate").clicked() {
            self.calculate();
        }
    }

    fn calculate(&mut self) {
        if self.expression_input.trim().is_empty() {
            return;
        }

        let input = self.expression_input.clone();
        match Parser::parse(&input) {
            Ok(ast) => {
                let env = Environment::default();
                let result = evaluate(&ast, &env);
                self.history.push((input, result.to_string()));
            }
            Err(e) => {
                self.history.push((input, format!("Error: {}", e)));
            }
        }
        self.expression_input.clear();
    }
}
