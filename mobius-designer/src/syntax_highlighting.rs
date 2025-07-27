use syntect::easy::HighlightLines;
use syntect::highlighting::{ThemeSet, Style};
use syntect::parsing::SyntaxSet;
use syntect::util::LinesWithEndings;

pub struct SyntaxHighlighter {
    syntax_set: SyntaxSet,
    theme_set: ThemeSet,
}

impl Default for SyntaxHighlighter {
    fn default() -> Self {
        Self {
            syntax_set: SyntaxSet::load_defaults_newlines(),
            theme_set: ThemeSet::load_defaults(),
        }
    }
}

impl SyntaxHighlighter {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn highlight_rust_code(&self, code: &str) -> Vec<(egui::Color32, String)> {
        let syntax = self.syntax_set.find_syntax_by_extension("rs")
            .unwrap_or_else(|| self.syntax_set.find_syntax_plain_text());
        
        // Use a dark theme that works well with egui
        let theme = &self.theme_set.themes["base16-ocean.dark"];
        
        let mut highlighter = HighlightLines::new(syntax, theme);
        let mut highlighted_segments = Vec::new();
        
        for line in LinesWithEndings::from(code) {
            let ranges: Vec<(Style, &str)> = highlighter.highlight_line(line, &self.syntax_set)
                .unwrap_or_default();
            
            for (style, text) in ranges {
                let color = egui::Color32::from_rgb(
                    ((style.foreground.r as f32 / 255.0) * 255.0) as u8,
                    ((style.foreground.g as f32 / 255.0) * 255.0) as u8,
                    ((style.foreground.b as f32 / 255.0) * 255.0) as u8,
                );
                highlighted_segments.push((color, text.to_string()));
            }
        }
        
        highlighted_segments
    }

    pub fn render_highlighted_code(&self, ui: &mut egui::Ui, code: &str) {
        let highlighted = self.highlight_rust_code(code);
        
        egui::ScrollArea::vertical()
            .max_height(ui.available_height() - 20.0)
            .show(ui, |ui| {
                ui.with_layout(egui::Layout::top_down(egui::Align::LEFT), |ui| {
                    ui.style_mut().wrap_mode = Some(egui::TextWrapMode::Extend);
                    
                    // Use a monospace font for code
                    let font_id = egui::FontId::monospace(12.0);
                    
                    let mut job = egui::text::LayoutJob::default();
                    
                    for (color, text) in highlighted {
                        job.append(
                            &text,
                            0.0,
                            egui::TextFormat {
                                font_id: font_id.clone(),
                                color,
                                ..Default::default()
                            },
                        );
                    }
                    
                    ui.add(egui::Label::new(job));
                });
            });
    }
}