use eframe::egui::{self};
use super::parser;

#[derive(Default)]
pub struct Highlighter {
    style: egui::Style,
    code: String,
    output: egui::text::LayoutJob
}

impl Highlighter {
    pub fn highlight(
        &mut self, 
        egui_style: &egui::Style,
        code: &str
    ) -> egui::text::LayoutJob {
        if (&self.style, self.code.as_str()) != (egui_style, code) {
            self.style = egui_style.clone();
            code.clone_into(&mut self.code);
            self.output = highlight_mark(egui_style, code);
        }
        self.output.clone()
    }
}

pub fn highlight_mark(
    egui_style: &egui::Style, 
    mut text: &str
) -> egui::text::LayoutJob {
    let mut job = egui::text::LayoutJob::default();
    let mut style = parser::Style::default();
    let mut start_of_line = true;

    while !text.is_empty() {
        if start_of_line && text.starts_with("```") {
            let end = text.find("\n```").map_or_else(|| text.len(), |i| i + 4);
            job.append(
                &text[..end],
                0.0,
                format_from_style(
                    egui_style,
                    &parser::Style {
                        code: true,
                        ..Default::default()
                    }
                )
            );
            text = &text[end..];
            style = Default::default();
            continue;
        }

        if text.starts_with('`') {
            style.code = true;
            let end = text[1..]
                .find(&['`', '\n'][..])
                .map_or_else(|| text.len(), |i| i + 2);
            job.append(
                &text[..end], 
                0.0, 
                format_from_style(egui_style, &style)
            );
            text = &text[end..];
            style.code = false;
            continue;
        }

        let mut skip;

        if text.starts_with('\\') && text.len() >= 2 {
            skip = 2;
        } else if start_of_line && text.starts_with(' ') {
            skip = 1;
        } else if start_of_line && text.starts_with("# ") {
            style.heading = true;
            skip = 2;
        } else if start_of_line && text.starts_with("## ") {
            style.subheading = true;
            skip = 3;
        } else if start_of_line && text.starts_with("> ") {
            style.quoted = true;
            skip = 2;
        } else if start_of_line && text.starts_with("- ") {
            skip = 2;
        } else if text.starts_with('*') {
            skip = 1;
            if style.strong {
                job.append(
                    &text[..skip], 
                    0.0, 
                    format_from_style(egui_style, &style)
                );
                text = &text[skip..];
                skip = 0;
            }
            style.strong ^= true;
        } else if text.starts_with('$') {
            skip = 1;
            if style.small {
                job.append(
                    &text[..skip],
                    0.0,
                    format_from_style(egui_style, &style)
                );
                text = &text[skip..];
                skip = 0;
            }
            style.small ^= true;
        } else if text.starts_with('^') {
            skip = 1;
            if style.raised {
                job.append(
                    &text[..skip],
                    0.0,
                    format_from_style(egui_style, &style)
                );
                text = &text[skip..];
                skip = 0;
            }
            style.raised ^= true;
        } else {
            skip = 0;
        }

        let line_end = text[skip..]
            .find('\n')
            .map_or_else(|| text.len(), |i| (skip + i + 1));
        let end = text[skip..]
            .find(&['*', '`', '~', '_', '/', '$', '^', '\\', '<', '['][..])
            .map_or_else(|| text.len(), |i| (skip + i).max(1));

        if line_end <= end {
            job.append(
                &text[..line_end],
                0.0,
                format_from_style(egui_style, &style)
            );
            text = &text[line_end..];
            start_of_line = true;
            style = Default::default();
        } else {
            job.append(
                &text[..end],
                0.0,
                format_from_style(egui_style, &style)
            );
            text = &text[end..];
            start_of_line = false;
        }
    }

    job
}

fn format_from_style(
    egui_style: &egui::Style,
    mark_style: &parser::Style
) -> egui::text::TextFormat {
    use egui::{Align, Color32, Stroke, TextStyle};

    let color = if mark_style.strong || mark_style.heading || mark_style.subheading {
        egui_style.visuals.strong_text_color()
    } else if mark_style.quoted {
        egui_style.visuals.weak_text_color()
    } else {
        egui_style.visuals.text_color()
    };

    let text_style = if mark_style.heading {
        TextStyle::Heading
    } else if mark_style.code {
        TextStyle::Monospace
    } else if mark_style.small | mark_style.raised {
        TextStyle::Small
    } else {
        TextStyle::Body
    };

    let background = if mark_style.code {
        egui_style.visuals.code_bg_color
    } else {
        Color32::TRANSPARENT
    };

    let underline = if mark_style.underline {
        Stroke::new(1.0, color)
    } else {
        Stroke::NONE
    };

    let strikethrough = if mark_style.strikethrough {
        Stroke::new(1.0, color)
    } else {
        Stroke::NONE
    };

    let valign = if mark_style.raised {
        Align::TOP
    } else {
        Align::BOTTOM
    };

    egui::text::TextFormat {
        font_id: text_style.resolve(egui_style),
        color,
        background,
        italics: mark_style.italics,
        underline,
        strikethrough,
        valign,
        ..Default::default()
    }
}
