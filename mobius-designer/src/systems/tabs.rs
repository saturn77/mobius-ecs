use bevy_ecs::prelude::*;
use egui::Ui;
use crate::integration::*;
use crate::components::*;
use crate::resources::*;
use crate::utils::*;
use crate::bundles::*;
use crate::systems::distribution::*;

pub fn render_tab_content(ui: &mut Ui, world: &mut World, tab: &Tab, renaming_entity: &mut Option<Entity>, rename_buffer: &mut String, show_add_menu: &mut bool, add_menu_pos: &mut egui::Pos2, resizing_entity: &mut Option<Entity>, drag_selection: &mut Option<crate::integration::DragSelection>, codegen_state: Option<&mut crate::events::CodeGenState>, file_dialog: &mut egui_file_dialog::FileDialog) {
    match tab.kind {
        TabKind::MainWork => {
            // Main work tab - just render the designer UI elements
            
            // Get actual grid settings from world
            let grid_settings = {
                let mut grid_query = world.query::<&GridSettings>();
                if let Some(settings) = grid_query.iter(world).next() {
                    settings.clone()
                } else {
                    GridSettings::default()
                }
            };
            
            ui.colored_label(egui::Color32::from_rgb(0, 255, 0), "✏️ Design Mode - Drag elements to move them");
            
            // First render UI elements
            crate::systems::render_dynamic_ui_elements(
                ui, 
                world, 
                &grid_settings,
                renaming_entity,
                rename_buffer,
                resizing_entity
            );
            
            // Handle drag selection - always enabled in design tool
            // Create a response for the main work area only
            let work_area_response = ui.allocate_response(ui.available_size(), egui::Sense::click_and_drag());
            handle_drag_selection_in_work_area(work_area_response, world, drag_selection);
            
            // Handle right-click to show add menu
            if ui.input(|i| i.pointer.button_clicked(egui::PointerButton::Secondary)) {
                if let Some(pos) = ui.ctx().pointer_interact_pos() {
                    *show_add_menu = true;
                    *add_menu_pos = pos;
                }
            }
            
            // Show the add menu if requested
            if *show_add_menu {
                egui::Window::new("Add UI Element")
                    .fixed_pos(*add_menu_pos)
                    .collapsible(false)
                    .resizable(false)
                    .title_bar(false)
                    .fixed_size(egui::Vec2::new(350.0, 0.0))
                    .show(ui.ctx(), |ui| {
                        
                        // Distribution options (only show if multiple elements are selected)
                        let selected_count = count_selected_elements(world);
                        if selected_count > 1 {
                            ui.columns(3, |columns| {
                                // First column - Design tool mode
                                columns[0].label("Mode:");
                                columns[0].label("✏️ Design");
                                
                                // Second column - Distribute
                                columns[1].label("Distribute:");
                                if columns[1].button("↕️ Vert").clicked() {
                                    let settings = if let Some(settings) = world.get_resource::<DistributionSettings>() {
                                        settings.clone()
                                    } else {
                                        DistributionSettings::new()
                                    };
                                    distribute_items_vertically(world, &settings);
                                    *show_add_menu = false;
                                }
                                if columns[1].button("↔️ Horiz").clicked() {
                                    let settings = if let Some(settings) = world.get_resource::<DistributionSettings>() {
                                        settings.clone()
                                    } else {
                                        DistributionSettings::new()
                                    };
                                    distribute_items_horizontally(world, &settings);
                                    *show_add_menu = false;
                                }
                                
                                // Third column - Align
                                columns[2].label("Align:");
                                if columns[2].button("⬅️ L").clicked() {
                                    align_selected_elements_left(world);
                                    *show_add_menu = false;
                                }
                                if columns[2].button("➡️ R").clicked() {
                                    align_selected_elements_right(world);
                                    *show_add_menu = false;
                                }
                                if columns[2].button("⬆️ T").clicked() {
                                    align_selected_elements_top(world);
                                    *show_add_menu = false;
                                }
                                if columns[2].button("⬇️ B").clicked() {
                                    align_selected_elements_bottom(world);
                                    *show_add_menu = false;
                                }
                            });
                            ui.separator();
                        } else {
                            // When no multiple selection, show design mode status
                            ui.label("✏️ Design Mode - Select elements to distribute/align");
                            ui.separator();
                        }
                        
                        ui.label("Add UI Element:");
                        ui.separator();
                        
                        if ui.button("➕ Add Button").clicked() {
                            add_ui_element_at_position_in_tab(world, "button", add_menu_pos.x, add_menu_pos.y, tab.kind.clone());
                            *show_add_menu = false;
                        }
                        
                        if ui.button("📝 Add Text Input").clicked() {
                            add_ui_element_at_position_in_tab(world, "text_input", add_menu_pos.x, add_menu_pos.y, tab.kind.clone());
                            *show_add_menu = false;
                        }
                        
                        if ui.button("☑️ Add Checkbox").clicked() {
                            add_ui_element_at_position_in_tab(world, "checkbox", add_menu_pos.x, add_menu_pos.y, tab.kind.clone());
                            *show_add_menu = false;
                        }
                        
                        if ui.button("🔘 Add Radio Button").clicked() {
                            add_ui_element_at_position_in_tab(world, "radio_button", add_menu_pos.x, add_menu_pos.y, tab.kind.clone());
                            *show_add_menu = false;
                        }
                        
                        if ui.button("📦 Add Group Box").clicked() {
                            add_ui_element_at_position_in_tab(world, "group_box", add_menu_pos.x, add_menu_pos.y, tab.kind.clone());
                            *show_add_menu = false;
                        }
                        
                        // Close menu if clicked outside or escape pressed
                        if ui.input(|i| i.key_pressed(egui::Key::Escape)) {
                            *show_add_menu = false;
                        }
                    });
                
                // Close menu if clicked elsewhere
                if ui.input(|i| i.pointer.button_clicked(egui::PointerButton::Primary)) {
                    *show_add_menu = false;
                }
            }
        }
        TabKind::Settings => {
            // For settings, we need to show our custom DesignerSettingsPanel
            render_designer_settings_panel(ui, world);
        }
        TabKind::EventLogger => {
            mobius_ecs::show_event_logger_panel(ui, world);
        }
        TabKind::Controls => {
            // For controls, we need to show our custom DesignerControlsPanel
            render_designer_controls_panel(ui, world);
        }
        TabKind::Inspector => {
            render_inspector_panel(ui, world);
        }
        TabKind::Preview => {
            render_preview_panel(ui, world, codegen_state, file_dialog);
        }
    }
}

