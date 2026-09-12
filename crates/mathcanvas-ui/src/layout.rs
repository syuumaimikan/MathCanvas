use crate::calculator::CalculatorView;
use crate::canvas::CanvasView;
use crate::cas::CASView;
use crate::graph2d::Graph2DView;
use crate::graph3d::Graph3DView;
use crate::notebook::NotebookView;
use crate::settings_view::SettingsView;
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
    canvas: CanvasView,
    notebook: NotebookView,
    graph2d: Graph2DView,
    graph3d: Graph3DView,
    cas: CASView,
    settings: SettingsView,
}

impl Layout {
    pub fn new() -> Self {
        Self {
            current_view: ViewMode::Home,
            calculator: CalculatorView::default(),
            canvas: CanvasView::default(),
            notebook: NotebookView::default(),
            graph2d: Graph2DView::default(),
            graph3d: Graph3DView::default(),
            cas: CASView::default(),
            settings: SettingsView::default(),
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
                        .selectable_label(
                            self.current_view == ViewMode::Home,
                            format!(
                                "{} Home",
                                material_icons::icon_to_char(material_icons::Icon::Home)
                            ),
                        )
                        .clicked()
                    {
                        self.current_view = ViewMode::Home;
                    }
                    if ui
                        .selectable_label(
                            self.current_view == ViewMode::Notebook,
                            format!(
                                "{} Notebooks",
                                material_icons::icon_to_char(material_icons::Icon::Book)
                            ),
                        )
                        .clicked()
                    {
                        self.current_view = ViewMode::Notebook;
                    }
                    if ui
                        .selectable_label(
                            self.current_view == ViewMode::Handwriting,
                            format!(
                                "{} Canvas",
                                material_icons::icon_to_char(material_icons::Icon::Draw)
                            ),
                        )
                        .clicked()
                    {
                        self.current_view = ViewMode::Handwriting;
                    }
                    if ui
                        .selectable_label(
                            self.current_view == ViewMode::Calculator,
                            format!(
                                "{} Calculator",
                                material_icons::icon_to_char(material_icons::Icon::Calculate)
                            ),
                        )
                        .clicked()
                    {
                        self.current_view = ViewMode::Calculator;
                    }
                    if ui
                        .selectable_label(
                            self.current_view == ViewMode::CAS,
                            format!(
                                "{} CAS",
                                material_icons::icon_to_char(material_icons::Icon::ElectricBolt)
                            ),
                        )
                        .clicked()
                    {
                        self.current_view = ViewMode::CAS;
                    }
                    if ui
                        .selectable_label(
                            self.current_view == ViewMode::Graph2D,
                            format!(
                                "{} 2D Graph",
                                material_icons::icon_to_char(material_icons::Icon::ShowChart)
                            ),
                        )
                        .clicked()
                    {
                        self.current_view = ViewMode::Graph2D;
                    }
                    if ui
                        .selectable_label(
                            self.current_view == ViewMode::Graph3D,
                            format!(
                                "{} 3D Graph",
                                material_icons::icon_to_char(material_icons::Icon::Category)
                            ),
                        )
                        .clicked()
                    {
                        self.current_view = ViewMode::Graph3D;
                    }
                    if ui
                        .selectable_label(
                            self.current_view == ViewMode::Statistics,
                            format!(
                                "{} Statistics",
                                material_icons::icon_to_char(material_icons::Icon::BarChart)
                            ),
                        )
                        .clicked()
                    {
                        self.current_view = ViewMode::Statistics;
                    }
                    ui.add_space(20.0);
                    if ui
                        .selectable_label(
                            self.current_view == ViewMode::History,
                            format!(
                                "{} History",
                                material_icons::icon_to_char(material_icons::Icon::History)
                            ),
                        )
                        .clicked()
                    {
                        self.current_view = ViewMode::History;
                    }
                    if ui
                        .selectable_label(
                            self.current_view == ViewMode::Settings,
                            format!(
                                "{} Settings",
                                material_icons::icon_to_char(material_icons::Icon::Settings)
                            ),
                        )
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
                    if ui
                        .button(format!(
                            "{} 新しいノート",
                            material_icons::icon_to_char(material_icons::Icon::Add)
                        ))
                        .clicked()
                    {}
                    if ui
                        .button(format!(
                            "{} 手書き計算",
                            material_icons::icon_to_char(material_icons::Icon::Draw)
                        ))
                        .clicked()
                    {}
                    if ui
                        .button(format!(
                            "{} グラフ",
                            material_icons::icon_to_char(material_icons::Icon::ShowChart)
                        ))
                        .clicked()
                    {}
                    if ui
                        .button(format!(
                            "{} 計算",
                            material_icons::icon_to_char(material_icons::Icon::Calculate)
                        ))
                        .clicked()
                    {}
                }
                ViewMode::Notebook => {
                    self.notebook.ui(ui);
                }
                ViewMode::Handwriting => {
                    self.canvas.ui(ui);
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
                ViewMode::Graph3D => {
                    self.graph3d.ui(ui);
                }
                ViewMode::Statistics => {
                    ui.heading("Statistics");
                }
                ViewMode::History => {
                    ui.heading("History");
                }
                ViewMode::Settings => {
                    self.settings.ui(ui);
                }
                _ => {
                    ui.heading("Work in progress");
                }
            }
        });
    }
}
