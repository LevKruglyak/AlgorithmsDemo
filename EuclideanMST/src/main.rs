mod plane;

use eframe::egui;

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native("Euclidean MST Demo", native_options, Box::new(|cc| Box::new(EuclideanMST::new(cc))));
}

#[derive(Default)]
struct EuclideanMST {}

impl EuclideanMST {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        Self::default()
    }
}

impl eframe::App for EuclideanMST {
   fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
       egui::CentralPanel::default().show(ctx, |ui| {
           ui.heading("Hello World!");
       });
   }
}
