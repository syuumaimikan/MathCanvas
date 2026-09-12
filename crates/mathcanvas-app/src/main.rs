use eframe::egui;
use mathcanvas_ui::MathCanvasApp;
use tokio::runtime::Runtime;

fn main() -> Result<(), eframe::Error> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Create a Tokio runtime for async tasks (CAS, etc.)
    let rt = Runtime::new().expect("Unable to create Runtime");
    let _enter = rt.enter(); // Set as default runtime for the main thread

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1024.0, 768.0])
            .with_min_inner_size([800.0, 600.0])
            .with_title("MathCanvas"),
        ..Default::default()
    };

    eframe::run_native(
        "MathCanvas",
        options,
        Box::new(|cc| Ok(Box::new(MathCanvasApp::new(cc)))),
    )
}
