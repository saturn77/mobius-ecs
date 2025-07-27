use bevy_ecs::prelude::*;
use crate::components::*;
use crate::integration::TabKind;
use std::fmt::Write;

pub struct CodeGenerator {
    indent_level: usize,
    code: String,
}

impl CodeGenerator {
    pub fn new() -> Self {
        Self {
            indent_level: 0,
            code: String::new(),
        }
    }

    fn indent(&self) -> String {
        "    ".repeat(self.indent_level)
    }

    fn writeln(&mut self, line: &str) {
        let indent = self.indent();
        writeln!(&mut self.code, "{}{}", indent, line).unwrap();
    }

    fn write_raw(&mut self, text: &str) {
        write!(&mut self.code, "{}", text).unwrap();
    }

    pub fn generate_panel_function(world: &mut World, panel_name: &str) -> String {
        let mut gen = CodeGenerator::new();
        
        // Generate imports
        gen.writeln("use egui;");
        gen.writeln("");
        
        // Generate function signature
        gen.writeln(&format!("pub fn show_{}_panel(ui: &mut egui::Ui, app: &mut App) {{", 
            panel_name.to_lowercase().replace(" ", "_")));
        gen.indent_level += 1;
        
        // Add panel heading
        gen.writeln(&format!("ui.heading(\"{}\");", panel_name));
        gen.writeln("ui.separator();");
        gen.writeln("");
        
        // Collect all UI elements for MainWork tab
        let mut elements = Vec::new();
        let mut query = world.query::<(
            Entity,
            &UiElementPosition,
            &UiElementSize,
            &UiElementTab,
            Option<&UiButton>,
            Option<&UiTextInput>,
            Option<&UiCheckbox>,
            Option<&UiRadioButton>,
            Option<&UiGroupBox>,
        )>();
        
        for (entity, pos, size, tab, button, text_input, checkbox, radio, group_box) in query.iter(world) {
            if matches!(tab.tab_kind, TabKind::MainWork) {
                elements.push((entity, pos, size, button, text_input, checkbox, radio, group_box));
            }
        }
        
        // Sort elements by position (top to bottom, left to right)
        elements.sort_by(|a, b| {
            let y_cmp = a.1.y.partial_cmp(&b.1.y).unwrap();
            if y_cmp == std::cmp::Ordering::Equal {
                a.1.x.partial_cmp(&b.1.x).unwrap()
            } else {
                y_cmp
            }
        });
        
        // Generate positioned elements using egui::Area for exact positioning
        gen.writeln("// Elements positioned exactly as in the designer");
        for element in elements {
            gen.generate_positioned_element_code(element);
            gen.writeln("");
        }
        
        gen.indent_level -= 1;
        gen.writeln("}");
        
        gen.code
    }
    
    fn generate_positioned_element_code(&mut self, element: (
        Entity,
        &UiElementPosition,
        &UiElementSize,
        Option<&UiButton>,
        Option<&UiTextInput>,
        Option<&UiCheckbox>,
        Option<&UiRadioButton>,
        Option<&UiGroupBox>,
    )) {
        let (entity, pos, size, button, text_input, checkbox, radio, group_box) = element;
        
        // Generate Area with fixed position
        let area_id = format!("element_{:?}", entity);
        self.writeln(&format!("egui::Area::new(egui::Id::new(\"{}\"))", area_id));
        self.indent_level += 1;
        self.writeln(&format!(".fixed_pos(egui::pos2({:.1}, {:.1}))", pos.x, pos.y));
        self.writeln(".show(ui.ctx(), |ui| {");
        self.indent_level += 1;
        
        if let Some(button) = button {
            self.generate_positioned_button_code(button, size);
        } else if let Some(text_input) = text_input {
            self.generate_positioned_text_input_code(text_input, size);
        } else if let Some(checkbox) = checkbox {
            self.generate_positioned_checkbox_code(checkbox);
        } else if let Some(radio) = radio {
            self.generate_positioned_radio_button_code(radio);
        } else if let Some(group_box) = group_box {
            self.generate_positioned_group_box_code(group_box, size);
        }
        
        self.indent_level -= 1;
        self.writeln("});");
        self.indent_level -= 1;
    }
    
