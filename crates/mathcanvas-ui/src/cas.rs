use egui::{TextEdit, Ui};
use mathcanvas_cas::display::format_expression;
use mathcanvas_cas::simplifier::Simplifier;
use mathcanvas_core::parser::Parser;
use mathcanvas_core::ast::Expression;

pub struct CASView {
    expression_input: String,
    history: Vec<(String, Expression, String)>, // (Original, AST, SimplifiedStr)
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
                for (expr_str, ast, res_str) in &self.history {
                    ui.group(|ui| {
                        ui.label(format!("Simplify: {} -> {}", expr_str, res_str));
                        ui.horizontal(|ui| {
                            if ui.button("📋 LaTeX").clicked() {
                                println!("LaTeX: {}", ast.to_latex());
                                // in a real app, copy to clipboard
                            }
                            if ui.button("🐍 Python").clicked() {
                                println!("Python: {}", ast.to_python());
                            }
                        });
                    });
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
                self.history.push((input, simplified, result_str));
            }
            Err(e) => {
                // For error, we just push a dummy AST
                self.history.push((input, mathcanvas_core::ast::Expression::Number(0.0), format!("Error: {}", e)));
            }
        }
        self.expression_input.clear();
    }
}
