use mk_core::eframe::egui::{
    self, Context, Ui, TextEdit, 
    TextStyle, TextFormat, FontId,
    Color32, text::LayoutJob,
    ScrollArea, Align
};
use mk_core::Highlighter;

pub struct Editor {
    pub code: String,
    highlighter: Highlighter
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
        let mut layouter = |ui: &Ui, mark: &str, wrap_width: f32| {
            let mut layout_job = self.highlighter.highlight(ui.style(), mark);
            layout_job.wrap.max_width = wrap_width;
            ui.fonts(|f| f.layout_job(layout_job))
        };

        ui.horizontal_top(|ui| {
            Self::numlines(ui, self.code.as_str());
            ui.add(
                egui::TextEdit::multiline(&mut self.code)
                    .frame(false)
                    .code_editor()
                    .desired_width(f32::INFINITY)
                    .desired_rows(10)
                    .layouter(&mut layouter)
            );
        });
    }

    fn numlines(ui: &mut Ui, code: &str) {
        let total = if code.ends_with('\n') || code.is_empty() {
            code.lines().count() + 1
        } else {
            code.lines().count()
        };

        let max_indent = total.to_string().len();
        let mut counter = (1..=total)
            .map(|i| {
                let label = i.to_string();
                format!(
                    "{}{}",
                    " ".repeat(max_indent.saturating_sub(label.len())),
                    label
                )
            })
            .collect::<Vec<String>>()
            .join("\n");

        #[allow(clippy::cast_precision_loss)]
        let width = max_indent as f32 * 12.0 * 0.5;

        let mut layouter = |ui: &Ui, string: &str, _wrap_width: f32| {
            let layout_job = LayoutJob::single_section(
                string.to_string(),
                TextFormat::simple(
                    FontId::monospace(12.0),
                    Color32::from_gray(150)
                )
            );
            ui.fonts(|f| f.layout_job(layout_job))
        };
        
        ui.add(
            TextEdit::multiline(&mut counter)
                .id_source(format!("numlines"))
                .font(TextStyle::Monospace)
                .interactive(false)
                .frame(false)
                .desired_width(width)
                .desired_rows(10)
                .layouter(&mut layouter)
        );
    }
}