fn render_designer_settings_panel(ui: &mut Ui, world: &mut World) {
    let mut query = world.query::<&mut DesignerSettingsPanel>();
    
    if let Some(mut settings) = query.iter_mut(world).next() {
        ui.heading("Designer Settings");
        ui.separator();
        
        ui.group(|ui| {
            ui.label("Units");
            ui.horizontal(|ui| {
                ui.radio_value(&mut settings.units, LengthUnit::Metric, "Metric");
                ui.radio_value(&mut settings.units, LengthUnit::Imperial, "Imperial");
            });
        });
        
        ui.add_space(10.0);
        
        ui.group(|ui| {
            ui.label("Timezone");
            ui.text_edit_singleline(&mut settings.timezone);
        });
    } else {
        ui.label("No designer settings panel found");
    }
}

fn render_designer_controls_panel(ui: &mut Ui, world: &mut World) {
    let mut query = world.query::<&mut DesignerControlsPanel>();
    
    if let Some(mut controls) = query.iter_mut(world).next() {
        ui.heading("Designer Controls");
        ui.separator();
        
        ui.group(|ui| {
            ui.label("Tool Selection");
            ui.horizontal(|ui| {
                ui.radio_value(&mut controls.selected_tool, Tool::Select, "Select");
                ui.radio_value(&mut controls.selected_tool, Tool::Draw, "Draw");
                ui.radio_value(&mut controls.selected_tool, Tool::Measure, "Measure");
                ui.radio_value(&mut controls.selected_tool, Tool::Place, "Place");
            });
        });
        
        ui.add_space(10.0);
        
        // Align & Distribute section
        let selected_count = count_selected_elements(world);
        ui.group(|ui| {
            ui.label("Align & Distribute");
            ui.separator();
            
            if selected_count == 0 {
                ui.label("Select elements to enable alignment");
            } else {
                ui.label(format!("{} elements selected", selected_count));
                
                if selected_count > 1 {
                    // Get distribution settings from world resource
                    let settings = if let Some(settings) = world.get_resource::<DistributionSettings>() {
                        settings.clone()
                    } else {
                        DistributionSettings::new()
                    };
                    
                    ui.horizontal(|ui| {
                        if ui.button("↕️ Distribute Vertically").clicked() {
                            distribute_items_vertically(world, &settings);
                        }
                        if ui.button("↔️ Distribute Horizontally").clicked() {
                            distribute_items_horizontally(world, &settings);
                        }
                    });
                    
                    ui.horizontal(|ui| {
                        if ui.button("⬅️ Align Left").clicked() {
                            align_selected_elements_left(world);
                        }
                        if ui.button("➡️ Align Right").clicked() {
                            align_selected_elements_right(world);
                        }
                    });
                    
                    ui.horizontal(|ui| {
                        if ui.button("⬆️ Align Top").clicked() {
                            align_selected_elements_top(world);
                        }
                        if ui.button("⬇️ Align Bottom").clicked() {
                            align_selected_elements_bottom(world);
                        }
                    });
                }
            }
        });
        
        ui.add_space(10.0);
        
        // Layout Organization section
        ui.group(|ui| {
            ui.label("Layout Organization");
            ui.separator();
            
            if selected_count == 0 {
                ui.label("Select elements to organize layout");
            } else if selected_count == 1 {
                ui.label("Select 2+ elements for layout organization");
            } else {
                ui.label(format!("Organize {} selected elements", selected_count));
                
                ui.horizontal(|ui| {
                    if ui.button("📄 Column Layout").clicked() {
                        arrange_elements_in_column(world);
                        add_designer_log(world, &format!("Arranged {} elements in column layout", selected_count));
                    }
                    if ui.button("📰 Row Layout").clicked() {
                        arrange_elements_in_row(world);
                        add_designer_log(world, &format!("Arranged {} elements in row layout", selected_count));
                    }
                });
                
                ui.horizontal(|ui| {
                    ui.label("Layout Spacing:");
                    let mut layout_spacing = ui.ctx().memory(|mem| {
                        mem.data.get_temp(egui::Id::new("layout_spacing")).unwrap_or(20.0f32)
                    });
                    
                    if ui.add(egui::DragValue::new(&mut layout_spacing).prefix("Gap: ").range(0.0..=100.0)).changed() {
                        ui.ctx().memory_mut(|mem| {
                            mem.data.insert_temp(egui::Id::new("layout_spacing"), layout_spacing);
                        });
                    }
                });
            }
        });
        
        ui.add_space(10.0);
        
        // Distribution Spacing Settings
        ui.group(|ui| {
            ui.label("Distribution Spacing");
            ui.separator();
            
            // Get or create distribution settings
            if world.get_resource::<DistributionSettings>().is_none() {
                world.insert_resource(DistributionSettings::new());
            }
            
            // Check if we have multiple selected elements
            let selected_count = count_selected_elements(world);
            if selected_count < 2 {
                ui.label("Select 2+ elements to adjust spacing");
                
                // Still show the sliders but disabled
                let settings = world.get_resource::<DistributionSettings>().unwrap();
                ui.add_enabled_ui(false, |ui| {
                    ui.horizontal(|ui| {
                        ui.label("Horizontal Spacing:");
                        let mut temp = settings.horizontal_spacing;
                        ui.add(egui::Slider::new(&mut temp, 10.0..=500.0));
                    });
                    ui.horizontal(|ui| {
                        ui.label("Vertical Spacing:");
                        let mut temp = settings.vertical_spacing;
                        ui.add(egui::Slider::new(&mut temp, 10.0..=500.0));
                    });
                });
            } else {
                ui.label(format!("{} elements selected", selected_count));
                
                // Horizontal spacing slider
                let mut horizontal_changed = false;
                let mut new_horizontal_spacing = 0.0;
                {
                    let mut settings = world.get_resource_mut::<DistributionSettings>().unwrap();
                    let mut temp_horizontal = settings.horizontal_spacing;
                    ui.horizontal(|ui| {
                        ui.label("Horizontal Spacing:");
                        if ui.add(egui::Slider::new(&mut temp_horizontal, 10.0..=500.0)).changed() {
                            settings.horizontal_spacing = temp_horizontal;
                            new_horizontal_spacing = temp_horizontal;
                            horizontal_changed = true;
                        }
                    });
                }
                
                if horizontal_changed {
                    let settings = world.get_resource::<DistributionSettings>().unwrap().clone();
                    distribute_items_horizontally(world, &settings);
                }
                
                // Vertical spacing slider
                let mut vertical_changed = false;
                let mut new_vertical_spacing = 0.0;
                {
                    let mut settings = world.get_resource_mut::<DistributionSettings>().unwrap();
                    let mut temp_vertical = settings.vertical_spacing;
                    ui.horizontal(|ui| {
                        ui.label("Vertical Spacing:");
                        if ui.add(egui::Slider::new(&mut temp_vertical, 10.0..=500.0)).changed() {
                            settings.vertical_spacing = temp_vertical;
                            new_vertical_spacing = temp_vertical;
                            vertical_changed = true;
                        }
                    });
                }
                
                if vertical_changed {
                    let settings = world.get_resource::<DistributionSettings>().unwrap().clone();
                    distribute_items_vertically(world, &settings);
                }
            }
        });
        
        ui.add_space(10.0);
        
        // Add UI Elements section
        ui.group(|ui| {
            ui.label("Add UI Elements");
            ui.separator();
            
            // Get the current tab kind for adding elements
            let current_tab = TabKind::MainWork; // Default to MainWork tab
            
            ui.horizontal(|ui| {
                if ui.button("➕ Add Button").clicked() {
                    add_ui_element_at_position_in_tab(world, "button", 100.0, 100.0, current_tab.clone());
                }
                if ui.button("📝 Add Text Input").clicked() {
                    add_ui_element_at_position_in_tab(world, "text_input", 100.0, 100.0, current_tab.clone());
                }
            });
            
            ui.horizontal(|ui| {
                if ui.button("☑️ Add Checkbox").clicked() {
                    add_ui_element_at_position_in_tab(world, "checkbox", 100.0, 100.0, current_tab.clone());
                }
                if ui.button("🔘 Add Radio Button").clicked() {
                    add_ui_element_at_position_in_tab(world, "radio_button", 100.0, 100.0, current_tab.clone());
                }
            });
            
            if ui.button("📦 Add Group Box").clicked() {
                add_ui_element_at_position_in_tab(world, "group_box", 100.0, 100.0, current_tab);
            }
        });
        
        ui.add_space(10.0);
        
        // Code Generation section
        ui.group(|ui| {
            ui.label("Code Generation");
            ui.separator();
            
            ui.horizontal(|ui| {
                if ui.button("🦀 Generate Rust Code").clicked() {
                    let generated_code = crate::codegen::CodeGenerator::generate_full_app_code(world);
                    
                    // Save to clipboard
                    ui.ctx().copy_text(generated_code.clone());
                    
                    // Also save to file
                    if let Err(e) = std::fs::write("generated_app.rs", &generated_code) {
                        add_designer_log(world, &format!("Failed to save generated code: {}", e));
                    } else {
                        add_designer_log(world, "Generated Rust/egui code saved to generated_app.rs and copied to clipboard");
                    }
                }
                
                if ui.button("📋 Generate Panel Function").clicked() {
                    let panel_code = crate::codegen::CodeGenerator::generate_panel_function(world, "Generated");
                    
                    // Save to clipboard
                    ui.ctx().copy_text(panel_code.clone());
                    
                    // Also save to file
                    if let Err(e) = std::fs::write("generated_panel.rs", &panel_code) {
                        add_designer_log(world, &format!("Failed to save panel code: {}", e));
                    } else {
                        add_designer_log(world, "Generated panel function saved to generated_panel.rs and copied to clipboard");
                    }
                }
            });
            
            ui.horizontal(|ui| {
                ui.label("💡 Tip: Generated code is copied to clipboard and saved to file");
            });
        });
    } else {
        ui.label("No designer controls panel found");
    }
}

