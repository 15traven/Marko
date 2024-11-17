use eframe::egui::{
    self, Response, Context, Ui,
};
use super::{UIAction, UIStateVariant};

pub struct MainUI {
    action: Option<UIAction>
}

impl Default for MainUI {
    fn default() -> Self {
        MainUI {
            action: None
        }
    }
}

impl UIStateVariant for MainUI {
    fn update_panel(&mut self, ctx: &Context) -> UIAction {
        egui::CentralPanel::default()
            .show(ctx, |ui| {
                ui.add(|ui: &mut Ui| self.ui(ui))
            });

        self.action.take().unwrap_or(UIAction::Nothing)
    }
}

impl MainUI {
    fn ui(&mut self, ui: &mut Ui) -> Response {
        ui.vertical_centered_justified(|ui| {
            ui.label("Hello");
        }).response
    }
}
