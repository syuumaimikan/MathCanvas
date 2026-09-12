#[cfg(target_os = "android")]
use eframe::android_activity;

#[cfg(target_os = "android")]
#[no_mangle]
pub extern "C" fn android_main(app: android_activity::AndroidApp) {
    use mathcanvas_ui::MathCanvasApp;
    
    // Initialize android logger
    std::env::set_var("RUST_BACKTRACE", "1");
    // android_logger::init_once(android_logger::Config::default().with_min_level(log::Level::Trace));

    let options = eframe::NativeOptions {
        android_app: Some(app),
        ..Default::default()
    };

    let _ = eframe::run_native(
        "MathCanvas",
        options,
        Box::new(|_cc| Ok(Box::new(MathCanvasApp::default()))),
    );
}