    fn generate_element_code(&mut self, element: (
        Entity,
        &UiElementPosition,
        &UiElementSize,
        Option<&UiButton>,
        Option<&UiTextInput>,
        Option<&UiCheckbox>,
        Option<&UiRadioButton>,
        Option<&UiGroupBox>,
    )) {
        let (_, _pos, size, button, text_input, checkbox, radio, group_box) = element;
        
        if let Some(button) = button {
            self.generate_button_code(button, size);
        } else if let Some(text_input) = text_input {
            self.generate_text_input_code(text_input, size);
        } else if let Some(checkbox) = checkbox {
            self.generate_checkbox_code(checkbox);
        } else if let Some(radio) = radio {
            self.generate_radio_button_code(radio);
        } else if let Some(group_box) = group_box {
            self.generate_group_box_code(group_box, size);
        }
    }
    
    fn generate_button_code(&mut self, button: &UiButton, size: &UiElementSize) {
        self.writeln(&format!("if ui.add_sized("));
        self.indent_level += 1;
        self.writeln(&format!("egui::vec2({:.1}, {:.1}),", size.width, size.height));
        self.writeln(&format!("egui::Button::new(\"{}\"){}",
            button.label,
            if !button.enabled { ".enabled(false)" } else { "" }
        ));
        self.indent_level -= 1;
        self.writeln(").clicked() {");
        self.indent_level += 1;
        self.writeln(&format!("// TODO: Handle {} button click", button.label));
        self.indent_level -= 1;
        self.writeln("}");
    }
    
    fn generate_text_input_code(&mut self, text_input: &UiTextInput, size: &UiElementSize) {
        self.writeln(&format!("ui.horizontal(|ui| {{"));
        self.indent_level += 1;
        self.writeln(&format!("ui.label(\"{}\");", text_input.label));
        self.writeln(&format!("ui.add_sized("));
        self.indent_level += 1;
        self.writeln(&format!("egui::vec2({:.1}, {:.1}),", size.width, size.height));
        self.writeln(&format!("egui::TextEdit::singleline(&mut app.{}){}",
            text_input.label.to_lowercase().replace(" ", "_"),
            if !text_input.enabled { ".enabled(false)" } else { "" }
        ));
        self.indent_level -= 1;
        self.writeln(");");
        self.indent_level -= 1;
        self.writeln("});");
    }
    
    fn generate_checkbox_code(&mut self, checkbox: &UiCheckbox) {
        self.writeln(&format!("ui.checkbox(&mut app.{}, \"{}\"){};",
            checkbox.label.to_lowercase().replace(" ", "_").replace("?", ""),
            checkbox.label,
            if !checkbox.enabled { ".enabled(false)" } else { "" }
        ));
    }
    
    fn generate_radio_button_code(&mut self, radio: &UiRadioButton) {
        self.writeln(&format!("ui.radio_value(&mut app.{}_selection, \"{}\", \"{}\"){};",
            radio.group_id,
            radio.label,
            radio.label,
            if !radio.enabled { ".enabled(false)" } else { "" }
        ));
    }
    
    fn generate_group_box_code(&mut self, group_box: &UiGroupBox, size: &UiElementSize) {
        self.writeln(&format!("ui.group(|ui| {{"));
        self.indent_level += 1;
        self.writeln(&format!("ui.set_min_size(egui::vec2({:.1}, {:.1}));", size.width, size.height));
        self.writeln(&format!("ui.label(\"{}\");", group_box.label));
        self.writeln("ui.separator();");
        self.writeln("// TODO: Add group box contents");
        self.indent_level -= 1;
        self.writeln("});");
    }
    
