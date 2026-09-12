use crate::layout::Layout;
use egui::{Context, FontDefinitions};

pub struct MathCanvasApp {
    layout: Layout,
}

impl Default for MathCanvasApp {
    fn default() -> Self {
        Self {
            layout: Layout::new(),
        }
    }
}

impl MathCanvasApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        setup_fonts(&cc.egui_ctx);
        Self::default()
    }
}

impl eframe::App for MathCanvasApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.layout.ui(ctx);
    }
}

fn setup_fonts(ctx: &Context) {
    let mut fonts = FontDefinitions::default();

    fonts.font_data.insert(
        "MaterialIcons".to_owned(),
        egui::FontData::from_static(include_bytes!("../assets/MaterialIcons-Regular.ttf")),
    );

    if let Some(vec) = fonts.families.get_mut(&egui::FontFamily::Proportional) {
        vec.push("MaterialIcons".to_owned());
    }

    if let Some(vec) = fonts.families.get_mut(&egui::FontFamily::Monospace) {
        vec.push("MaterialIcons".to_owned());
    }

    // We should ideally load a proper Japanese font from assets.
    // For now, let's rely on system fallback if possible or just use the default.
    // eframe/egui standard fonts don't include full CJK.
    // A future step is loading a bundled NotoSansJP or similar.

    ctx.set_fonts(fonts);
}
