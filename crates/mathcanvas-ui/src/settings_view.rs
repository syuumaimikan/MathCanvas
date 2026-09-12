use egui::{ComboBox, Ui};
use mathcanvas_core::settings::{
    AngleUnit, AppSettings, CalculationMode, ComplexMode, CoordinateSystem, PenInputMode,
    ReactiveMode,
};

pub struct SettingsView {
    pub settings: AppSettings,
}

impl Default for SettingsView {
    fn default() -> Self {
        Self {
            settings: AppSettings::default(),
        }
    }
}

impl SettingsView {
    pub fn ui(&mut self, ui: &mut Ui) {
        ui.heading("Settings");
        ui.separator();

        egui::ScrollArea::vertical().show(ui, |ui| {
            // Math Engine Settings
            ui.collapsing("Math Engine", |ui| {
                ui.horizontal(|ui| {
                    ui.label("Angle Unit:");
                    ComboBox::from_id_salt("angle_unit")
                        .selected_text(format!("{:?}", self.settings.math.angle_unit))
                        .show_ui(ui, |ui| {
                            ui.selectable_value(
                                &mut self.settings.math.angle_unit,
                                AngleUnit::Radian,
                                "Radian",
                            );
                            ui.selectable_value(
                                &mut self.settings.math.angle_unit,
                                AngleUnit::Degree,
                                "Degree",
                            );
                            ui.selectable_value(
                                &mut self.settings.math.angle_unit,
                                AngleUnit::Gradian,
                                "Gradian",
                            );
                        });
                });

                ui.horizontal(|ui| {
                    ui.label("Calculation Mode:");
                    ComboBox::from_id_salt("calc_mode")
                        .selected_text(format!("{:?}", self.settings.math.calculation_mode))
                        .show_ui(ui, |ui| {
                            ui.selectable_value(
                                &mut self.settings.math.calculation_mode,
                                CalculationMode::ExactAlgebraic,
                                "Exact (Algebraic)",
                            );
                            ui.selectable_value(
                                &mut self.settings.math.calculation_mode,
                                CalculationMode::NumericApproximate,
                                "Numeric (Fast)",
                            );
                        });
                });
            });

            // Pen & Stroke Settings
            ui.collapsing("Pen & Input", |ui| {
                ui.horizontal(|ui| {
                    ui.label("Pen Mode:");
                    ComboBox::from_id_salt("pen_mode")
                        .selected_text(format!("{:?}", self.settings.pen.input_mode))
                        .show_ui(ui, |ui| {
                            ui.selectable_value(
                                &mut self.settings.pen.input_mode,
                                PenInputMode::StylusOnly,
                                "Stylus Only",
                            );
                            ui.selectable_value(
                                &mut self.settings.pen.input_mode,
                                PenInputMode::Hybrid,
                                "Hybrid (Finger + Stylus)",
                            );
                        });
                });
                ui.checkbox(
                    &mut self.settings.pen.auto_shape_recognition,
                    "Auto Shape Recognition",
                );
            });

            // Graph Settings
            ui.collapsing("Graph & Rendering", |ui| {
                ui.horizontal(|ui| {
                    ui.label("Coordinate System:");
                    ComboBox::from_id_salt("coord_sys")
                        .selected_text(format!("{:?}", self.settings.graph.coordinate_system))
                        .show_ui(ui, |ui| {
                            ui.selectable_value(
                                &mut self.settings.graph.coordinate_system,
                                CoordinateSystem::Cartesian,
                                "Cartesian",
                            );
                            ui.selectable_value(
                                &mut self.settings.graph.coordinate_system,
                                CoordinateSystem::Polar,
                                "Polar",
                            );
                            ui.selectable_value(
                                &mut self.settings.graph.coordinate_system,
                                CoordinateSystem::Logarithmic,
                                "Logarithmic",
                            );
                        });
                });
                ui.checkbox(
                    &mut self.settings.graph.hardware_acceleration,
                    "Hardware Acceleration (wgpu)",
                );
                ui.checkbox(
                    &mut self.settings.graph.adaptive_sampling,
                    "Adaptive Sampling",
                );
            });

            // Notebook Settings
            ui.collapsing("Notebook", |ui| {
                ui.horizontal(|ui| {
                    ui.label("Reactive DAG Mode:");
                    ComboBox::from_id_salt("reactive_mode")
                        .selected_text(format!("{:?}", self.settings.notebook.reactive_dag))
                        .show_ui(ui, |ui| {
                            ui.selectable_value(
                                &mut self.settings.notebook.reactive_dag,
                                ReactiveMode::Immediate,
                                "Immediate",
                            );
                            ui.selectable_value(
                                &mut self.settings.notebook.reactive_dag,
                                ReactiveMode::ManualBatch,
                                "Manual (Batch)",
                            );
                        });
                });
                ui.add(
                    egui::Slider::new(&mut self.settings.notebook.undo_stack_limit, 10..=500)
                        .text("Undo Stack Limit"),
                );
            });
        });
    }
}