fn render_inspector_panel(ui: &mut Ui, world: &mut World) {
    ui.heading("Inspector");
    ui.separator();
    
    // Count selected entities
    let mut selected_count = 0;
    let mut selected_entities = Vec::new();
    
    let mut query = world.query::<(Entity, &UiElementSelected)>();
    for (entity, selected) in query.iter(world) {
        if selected.selected {
            selected_count += 1;
            selected_entities.push(entity);
        }
    }
    
    ui.label(format!("Selected: {} items", selected_count));
    
    if selected_count == 0 {
        ui.separator();
        ui.label("No items selected.");
        ui.label("Select UI elements to view their properties.");
    } else if selected_count > 1 {
        // Show group properties for multiple selected elements
        render_group_properties(ui, world, &selected_entities);
    } else {
        // Show properties for single selected entity
        for (i, entity) in selected_entities.iter().enumerate() {
            ui.separator();
            
            if selected_count > 1 {
                ui.label(format!("Item {} of {}:", i + 1, selected_count));
            }
            
            // Show editable properties based on component type
            if let Some(mut button) = world.get_mut::<UiButton>(*entity) {
                ui.label("🔘 Button Properties:");
                ui.horizontal(|ui| {
                    ui.label("Label:");
                    ui.text_edit_singleline(&mut button.label);
                });
                ui.horizontal(|ui| {
                    ui.label("Font Size:");
                    ui.add(egui::DragValue::new(&mut button.font_size).range(8.0..=48.0));
                });
                ui.checkbox(&mut button.enabled, "Enabled");
                
            } else if let Some(mut text_input) = world.get_mut::<UiTextInput>(*entity) {
                ui.label("📝 Text Input Properties:");
                ui.horizontal(|ui| {
                    ui.label("Label:");
                    ui.text_edit_singleline(&mut text_input.label);
                });
                ui.horizontal(|ui| {
                    ui.label("Value:");
                    ui.text_edit_singleline(&mut text_input.value);
                });
                ui.horizontal(|ui| {
                    ui.label("Font Size:");
                    ui.add(egui::DragValue::new(&mut text_input.font_size).range(8.0..=48.0));
                });
                ui.checkbox(&mut text_input.enabled, "Enabled");
                
            } else if let Some(mut checkbox) = world.get_mut::<UiCheckbox>(*entity) {
                ui.label("☑️ Checkbox Properties:");
                ui.horizontal(|ui| {
                    ui.label("Label:");
                    ui.text_edit_singleline(&mut checkbox.label);
                });
                ui.horizontal(|ui| {
                    ui.label("Font Size:");
                    ui.add(egui::DragValue::new(&mut checkbox.font_size).range(8.0..=48.0));
                });
                ui.checkbox(&mut checkbox.checked, "Checked");
                ui.checkbox(&mut checkbox.enabled, "Enabled");
                
            } else if let Some(mut radio_button) = world.get_mut::<UiRadioButton>(*entity) {
                ui.label("🔘 Radio Button Properties:");
                ui.horizontal(|ui| {
                    ui.label("Label:");
                    ui.text_edit_singleline(&mut radio_button.label);
                });
                ui.horizontal(|ui| {
                    ui.label("Group ID:");
                    ui.text_edit_singleline(&mut radio_button.group_id);
                });
                ui.horizontal(|ui| {
                    ui.label("Font Size:");
                    ui.add(egui::DragValue::new(&mut radio_button.font_size).range(8.0..=48.0));
                });
                ui.checkbox(&mut radio_button.selected, "Selected");
                ui.checkbox(&mut radio_button.enabled, "Enabled");
                
            } else if let Some(mut group_box) = world.get_mut::<UiGroupBox>(*entity) {
                ui.label("📦 Group Box Properties:");
                ui.horizontal(|ui| {
                    ui.label("Label:");
                    ui.text_edit_singleline(&mut group_box.label);
                });
                ui.horizontal(|ui| {
                    ui.label("Font Size:");
                    ui.add(egui::DragValue::new(&mut group_box.font_size).range(8.0..=48.0));
                });
                ui.checkbox(&mut group_box.enabled, "Enabled");
                ui.label(format!("Contains {} widgets", group_box.contained_widgets.len()));
            }
            
            // Show and allow editing position and size
            if let Some(mut pos) = world.get_mut::<UiElementPosition>(*entity) {
                ui.separator();
                ui.label("📍 Position & Size:");
                ui.horizontal(|ui| {
                    ui.label("X:");
                    ui.add(egui::DragValue::new(&mut pos.x).range(0.0..=2000.0));
                    ui.label("Y:");
                    ui.add(egui::DragValue::new(&mut pos.y).range(0.0..=2000.0));
                });
            }
            
            if let Some(mut size) = world.get_mut::<UiElementSize>(*entity) {
                ui.horizontal(|ui| {
                    ui.label("Width:");
                    ui.add(egui::DragValue::new(&mut size.width).range(10.0..=500.0));
                    ui.label("Height:");
                    ui.add(egui::DragValue::new(&mut size.height).range(10.0..=500.0));
                });
            }
            
            // Show tab assignment
            if let Some(tab) = world.get::<UiElementTab>(*entity) {
                ui.separator();
                ui.label(format!("📁 Tab: {:?}", tab.tab_kind));
            }
        }
    }
}