    // Positioned versions for exact layout matching
    fn generate_positioned_button_code(&mut self, button: &UiButton, size: &UiElementSize) {
        self.writeln(&format!("if ui.add_sized("));
        self.indent_level += 1;
        self.writeln(&format!("egui::vec2({:.1}, {:.1}),", size.width, size.height));
        self.writeln(&format!("egui::Button::new(\"{}\"){}",
            button.label,
            if !button.enabled { ".enabled(false)" } else { "" }
        ));
        self.indent_level -= 1;
        self.writeln(").clicked() {");
        self.indent_level += 1;
        self.writeln(&format!("// TODO: Handle {} button click", button.label));
        self.indent_level -= 1;
        self.writeln("}");
    }
    
    fn generate_positioned_text_input_code(&mut self, text_input: &UiTextInput, size: &UiElementSize) {
        self.writeln(&format!("ui.vertical(|ui| {{"));
        self.indent_level += 1;
        self.writeln(&format!("ui.label(\"{}\");", text_input.label));
        self.writeln(&format!("ui.add_sized("));
        self.indent_level += 1;
        self.writeln(&format!("egui::vec2({:.1}, {:.1}),", size.width, size.height));
        self.writeln(&format!("egui::TextEdit::singleline(&mut app.{}){}",
            text_input.label.to_lowercase().replace(" ", "_"),
            if !text_input.enabled { ".enabled(false)" } else { "" }
        ));
        self.indent_level -= 1;
        self.writeln(");");
        self.indent_level -= 1;
        self.writeln("});");
    }
    
    fn generate_positioned_checkbox_code(&mut self, checkbox: &UiCheckbox) {
        self.writeln(&format!("ui.checkbox(&mut app.{}, \"{}\"){};",
            checkbox.label.to_lowercase().replace(" ", "_").replace("?", ""),
            checkbox.label,
            if !checkbox.enabled { ".enabled(false)" } else { "" }
        ));
    }
    
    fn generate_positioned_radio_button_code(&mut self, radio: &UiRadioButton) {
        self.writeln(&format!("ui.radio_value(&mut app.{}_selection, \"{}\", \"{}\"){};",
            radio.group_id,
            radio.label,
            radio.label,
            if !radio.enabled { ".enabled(false)" } else { "" }
        ));
    }
    
    fn generate_positioned_group_box_code(&mut self, group_box: &UiGroupBox, size: &UiElementSize) {
        self.writeln(&format!("ui.group(|ui| {{"));
        self.indent_level += 1;
        self.writeln(&format!("ui.set_min_size(egui::vec2({:.1}, {:.1}));", size.width, size.height));
        self.writeln(&format!("ui.label(\"{}\");", group_box.label));
        self.writeln("ui.separator();");
        self.writeln("// TODO: Add group box contents");
        self.indent_level -= 1;
        self.writeln("});");
    }
    
