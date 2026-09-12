use crate::calculator::CalculatorView;
use crate::cas::CASView;
use crate::graph2d::Graph2DView;
use crate::notebook::NotebookView;
use egui::{Align, CentralPanel, Color32, Context, Layout as EguiLayout, RichText, SidePanel};

#[derive(PartialEq)]
pub enum ViewMode {
    Home,
    Notebook,
    Handwriting,
    Calculator,
    CAS,
    Graph2D,
    Graph3D,
    Statistics,
    Matrix,
    Simulation,
    History,
    Settings,
}

pub struct Layout {
    pub current_view: ViewMode,
    calculator: CalculatorView,
    notebook: NotebookView,
    graph2d: Graph2DView,
    cas: CASView,
}

impl Layout {
    pub fn new() -> Self {
        Self {
            current_view: ViewMode::Home,
            calculator: CalculatorView::default(),
            notebook: NotebookView::default(),
            graph2d: Graph2DView::default(),
            cas: CASView::default(),
        }
    }

    pub fn ui(&mut self, ctx: &Context) {
        SidePanel::left("navigation_panel")
            .resizable(false)
            .default_width(200.0)
            .show(ctx, |ui| {
                ui.with_layout(EguiLayout::top_down_justified(Align::Min), |ui| {
                    ui.add_space(10.0);
                    ui.heading(
                        RichText::new("MathCanvas")
                            .strong()
                            .color(Color32::from_rgb(100, 150, 255)),
                    );
                    ui.add_space(20.0);

                    if ui
                        .selectable_label(self.current_view == ViewMode::Home, "🏠 Home")
                        .clicked()
                    {
                        self.current_view = ViewMode::Home;
                    }
                    if ui
                        .selectable_label(self.current_view == ViewMode::Notebook, "📓 Notebooks")
                        .clicked()
                    {
                        self.current_view = ViewMode::Notebook;
                    }
                    if ui
                        .selectable_label(
                            self.current_view == ViewMode::Calculator,
                            "🔢 Calculator",
                        )
                        .clicked()
                    {
                        self.current_view = ViewMode::Calculator;
                    }
                    if ui
                        .selectable_label(self.current_view == ViewMode::CAS, "⚡ CAS")
                        .clicked()
                    {
                        self.current_view = ViewMode::CAS;
                    }
                    if ui
                        .selectable_label(self.current_view == ViewMode::Graph2D, "📈 Graph")
                        .clicked()
                    {
                        self.current_view = ViewMode::Graph2D;
                    }
                    if ui
                        .selectable_label(
                            self.current_view == ViewMode::Statistics,
                            "📊 Statistics",
                        )
                        .clicked()
                    {
                        self.current_view = ViewMode::Statistics;
                    }
                    ui.add_space(20.0);
                    if ui
                        .selectable_label(self.current_view == ViewMode::History, "⏳ History")
                        .clicked()
                    {
                        self.current_view = ViewMode::History;
                    }
                    if ui
                        .selectable_label(self.current_view == ViewMode::Settings, "⚙ Settings")
                        .clicked()
                    {
                        self.current_view = ViewMode::Settings;
                    }
                });
            });

        CentralPanel::default().show(ctx, |ui| {
            match self.current_view {
                ViewMode::Home => {
                    ui.heading("おかえりなさい"); // Testing Japanese text
                    ui.add_space(20.0);
                    if ui.button("＋ 新しいノート").clicked() {}
                    if ui.button("✎ 手書き計算").clicked() {}
                    if ui.button("ƒ(x) グラフ").clicked() {}
                    if ui.button("Σ 計算").clicked() {}
                }
                ViewMode::Notebook => {
                    self.notebook.ui(ui);
                }
                ViewMode::Calculator => {
                    self.calculator.ui(ui);
                }
                ViewMode::CAS => {
                    self.cas.ui(ui);
                }
                ViewMode::Graph2D => {
                    self.graph2d.ui(ui);
                }
                ViewMode::Statistics => {
                    ui.heading("Statistics");
                }
                ViewMode::History => {
                    ui.heading("History");
                }
                ViewMode::Settings => {
                    ui.heading("Settings");
                }
                _ => {
                    ui.heading("Work in progress");
                }
            }
        });
    }
}
