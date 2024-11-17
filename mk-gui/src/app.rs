use eframe::{
    self, App, Frame,
    egui::{ Context }
};
use super::app_state::UIState;

pub struct MarkoApp {
    state: UIState
}

impl Default for MarkoApp {
    fn default() -> Self {
        MarkoApp {
            state: UIState::default()
        }
    }
}

impl App for MarkoApp {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        self.state.update(ctx);
    }
}
