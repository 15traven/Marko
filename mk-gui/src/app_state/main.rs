use mk_core::eframe::egui::{
    self, Response, Context, Ui,
    ScrollArea
};
use super::super::widgets::Editor;
use super::{UIAction, UIStateVariant};

pub struct MainUI {
    editor: Editor,
    show_source: bool,
    show_rendered: bool,
    action: Option<UIAction>
}

impl Default for MainUI {
    fn default() -> Self {
        MainUI {
            editor: Default::default(),
            show_source: true,
            show_rendered: true,
            action: None
        }
    }
}

impl UIStateVariant for MainUI {
    fn update_panel(&mut self, ctx: &Context) -> UIAction {
        egui::CentralPanel::default()
            .show(ctx, |ui| {
                self.ui(ui);
            });

        self.action.take().unwrap_or(UIAction::Nothing)
    }
}

impl MainUI {
    fn ui(&mut self, ui: &mut Ui) {
        if self.show_source && self.show_rendered {
            ui.columns(2, |columns| { 
                ScrollArea::vertical()
                    .id_salt("source")
                    .show(&mut columns[0], |ui| self.editor.ui(ui));
            });
        } else if self.show_source { 
            ScrollArea::vertical()
                .id_salt("source")
                .show(ui, |ui| self.editor.ui(ui));
        }
    }
}