fn render_preview_panel(ui: &mut Ui, world: &mut World, codegen_state: Option<&mut crate::events::CodeGenState>, file_dialog: &mut egui_file_dialog::FileDialog) {
    if let Some(codegen_state) = codegen_state {
        render_preview_panel_threaded(ui, world, codegen_state, file_dialog);
    } else {
        ui.heading("Code Preview");
        ui.separator();
        ui.label("❌ CodeGen State not available");
        ui.label("The threaded code generation system is not properly initialized.");
    }
}

fn render_preview_panel_threaded(ui: &mut Ui, world: &mut World, codegen_state: &mut crate::events::CodeGenState, file_dialog: &mut egui_file_dialog::FileDialog) {
    ui.heading("Code Preview");
    ui.separator();
    
    let mut current_mode = get_preview_mode(ui);
    ui.horizontal(|ui| {
        ui.label("Preview Mode:");
        if ui.radio_value(&mut current_mode, PreviewMode::FullApp, "Full App").changed() {
            set_preview_mode(ui, current_mode);
            // Request code regeneration for new mode
            codegen_state.request_code_generation(crate::integration::TabKind::MainWork, match current_mode {
                PreviewMode::FullApp => crate::events::CodeGenMode::FullApp,
                PreviewMode::PanelFunction => crate::events::CodeGenMode::PanelFunction,
            });
        }
        if ui.radio_value(&mut current_mode, PreviewMode::PanelFunction, "Panel Function").changed() {
            set_preview_mode(ui, current_mode);
            // Request code regeneration for new mode
            codegen_state.request_code_generation(crate::integration::TabKind::MainWork, match current_mode {
                PreviewMode::FullApp => crate::events::CodeGenMode::FullApp,
                PreviewMode::PanelFunction => crate::events::CodeGenMode::PanelFunction,
            });
        }
    });
    
    ui.add_space(10.0);
    
    // Show generation status and time
    ui.horizontal(|ui| {
        if codegen_state.is_generating() {
            ui.spinner();
            ui.label("Generating...");
        } else if let Some(last_generation_time) = codegen_state.get_last_generation_time() {
            ui.label(format!("⏱️ Generated in {}ms", last_generation_time));
        }
    });
    
    ui.add_space(10.0);
    
    // Get the currently generated code from the threaded system
    let generated_code = codegen_state.get_generated_code(crate::integration::TabKind::MainWork, match current_mode {
        PreviewMode::FullApp => crate::events::CodeGenMode::FullApp,
        PreviewMode::PanelFunction => crate::events::CodeGenMode::PanelFunction,
    }).unwrap_or_else(|| "// Code generation in progress...".to_string());
    
    // File save controls
    ui.group(|ui| {
        ui.horizontal(|ui| {
            ui.label("Save as:");
            
            // Get/set the filename from UI memory
            let mut filename = ui.ctx().data_mut(|data| 
                data.get_temp::<String>(egui::Id::new("export_filename"))
                    .unwrap_or_else(|| match current_mode {
                        PreviewMode::FullApp => "app.rs".to_string(),
                        PreviewMode::PanelFunction => "ui_panel.rs".to_string(),
                    })
            );
            
            // Text input for filename
            if ui.text_edit_singleline(&mut filename).changed() {
                ui.ctx().data_mut(|data| 
                    data.insert_temp(egui::Id::new("export_filename"), filename.clone())
                );
            }
            
            // Browse button
            if ui.button("📁 Browse...").clicked() {
                file_dialog.save_file();
            }
            
            ui.separator();
            
            // Save button
            if ui.button("💾 Save").clicked() {
                let path = std::path::Path::new(&filename);
                if let Err(e) = std::fs::write(path, &generated_code) {
                    add_designer_log(world, &format!("Failed to save: {}", e));
                } else {
                    add_designer_log(world, &format!("Code saved to {}", path.display()));
                }
            }
            
            ui.separator();
            
            if ui.button("📋 Copy to Clipboard").clicked() {
                ui.ctx().copy_text(generated_code.clone());
                add_designer_log(world, "Code copied to clipboard");
            }
        });
    });
    
    // Update the file dialog and handle result
    file_dialog.update(ui.ctx());
    if let Some(path) = file_dialog.take_picked() {
        // Update the filename with the selected path
        ui.ctx().data_mut(|data| 
            data.insert_temp(egui::Id::new("export_filename"), path.to_string_lossy().to_string())
        );
    }
    
    ui.add_space(10.0);
    
    // Check if we should use syntax highlighting or fallback to plain text
    ui.horizontal(|ui| {
        ui.label("Display:");
        let mut use_highlighting = get_use_highlighting(ui);
        if ui.checkbox(&mut use_highlighting, "Syntax Highlighting").changed() {
            set_use_highlighting(ui, use_highlighting);
        }
    });
    
    ui.add_space(5.0);
    
    // Show code with optional syntax highlighting
    if get_use_highlighting(ui) {
        render_highlighted_code_cached(ui, &generated_code);
    } else {
        // Fallback to plain text for better performance
        egui::ScrollArea::vertical()
            .max_height(ui.available_height() - 20.0)
            .show(ui, |ui| {
                ui.add(
                    egui::TextEdit::multiline(&mut generated_code.as_str())
                        .font(egui::TextStyle::Monospace)
                        .code_editor()
                        .desired_width(f32::INFINITY)
                );
            });
    }
}

