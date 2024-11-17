use mk_gui::{
    MarkoApp,
    eframe::{self, NativeOptions}
};

fn main() -> eframe::Result {
    eframe::run_native(
        "Marko",
        NativeOptions::default(),
        Box::new(|_| {
            Ok(Box::new(MarkoApp::default()))
        })
    )
}