    pub fn generate_full_app_code(world: &mut World) -> String {
        let mut gen = CodeGenerator::new();
        
        // Generate file header
        gen.writeln("// Generated by Mobius Designer");
        gen.writeln("// This code provides a starting point for your egui application");
        gen.writeln("");
        gen.writeln("use eframe::egui;");
        gen.writeln("");
        
        // Generate app struct
        gen.writeln("#[derive(Default)]");
        gen.writeln("pub struct App {");
        gen.indent_level += 1;
        
        // Add fields based on UI elements
        gen.generate_app_fields(world);
        
        gen.indent_level -= 1;
        gen.writeln("}");
        gen.writeln("");
        
        // Generate implementation
        gen.writeln("impl App {");
        gen.indent_level += 1;
        gen.writeln("pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {");
        gen.indent_level += 1;
        gen.writeln("Default::default()");
        gen.indent_level -= 1;
        gen.writeln("}");
        gen.indent_level -= 1;
        gen.writeln("}");
        gen.writeln("");
        
        // Generate eframe::App implementation
        gen.writeln("impl eframe::App for App {");
        gen.indent_level += 1;
        gen.writeln("fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {");
        gen.indent_level += 1;
        gen.writeln("// Top menu bar");
        gen.writeln("egui::TopBottomPanel::top(\"top_panel\").show(ctx, |ui| {");
        gen.indent_level += 1;
        gen.writeln("egui::menu::bar(ui, |ui| {");
        gen.indent_level += 1;
        gen.writeln("ui.menu_button(\"File\", |ui| {");
        gen.indent_level += 1;
        gen.writeln("if ui.button(\"New\").clicked() {");
        gen.indent_level += 1;
        gen.writeln("// TODO: Implement new file");
        gen.indent_level -= 1;
        gen.writeln("}");
        gen.writeln("if ui.button(\"Open\").clicked() {");
        gen.indent_level += 1;
        gen.writeln("// TODO: Implement open file");
        gen.indent_level -= 1;
        gen.writeln("}");
        gen.writeln("if ui.button(\"Save\").clicked() {");
        gen.indent_level += 1;
        gen.writeln("// TODO: Implement save file");
        gen.indent_level -= 1;
        gen.writeln("}");
        gen.indent_level -= 1;
        gen.writeln("});");
        gen.indent_level -= 1;
        gen.writeln("});");
        gen.indent_level -= 1;
        gen.writeln("});");
        gen.writeln("");
        gen.writeln("// Main content area with generated UI");
        gen.writeln("egui::CentralPanel::default().show(ctx, |ui| {");
        gen.indent_level += 1;
        gen.writeln("ui.heading(\"My Generated App\");");
        gen.writeln("ui.separator();");
        gen.writeln("");
        gen.writeln("// Call the generated panel function");
        gen.writeln("show_main_panel(ui, self);");
        gen.indent_level -= 1;
        gen.writeln("});");
        gen.indent_level -= 1;
        gen.writeln("}");
        gen.indent_level -= 1;
        gen.writeln("}");
        gen.writeln("");
        
        // Generate panel function
        let panel_code = Self::generate_panel_function(world, "Main");
        gen.write_raw(&panel_code);
        gen.writeln("");
        
        // Generate main function
        gen.writeln("fn main() -> Result<(), eframe::Error> {");
        gen.indent_level += 1;
        gen.writeln("let options = eframe::NativeOptions {");
        gen.indent_level += 1;
        gen.writeln("viewport: egui::ViewportBuilder::default().with_inner_size([800.0, 600.0]),");
        gen.writeln("..Default::default()");
        gen.indent_level -= 1;
        gen.writeln("};");
        gen.writeln("");
        gen.writeln("eframe::run_native(");
        gen.indent_level += 1;
        gen.writeln("\"My egui App\",");
        gen.writeln("options,");
        gen.writeln("Box::new(|cc| Box::new(App::new(cc))),");
        gen.indent_level -= 1;
        gen.writeln(")");
        gen.indent_level -= 1;
        gen.writeln("}");
        
        gen.code
    }
    
    fn generate_app_fields(&mut self, world: &mut World) {
        // Collect unique field names from UI elements
        let mut fields = std::collections::HashSet::new();
        
        let mut query = world.query::<(
            &UiElementTab,
            Option<&UiTextInput>,
            Option<&UiCheckbox>,
            Option<&UiRadioButton>,
        )>();
        
        for (tab, text_input, checkbox, radio) in query.iter(world) {
            if matches!(tab.tab_kind, TabKind::MainWork) {
                if let Some(text_input) = text_input {
                    let field_name = text_input.label.to_lowercase().replace(" ", "_");
                    if fields.insert(field_name.clone()) {
                        self.writeln(&format!("{}: String,", field_name));
                    }
                } else if let Some(checkbox) = checkbox {
                    let field_name = checkbox.label.to_lowercase().replace(" ", "_").replace("?", "");
                    if fields.insert(field_name.clone()) {
                        self.writeln(&format!("{}: bool,", field_name));
                    }
                } else if let Some(radio) = radio {
                    let field_name = format!("{}_selection", radio.group_id);
                    if fields.insert(field_name.clone()) {
                        self.writeln(&format!("{}: String,", field_name));
                    }
                }
            }
        }
    }
}