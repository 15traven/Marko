pub use mk_core::eframe::egui::{self};

mod main;

pub use main::MainUI;

pub trait UIStateVariant {
    fn update_panel(&mut self, ctx: &egui::Context) -> UIAction;
}

#[derive(Clone, Debug)]
pub enum UIAction {
    Nothing
}

pub enum UIState {
    Main(MainUI)
}

impl Default for UIState {
    fn default() -> Self {
        UIState::Main(MainUI::default())
    }
}

impl UIState {
    pub fn update(&mut self, ctx: &egui::Context) {
        let response = match self {
            UIState::Main(panel) => panel.update_panel(ctx)
        };

        match response {
            UIAction::Nothing => {}
        }
    }
}
