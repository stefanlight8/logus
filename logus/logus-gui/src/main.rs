mod app;

use app::Logus;

fn main() -> Result<(), eframe::Error> {
    let app = Logus {};
    let native_options = eframe::NativeOptions::default();

    eframe::run_native(
        "Logus",
        native_options,
        Box::new(|cc| Ok(Box::<Logus>::new(app))),
    )
}