#[derive(PartialEq, Clone, Copy)]
pub enum PreviewMode {
    FullApp,
    PanelFunction,
}

fn get_preview_mode(ui: &egui::Ui) -> PreviewMode {
    ui.ctx().memory(|mem| {
        mem.data.get_temp(egui::Id::new("preview_mode"))
            .unwrap_or(PreviewMode::PanelFunction)
    })
}

fn set_preview_mode(ui: &egui::Ui, mode: PreviewMode) {
    ui.ctx().memory_mut(|mem| {
        mem.data.insert_temp(egui::Id::new("preview_mode"), mode);
    });
}

fn get_use_highlighting(ui: &egui::Ui) -> bool {
    ui.ctx().memory(|mem| {
        mem.data.get_temp(egui::Id::new("use_highlighting"))
            .unwrap_or(true) // Default to true with caching for good performance
    })
}

fn set_use_highlighting(ui: &egui::Ui, use_highlighting: bool) {
    ui.ctx().memory_mut(|mem| {
        mem.data.insert_temp(egui::Id::new("use_highlighting"), use_highlighting);
    });
}

// Cached highlighting - only rehighlight when code changes
fn render_highlighted_code_cached(ui: &mut egui::Ui, code: &str) {
    let code_hash = {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        let mut hasher = DefaultHasher::new();
        code.hash(&mut hasher);
        hasher.finish()
    };
    
    let cached_result = ui.ctx().memory(|mem| {
        mem.data.get_temp::<(u64, Vec<(egui::Color32, String)>)>(egui::Id::new("highlighted_code_cache"))
    });
    
    let highlighted_segments = if let Some((cached_hash, cached_segments)) = cached_result {
        if cached_hash == code_hash {
            // Use cached result
            cached_segments
        } else {
            // Code changed, re-highlight
            let highlighter = crate::syntax_highlighting::SyntaxHighlighter::new();
            let segments = highlighter.highlight_rust_code(code);
            
            // Cache the new result
            ui.ctx().memory_mut(|mem| {
                mem.data.insert_temp(egui::Id::new("highlighted_code_cache"), (code_hash, segments.clone()));
            });
            
            segments
        }
    } else {
        // First time highlighting
        let highlighter = crate::syntax_highlighting::SyntaxHighlighter::new();
        let segments = highlighter.highlight_rust_code(code);
        
        // Cache the result
        ui.ctx().memory_mut(|mem| {
            mem.data.insert_temp(egui::Id::new("highlighted_code_cache"), (code_hash, segments.clone()));
        });
        
        segments
    };
    
    // Render the cached segments
    egui::ScrollArea::vertical()
        .max_height(ui.available_height() - 20.0)
        .show(ui, |ui| {
            ui.with_layout(egui::Layout::top_down(egui::Align::LEFT), |ui| {
                ui.style_mut().wrap_mode = Some(egui::TextWrapMode::Extend);
                
                let font_id = egui::FontId::monospace(12.0);
                let mut job = egui::text::LayoutJob::default();
                
                for (color, text) in highlighted_segments {
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

fn render_group_properties(ui: &mut Ui, world: &mut World, selected_entities: &[Entity]) {
    ui.separator();
    ui.label(format!("📦 Group Properties ({} items)", selected_entities.len()));
    ui.separator();
    
    // Collect common properties from all selected entities
    let mut common_positions = Vec::new();
    let mut common_sizes = Vec::new();
    let mut common_enabled_states = Vec::new();
    
    for entity in selected_entities {
        if let Some(pos) = world.get::<UiElementPosition>(*entity) {
            common_positions.push((*entity, pos.x, pos.y));
        }
        if let Some(size) = world.get::<UiElementSize>(*entity) {
            common_sizes.push((*entity, size.width, size.height));
        }
        
        // Check if entity has an enabled property
        let enabled = if let Some(button) = world.get::<UiButton>(*entity) {
            Some(button.enabled)
        } else if let Some(text_input) = world.get::<UiTextInput>(*entity) {
            Some(text_input.enabled)
        } else if let Some(checkbox) = world.get::<UiCheckbox>(*entity) {
            Some(checkbox.enabled)
        } else if let Some(radio) = world.get::<UiRadioButton>(*entity) {
            Some(radio.enabled)
        } else if let Some(group_box) = world.get::<UiGroupBox>(*entity) {
            Some(group_box.enabled)
        } else {
            None
        };
        
        if let Some(enabled) = enabled {
            common_enabled_states.push((*entity, enabled));
        }
    }
    
    // Group Position & Size Controls
    if !common_positions.is_empty() || !common_sizes.is_empty() {
        ui.group(|ui| {
            ui.label("📍 Position & Size (Batch Edit)");
            ui.separator();
            
            if !common_positions.is_empty() {
                ui.horizontal(|ui| {
                    ui.label("Position Offset:");
                    
                    // Use persistent storage for offset values
                    let mut x_offset = ui.ctx().memory(|mem| {
                        mem.data.get_temp(egui::Id::new("group_x_offset")).unwrap_or(0.0f32)
                    });
                    let mut y_offset = ui.ctx().memory(|mem| {
                        mem.data.get_temp(egui::Id::new("group_y_offset")).unwrap_or(0.0f32)
                    });
                    
                    if ui.add(egui::DragValue::new(&mut x_offset).prefix("X: ").range(-500.0..=500.0)).changed() {
                        ui.ctx().memory_mut(|mem| {
                            mem.data.insert_temp(egui::Id::new("group_x_offset"), x_offset);
                        });
                    }
                    if ui.add(egui::DragValue::new(&mut y_offset).prefix("Y: ").range(-500.0..=500.0)).changed() {
                        ui.ctx().memory_mut(|mem| {
                            mem.data.insert_temp(egui::Id::new("group_y_offset"), y_offset);
                        });
                    }
                    
                    if ui.button("Apply Offset").clicked() {
                        for (entity, _, _) in &common_positions {
                            if let Some(mut pos) = world.get_mut::<UiElementPosition>(*entity) {
                                pos.x += x_offset;
                                pos.y += y_offset;
                            }
                        }
                        add_designer_log(world, &format!("Applied offset ({:.1}, {:.1}) to {} elements", x_offset, y_offset, common_positions.len()));
                        
                        // Reset offset values after applying
                        ui.ctx().memory_mut(|mem| {
                            mem.data.insert_temp(egui::Id::new("group_x_offset"), 0.0f32);
                            mem.data.insert_temp(egui::Id::new("group_y_offset"), 0.0f32);
                        });
                    }
                });
            }
            
            if !common_sizes.is_empty() {
                ui.horizontal(|ui| {
                    ui.label("Set Size:");
                    
                    // Use persistent storage for size values, initialize with first element's size
                    let default_width = common_sizes[0].1;
                    let default_height = common_sizes[0].2;
                    
                    let mut new_width = ui.ctx().memory(|mem| {
                        mem.data.get_temp(egui::Id::new("group_width")).unwrap_or(default_width)
                    });
                    let mut new_height = ui.ctx().memory(|mem| {
                        mem.data.get_temp(egui::Id::new("group_height")).unwrap_or(default_height)
                    });
                    
                    if ui.add(egui::DragValue::new(&mut new_width).prefix("W: ").range(10.0..=500.0)).changed() {
                        ui.ctx().memory_mut(|mem| {
                            mem.data.insert_temp(egui::Id::new("group_width"), new_width);
                        });
                    }
                    if ui.add(egui::DragValue::new(&mut new_height).prefix("H: ").range(10.0..=500.0)).changed() {
                        ui.ctx().memory_mut(|mem| {
                            mem.data.insert_temp(egui::Id::new("group_height"), new_height);
                        });
                    }
                    
                    if ui.button("Apply Size").clicked() {
                        for (entity, _, _) in &common_sizes {
                            if let Some(mut size) = world.get_mut::<UiElementSize>(*entity) {
                                size.width = new_width;
                                size.height = new_height;
                            }
                        }
                        add_designer_log(world, &format!("Set size {}x{} for {} elements", new_width, new_height, common_sizes.len()));
                    }
                });
                
                ui.horizontal(|ui| {
                    ui.label("Scale Size:");
                    
                    let mut scale_factor = ui.ctx().memory(|mem| {
                        mem.data.get_temp(egui::Id::new("group_scale")).unwrap_or(1.0f32)
                    });
                    
                    if ui.add(egui::DragValue::new(&mut scale_factor).prefix("Scale: ").range(0.1..=3.0).speed(0.1)).changed() {
                        ui.ctx().memory_mut(|mem| {
                            mem.data.insert_temp(egui::Id::new("group_scale"), scale_factor);
                        });
                    }
                    
                    if ui.button("Apply Scale").clicked() {
                        for (entity, _, _) in &common_sizes {
                            if let Some(mut size) = world.get_mut::<UiElementSize>(*entity) {
                                size.width *= scale_factor;
                                size.height *= scale_factor;
                            }
                        }
                        add_designer_log(world, &format!("Scaled {} elements by {:.1}x", common_sizes.len(), scale_factor));
                        
                        // Reset scale factor after applying
                        ui.ctx().memory_mut(|mem| {
                            mem.data.insert_temp(egui::Id::new("group_scale"), 1.0f32);
                        });
                    }
                });
            }
        });
    }
    
    // Group Enable/Disable Controls
    if !common_enabled_states.is_empty() {
        ui.group(|ui| {
            ui.label("⚙️ Enable/Disable (Batch Edit)");
            ui.separator();
            
            ui.horizontal(|ui| {
                if ui.button("✅ Enable All").clicked() {
                    for (entity, _) in &common_enabled_states {
                        // Update the enabled state for each component type
                        if let Some(mut button) = world.get_mut::<UiButton>(*entity) {
                            button.enabled = true;
                        } else if let Some(mut text_input) = world.get_mut::<UiTextInput>(*entity) {
                            text_input.enabled = true;
                        } else if let Some(mut checkbox) = world.get_mut::<UiCheckbox>(*entity) {
                            checkbox.enabled = true;
                        } else if let Some(mut radio) = world.get_mut::<UiRadioButton>(*entity) {
                            radio.enabled = true;
                        } else if let Some(mut group_box) = world.get_mut::<UiGroupBox>(*entity) {
                            group_box.enabled = true;
                        }
                    }
                    add_designer_log(world, &format!("Enabled {} elements", common_enabled_states.len()));
                }
                
                if ui.button("❌ Disable All").clicked() {
                    for (entity, _) in &common_enabled_states {
                        // Update the enabled state for each component type
                        if let Some(mut button) = world.get_mut::<UiButton>(*entity) {
                            button.enabled = false;
                        } else if let Some(mut text_input) = world.get_mut::<UiTextInput>(*entity) {
                            text_input.enabled = false;
                        } else if let Some(mut checkbox) = world.get_mut::<UiCheckbox>(*entity) {
                            checkbox.enabled = false;
                        } else if let Some(mut radio) = world.get_mut::<UiRadioButton>(*entity) {
                            radio.enabled = false;
                        } else if let Some(mut group_box) = world.get_mut::<UiGroupBox>(*entity) {
                            group_box.enabled = false;
                        }
                    }
                    add_designer_log(world, &format!("Disabled {} elements", common_enabled_states.len()));
                }
                
                if ui.button("🔄 Toggle All").clicked() {
                    for (entity, current_enabled) in &common_enabled_states {
                        let new_enabled = !current_enabled;
                        // Update the enabled state for each component type
                        if let Some(mut button) = world.get_mut::<UiButton>(*entity) {
                            button.enabled = new_enabled;
                        } else if let Some(mut text_input) = world.get_mut::<UiTextInput>(*entity) {
                            text_input.enabled = new_enabled;
                        } else if let Some(mut checkbox) = world.get_mut::<UiCheckbox>(*entity) {
                            checkbox.enabled = new_enabled;
                        } else if let Some(mut radio) = world.get_mut::<UiRadioButton>(*entity) {
                            radio.enabled = new_enabled;
                        } else if let Some(mut group_box) = world.get_mut::<UiGroupBox>(*entity) {
                            group_box.enabled = new_enabled;
                        }
                    }
                    add_designer_log(world, &format!("Toggled enabled state for {} elements", common_enabled_states.len()));
                }
            });
        });
    }
    
    // Group Deletion
    ui.group(|ui| {
        ui.label("🗑️ Danger Zone");
        ui.separator();
        
        if ui.button("🗑️ Delete All Selected").clicked() {
            for entity in selected_entities {
                world.despawn(*entity);
            }
            add_designer_log(world, &format!("Deleted {} selected elements", selected_entities.len()));
        }
    });
}


fn add_ui_element_at_position_in_tab(world: &mut World, element_type: &str, x: f32, y: f32, tab_kind: TabKind) {
    match element_type {
        "button" => {
            let button_count = world.query::<&UiButton>().iter(world).count();
            world.spawn(UiButtonBundle {
                button: UiButton {
                    label: format!("Button {}", button_count + 1),
                    clicked: false,
                    enabled: true,
                    click_time: None,
                    font_size: 14.0,
                },
                tab: UiElementTab {
                    tab_kind,
                    position: 0,
                },
                position: UiElementPosition { x, y },
                size: UiElementSize::default(),
                selected: UiElementSelected::default(),
                container: UiElementContainer { parent_group: None },
            });
            add_designer_log(world, &format!("Added Button {} at ({:.0}, {:.0})", button_count + 1, x, y));
        }
        "text_input" => {
            let input_count = world.query::<&UiTextInput>().iter(world).count();
            world.spawn(UiTextInputBundle {
                text_input: UiTextInput {
                    label: format!("Input {}", input_count + 1),
                    value: String::new(),
                    enabled: true,
                    font_size: 14.0,
                },
                tab: UiElementTab {
                    tab_kind,
                    position: 0,
                },
                position: UiElementPosition { x, y },
                size: UiElementSize { width: 200.0, height: 30.0 },
                selected: UiElementSelected::default(),
                container: UiElementContainer { parent_group: None },
            });
            add_designer_log(world, &format!("Added Text Input {} at ({:.0}, {:.0})", input_count + 1, x, y));
        }
        "checkbox" => {
            let checkbox_count = world.query::<&UiCheckbox>().iter(world).count();
            world.spawn(UiCheckboxBundle {
                checkbox: UiCheckbox {
                    label: format!("Checkbox {}", checkbox_count + 1),
                    checked: false,
                    enabled: true,
                    font_size: 14.0,
                },
                tab: UiElementTab {
                    tab_kind,
                    position: 0,
                },
                position: UiElementPosition { x, y },
                size: UiElementSize::default(),
                selected: UiElementSelected::default(),
                container: UiElementContainer { parent_group: None },
            });
            add_designer_log(world, &format!("Added Checkbox {} at ({:.0}, {:.0})", checkbox_count + 1, x, y));
        }
        "radio_button" => {
            let radio_count = world.query::<&UiRadioButton>().iter(world).count();
            world.spawn(UiRadioButtonBundle {
                radio_button: UiRadioButton {
                    label: format!("Radio {}", radio_count + 1),
                    selected: false,
                    enabled: true,
                    font_size: 14.0,
                    group_id: "default_group".to_string(),
                },
                tab: UiElementTab {
                    tab_kind,
                    position: 0,
                },
                position: UiElementPosition { x, y },
                size: UiElementSize::default(),
                selected: UiElementSelected::default(),
                container: UiElementContainer { parent_group: None },
            });
            add_designer_log(world, &format!("Added Radio Button {} at ({:.0}, {:.0})", radio_count + 1, x, y));
        }
        "group_box" => {
            let group_count = world.query::<&UiGroupBox>().iter(world).count();
            world.spawn(UiGroupBoxBundle {
                group_box: UiGroupBox {
                    label: format!("Group {}", group_count + 1),
                    enabled: true,
                    font_size: 14.0,
                    contained_widgets: Vec::new(),
                },
                tab: UiElementTab {
                    tab_kind,
                    position: 0,
                },
                position: UiElementPosition { x, y },
                size: UiElementSize { width: 200.0, height: 150.0 },
                selected: UiElementSelected::default(),
                container: UiElementContainer { parent_group: None },
            });
            add_designer_log(world, &format!("Added Group Box {} at ({:.0}, {:.0})", group_count + 1, x, y));
        }
        _ => {}
    }
}

fn handle_drag_selection_in_work_area(work_area_response: egui::Response, world: &mut World, drag_selection: &mut Option<crate::integration::DragSelection>) {
    // Start drag selection on drag_started (more reliable than clicked)
    if work_area_response.drag_started() {
        if let Some(pos) = work_area_response.interact_pointer_pos() {
            // Only clear selections if we're not holding Ctrl (for multi-select)
            let ctrl_held = work_area_response.ctx.input(|i| i.modifiers.ctrl);
            if !ctrl_held {
                clear_all_selections(world);
            }
            
            // Start new drag selection
            *drag_selection = Some(crate::integration::DragSelection {
                start_pos: pos,
                current_pos: pos,
                is_active: true,
            });
        }
    }
    
    // Also handle regular clicks for single selection
    if work_area_response.clicked() {
        let ctrl_held = work_area_response.ctx.input(|i| i.modifiers.ctrl);
        if !ctrl_held {
            clear_all_selections(world);
        }
    }
    
    // Update drag selection if dragging
    if let Some(ref mut selection) = drag_selection {
        if selection.is_active {
            if work_area_response.dragged() {
                if let Some(pos) = work_area_response.interact_pointer_pos() {
                    selection.current_pos = pos;
                    
                    // Draw selection rectangle
                    let min_pos = egui::Pos2::new(
                        selection.start_pos.x.min(selection.current_pos.x),
                        selection.start_pos.y.min(selection.current_pos.y),
                    );
                    let max_pos = egui::Pos2::new(
                        selection.start_pos.x.max(selection.current_pos.x),
                        selection.start_pos.y.max(selection.current_pos.y),
                    );
                    let selection_rect = egui::Rect::from_two_pos(min_pos, max_pos);
                    
                    // Draw selection rectangle on the work area
                    work_area_response.ctx.debug_painter().rect_stroke(
                        selection_rect,
                        0.0,
                        egui::Stroke::new(2.0, egui::Color32::BLUE),
                        egui::StrokeKind::Outside
                    );
                    work_area_response.ctx.debug_painter().rect_filled(
                        selection_rect,
                        0.0,
                        egui::Color32::BLUE.gamma_multiply(0.1)
                    );
                    
                    // Select elements within rectangle
                    select_elements_in_rect(world, selection_rect);
                }
            }
            
            // End selection on button release
            if work_area_response.drag_stopped() {
                selection.is_active = false;
                *drag_selection = None;
            }
        }
    }
}

fn select_elements_in_rect(world: &mut World, rect: egui::Rect) {
    let mut query = world.query::<(Entity, &UiElementPosition, &UiElementSize, &mut UiElementSelected)>();
    for (_, pos, size, mut selected) in query.iter_mut(world) {
        let element_rect = egui::Rect::from_min_size(
            egui::Pos2::new(pos.x, pos.y),
            egui::Vec2::new(size.width, size.height)
        );
        
        // Check if element is within selection rectangle
        selected.selected = rect.intersects(element_rect);
    }
}

fn count_selected_elements(world: &mut World) -> usize {
    let mut query = world.query::<&UiElementSelected>();
    query.iter(world).filter(|selected| selected.selected).count()
}


