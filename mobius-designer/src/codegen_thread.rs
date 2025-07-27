use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::time::Instant;
use bevy_ecs::prelude::*;
use egui_mobius::signals::Signal;
use egui_mobius::slot::Slot;
use egui_mobius::types::Value;

use crate::events::{CodeGenEvent, CodeGenResponse, CodeGenMode};
use crate::components::*;
use crate::integration::TabKind;
use crate::codegen::CodeGenerator;

/// Background thread for code generation
/// This runs in a separate thread to avoid blocking the UI
pub fn start_codegen_thread(
    mut slot_from_ui: Slot<CodeGenEvent>,
    signal_to_ui: Signal<CodeGenResponse>,
    world_snapshot: Value<Option<WorldSnapshot>>,
) {
    slot_from_ui.start(move |event: CodeGenEvent| {
        match event {
            CodeGenEvent::RegenerateCode { tab_kind, mode } => {
                let start_time = Instant::now();
                
                // Get world snapshot
                let world_snapshot = world_snapshot.lock().unwrap();
                if let Some(ref snapshot) = *world_snapshot {
                    match generate_code_from_snapshot(snapshot, &mode, &tab_kind) {
                        Ok(generated_code) => {
                            let generation_time = start_time.elapsed().as_millis() as u64;
                            
                            let response = CodeGenResponse::CodeReady {
                                tab_kind,
                                mode,
                                code: generated_code,
                                generation_time_ms: generation_time,
                            };
                            
                            if let Err(e) = signal_to_ui.send(response) {
                                eprintln!("Failed to send generated code to UI: {}", e);
                            }
                        }
                        Err(e) => {
                            let response = CodeGenResponse::Error {
                                message: format!("Code generation failed: {}", e),
                            };
                            
                            if let Err(e) = signal_to_ui.send(response) {
                                eprintln!("Failed to send error to UI: {}", e);
                            }
                        }
                    }
                } else {
                    let response = CodeGenResponse::Error {
                        message: "No world snapshot available for code generation".to_string(),
                    };
                    
                    if let Err(e) = signal_to_ui.send(response) {
                        eprintln!("Failed to send error to UI: {}", e);
                    }
                }
            }
            
            CodeGenEvent::ClearCache => {
                if let Err(e) = signal_to_ui.send(CodeGenResponse::CacheCleared) {
                    eprintln!("Failed to send cache cleared response: {}", e);
                }
            }
            
            CodeGenEvent::Shutdown => {
                // Thread will naturally exit when the slot is dropped
                return;
            }
        }
    });
}

/// Snapshot of world state for code generation
/// This allows us to generate code without holding a mutable reference to the world
#[derive(Clone, Debug)]
pub struct WorldSnapshot {
    pub ui_elements: Vec<UiElementSnapshot>,
    pub grid_settings: GridSettings,
    pub hash: u64,
}

#[derive(Clone, Debug, Hash)]
pub struct UiElementSnapshot {
    pub entity_id: Entity,
    pub position: UiElementPosition,
    pub size: UiElementSize,
    pub tab: UiElementTab,
    pub element_type: UiElementType,
}

#[derive(Clone, Debug)]
pub enum UiElementType {
    Button {
        label: String,
        enabled: bool,
        font_size: f32,
    },
    TextInput {
        label: String,
        value: String,
        enabled: bool,
        font_size: f32,
    },
    Checkbox {
        label: String,
        checked: bool,
        enabled: bool,
        font_size: f32,
    },
    RadioButton {
        label: String,
        selected: bool,
        enabled: bool,
        font_size: f32,
        group_id: String,
    },
    GroupBox {
        label: String,
        enabled: bool,
        font_size: f32,
    },
}

impl std::hash::Hash for UiElementType {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            UiElementType::Button { label, enabled, font_size } => {
                0u8.hash(state);
                label.hash(state);
                enabled.hash(state);
                font_size.to_bits().hash(state);
            }
            UiElementType::TextInput { label, value, enabled, font_size } => {
                1u8.hash(state);
                label.hash(state);
                value.hash(state);
                enabled.hash(state);
                font_size.to_bits().hash(state);
            }
            UiElementType::Checkbox { label, checked, enabled, font_size } => {
                2u8.hash(state);
                label.hash(state);
                checked.hash(state);
                enabled.hash(state);
                font_size.to_bits().hash(state);
            }
            UiElementType::RadioButton { label, selected, enabled, font_size, group_id } => {
                3u8.hash(state);
                label.hash(state);
                selected.hash(state);
                enabled.hash(state);
                font_size.to_bits().hash(state);
                group_id.hash(state);
            }
            UiElementType::GroupBox { label, enabled, font_size } => {
                4u8.hash(state);
                label.hash(state);
                enabled.hash(state);
                font_size.to_bits().hash(state);
            }
        }
    }
}

