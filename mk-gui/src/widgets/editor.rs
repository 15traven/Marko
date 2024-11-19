use mk_core::eframe::egui::{
    self, Context, Ui, 
    TextEdit, TextStyle
};
use mk_core::Highlighter;

pub struct Editor {
    pub code: String,
    highlighter: Highlighter
}

impl PartialEq for Editor {
    fn eq(&self, other: &Self) -> bool {
        &self.code == &other.code
    }
}

impl Default for Editor {
    fn default() -> Self {
        Editor {
            code: "# Marko".to_string(),
            highlighter: Default::default()
        }
    }
}

impl Editor {
    pub fn ui(&mut self, ui: &mut Ui) {
        let Self {
            code, highlighter
        } = self;

        let mut layouter = |ui: &Ui, mark: &str, wrap_width: f32| {
            let mut layout_job = highlighter.highlight(ui.style(), mark);
            layout_job.wrap.max_width = wrap_width;
            ui.fonts(|f| f.layout_job(layout_job))
        };

        ui.add(
            egui::TextEdit::multiline(code)
                .frame(false)
                .code_editor()
                .desired_width(f32::INFINITY)
                .layouter(&mut layouter)
        );
    }
}