impl WorldSnapshot {
    /// Create a snapshot from the current world state
    pub fn from_world(world: &mut World) -> Self {
        let mut ui_elements = Vec::new();
        
        // Collect UI elements
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
            let element_type = if let Some(button) = button {
                UiElementType::Button {
                    label: button.label.clone(),
                    enabled: button.enabled,
                    font_size: button.font_size,
                }
            } else if let Some(text_input) = text_input {
                UiElementType::TextInput {
                    label: text_input.label.clone(),
                    value: text_input.value.clone(),
                    enabled: text_input.enabled,
                    font_size: text_input.font_size,
                }
            } else if let Some(checkbox) = checkbox {
                UiElementType::Checkbox {
                    label: checkbox.label.clone(),
                    checked: checkbox.checked,
                    enabled: checkbox.enabled,
                    font_size: checkbox.font_size,
                }
            } else if let Some(radio) = radio {
                UiElementType::RadioButton {
                    label: radio.label.clone(),
                    selected: radio.selected,
                    enabled: radio.enabled,
                    font_size: radio.font_size,
                    group_id: radio.group_id.clone(),
                }
            } else if let Some(group_box) = group_box {
                UiElementType::GroupBox {
                    label: group_box.label.clone(),
                    enabled: group_box.enabled,
                    font_size: group_box.font_size,
                }
            } else {
                continue; // Skip unknown element types
            };
            
            ui_elements.push(UiElementSnapshot {
                entity_id: entity,
                position: pos.clone(),
                size: size.clone(),
                tab: tab.clone(),
                element_type,
            });
        }
        
        // Get grid settings
        let grid_settings = world.query::<&GridSettings>()
            .iter(world)
            .next()
            .cloned()
            .unwrap_or_default();
        
        // Calculate hash for change detection
        let mut hasher = DefaultHasher::new();
        ui_elements.hash(&mut hasher);
        grid_settings.hash(&mut hasher);
        let hash = hasher.finish();
        
        Self {
            ui_elements,
            grid_settings,
            hash,
        }
    }
}

/// Generate code from a world snapshot
fn generate_code_from_snapshot(
    snapshot: &WorldSnapshot,
    mode: &CodeGenMode,
    tab_kind: &TabKind,
) -> Result<String, String> {
    // Filter elements for the specific tab
    let tab_elements: Vec<&UiElementSnapshot> = snapshot.ui_elements
        .iter()
        .filter(|element| element.tab.tab_kind == *tab_kind)
        .collect();
    
    match mode {
        CodeGenMode::FullApp => generate_full_app_from_snapshot(&tab_elements),
        CodeGenMode::PanelFunction => generate_panel_from_snapshot(&tab_elements),
    }
}

fn generate_full_app_from_snapshot(elements: &[&UiElementSnapshot]) -> Result<String, String> {
    // First generate the panel function
    let panel_code = generate_panel_from_snapshot(elements)?;
    
    // Now generate the full app that uses the panel function
    let mut code = String::from("// Generated by Mobius Designer\n");
    code.push_str("// Full Application Code\n\n");
    code.push_str("use eframe::egui;\n\n");
    
    // Add the panel function first
    code.push_str(&panel_code);
    code.push_str("\n\n");
    
    // Add the App struct
    code.push_str("#[derive(Default)]\n");
    code.push_str("pub struct App {\n");
    code.push_str("    // Add your app state fields here\n");
    code.push_str("}\n\n");
    
    // Implement the App trait
    code.push_str("impl eframe::App for App {\n");
    code.push_str("    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {\n");
    code.push_str("        egui::CentralPanel::default().show(ctx, |ui| {\n");
    code.push_str("            show_generated_panel(ui, self);\n");
    code.push_str("        });\n");
    code.push_str("    }\n");
    code.push_str("}\n\n");
    
    // Add the main function
    code.push_str("fn main() -> Result<(), eframe::Error> {\n");
    code.push_str("    let options = eframe::NativeOptions::default();\n");
    code.push_str("    eframe::run_native(\n");
    code.push_str("        \"Generated App\",\n");
    code.push_str("        options,\n");
    code.push_str("        Box::new(|_cc| Ok(Box::new(App::default()))),\n");
    code.push_str("    )\n");
    code.push_str("}\n");
    
    Ok(code)
}

fn generate_panel_from_snapshot(elements: &[&UiElementSnapshot]) -> Result<String, String> {
    // Generate panel function code
    let mut code = String::from("// Generated Panel Function\n");
    code.push_str("use egui;\n\n");
    code.push_str("pub fn show_generated_panel(ui: &mut egui::Ui, app: &mut App) {\n");
    code.push_str("    ui.heading(\"Generated Panel\");\n");
    code.push_str("    ui.separator();\n\n");
    
    for element in elements {
        match &element.element_type {
            UiElementType::Button { label, enabled, .. } => {
                code.push_str(&format!(
                    "    egui::Area::new(egui::Id::new(\"btn_{:?}\"))\n",
                    element.entity_id
                ));
                code.push_str(&format!(
                    "        .fixed_pos(egui::pos2({:.1}, {:.1}))\n",
                    element.position.x, element.position.y
                ));
                code.push_str("        .show(ui.ctx(), |ui| {\n");
                code.push_str(&format!(
                    "            if ui.add_sized(egui::vec2({:.1}, {:.1}), egui::Button::new(\"{}\"){}).clicked() {{\n",
                    element.size.width, element.size.height, label,
                    if !enabled { ".enabled(false)" } else { "" }
                ));
                code.push_str(&format!(
                    "                // Handle {} button click\n",
                    label
                ));
                code.push_str("            }\n");
                code.push_str("        });\n");
            }
            _ => {
                code.push_str(&format!("    // TODO: Implement {:?}\n", element.element_type));
            }
        }
    }
    
    code.push_str("}\n");
    
    Ok(code)
}