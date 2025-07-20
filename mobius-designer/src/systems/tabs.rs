use bevy_ecs::prelude::*;
use egui::Ui;
use crate::integration::*;
use crate::components::*;
use crate::resources::*;
use crate::utils::*;
use crate::bundles::*;

pub fn render_tab_content(ui: &mut Ui, world: &mut World, tab: &Tab, edit_mode: &mut bool, renaming_entity: &mut Option<Entity>, rename_buffer: &mut String, show_add_menu: &mut bool, add_menu_pos: &mut egui::Pos2, resizing_entity: &mut Option<Entity>, drag_selection: &mut Option<crate::integration::DragSelection>) {
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
            
            if *edit_mode {
                ui.colored_label(egui::Color32::from_rgb(0, 255, 0), "✏️ Edit Mode - Drag elements to move them");
            } else {
                ui.colored_label(egui::Color32::from_rgb(150, 150, 150), "👁 View Mode - Right-click to enable Edit Mode");
            }
            
            // First render UI elements
            crate::systems::render_dynamic_ui_elements(
                ui, 
                world, 
                *edit_mode, 
                &grid_settings,
                renaming_entity,
                rename_buffer,
                resizing_entity
            );
            
            // Handle drag selection in edit mode - only in the main work area
            if *edit_mode {
                // Create a response for the main work area only
                let work_area_response = ui.allocate_response(ui.available_size(), egui::Sense::click_and_drag());
                handle_drag_selection_in_work_area(work_area_response, world, drag_selection);
            }
            
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
                    .fixed_size(egui::Vec2::new(150.0, 0.0))
                    .show(ui.ctx(), |ui| {
                        // Edit mode toggle
                        let edit_text = if *edit_mode { "🔒 Switch to View Mode" } else { "✏️ Switch to Edit Mode" };
                        if ui.button(edit_text).clicked() {
                            *edit_mode = !*edit_mode;
                            add_designer_log(world, if *edit_mode { "Entered edit mode" } else { "Entered view mode" });
                            *show_add_menu = false;
                        }
                        
                        ui.separator();
                        
                        // Distribution options (only show if multiple elements are selected)
                        let selected_count = count_selected_elements(world);
                        if selected_count > 1 {
                            ui.label("Distribute Selected:");
                            if ui.button("↕️ Distribute Vertically").clicked() {
                                distribute_elements_vertically(world);
                                add_designer_log(world, &format!("Distributed {} elements vertically", selected_count));
                                *show_add_menu = false;
                            }
                            if ui.button("↔️ Distribute Horizontally").clicked() {
                                distribute_elements_horizontally(world);
                                add_designer_log(world, &format!("Distributed {} elements horizontally", selected_count));
                                *show_add_menu = false;
                            }
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
                    ui.horizontal(|ui| {
                        if ui.button("↕️ Distribute Vertically").clicked() {
                            distribute_elements_vertically(world);
                            add_designer_log(world, &format!("Distributed {} elements vertically", selected_count));
                        }
                        if ui.button("↔️ Distribute Horizontally").clicked() {
                            distribute_elements_horizontally(world);
                            add_designer_log(world, &format!("Distributed {} elements horizontally", selected_count));
                        }
                    });
                    
                    ui.horizontal(|ui| {
                        if ui.button("⬅️ Align Left").clicked() {
                            align_elements_left(world);
                            add_designer_log(world, &format!("Aligned {} elements to left", selected_count));
                        }
                        if ui.button("➡️ Align Right").clicked() {
                            align_elements_right(world);
                            add_designer_log(world, &format!("Aligned {} elements to right", selected_count));
                        }
                    });
                    
                    ui.horizontal(|ui| {
                        if ui.button("⬆️ Align Top").clicked() {
                            align_elements_top(world);
                            add_designer_log(world, &format!("Aligned {} elements to top", selected_count));
                        }
                        if ui.button("⬇️ Align Bottom").clicked() {
                            align_elements_bottom(world);
                            add_designer_log(world, &format!("Aligned {} elements to bottom", selected_count));
                        }
                    });
                }
            }
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
    } else {
        // Show properties for each selected entity
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

fn distribute_elements_vertically(world: &mut World) {
    let mut selected_elements = Vec::new();
    
    // Collect selected elements with their positions
    let mut query = world.query::<(Entity, &UiElementPosition, &UiElementSelected)>();
    for (entity, pos, selected) in query.iter(world) {
        if selected.selected {
            selected_elements.push((entity, pos.clone()));
        }
    }
    
    if selected_elements.len() < 2 {
        return;
    }
    
    // Sort by Y position
    selected_elements.sort_by(|a, b| a.1.y.partial_cmp(&b.1.y).unwrap());
    
    // Calculate even distribution
    let min_y = selected_elements.first().unwrap().1.y;
    let max_y = selected_elements.last().unwrap().1.y;
    let step = if selected_elements.len() > 1 {
        (max_y - min_y) / (selected_elements.len() - 1) as f32
    } else {
        0.0
    };
    
    // Update positions
    for (i, (entity, _)) in selected_elements.iter().enumerate() {
        if let Some(mut pos) = world.get_mut::<UiElementPosition>(*entity) {
            pos.y = min_y + (i as f32 * step);
        }
    }
}

fn distribute_elements_horizontally(world: &mut World) {
    let mut selected_elements = Vec::new();
    
    // Collect selected elements with their positions
    let mut query = world.query::<(Entity, &UiElementPosition, &UiElementSelected)>();
    for (entity, pos, selected) in query.iter(world) {
        if selected.selected {
            selected_elements.push((entity, pos.clone()));
        }
    }
    
    if selected_elements.len() < 2 {
        return;
    }
    
    // Sort by X position
    selected_elements.sort_by(|a, b| a.1.x.partial_cmp(&b.1.x).unwrap());
    
    // Calculate even distribution
    let min_x = selected_elements.first().unwrap().1.x;
    let max_x = selected_elements.last().unwrap().1.x;
    let step = if selected_elements.len() > 1 {
        (max_x - min_x) / (selected_elements.len() - 1) as f32
    } else {
        0.0
    };
    
    // Update positions
    for (i, (entity, _)) in selected_elements.iter().enumerate() {
        if let Some(mut pos) = world.get_mut::<UiElementPosition>(*entity) {
            pos.x = min_x + (i as f32 * step);
        }
    }
}

fn align_elements_left(world: &mut World) {
    let mut selected_elements = Vec::new();
    
    // Collect selected elements
    let mut query = world.query::<(Entity, &UiElementPosition, &UiElementSelected)>();
    for (entity, pos, selected) in query.iter(world) {
        if selected.selected {
            selected_elements.push((entity, pos.x));
        }
    }
    
    if selected_elements.len() < 2 {
        return;
    }
    
    // Find leftmost position
    let leftmost_x = selected_elements.iter().map(|(_, x)| *x).fold(f32::INFINITY, f32::min);
    
    // Align all to leftmost position
    for (entity, _) in selected_elements {
        if let Some(mut pos) = world.get_mut::<UiElementPosition>(entity) {
            pos.x = leftmost_x;
        }
    }
}

fn align_elements_right(world: &mut World) {
    let mut selected_elements = Vec::new();
    
    // Collect selected elements with their positions and sizes
    let mut query = world.query::<(Entity, &UiElementPosition, &UiElementSize, &UiElementSelected)>();
    for (entity, pos, size, selected) in query.iter(world) {
        if selected.selected {
            selected_elements.push((entity, pos.x + size.width, size.width));
        }
    }
    
    if selected_elements.len() < 2 {
        return;
    }
    
    // Find rightmost position
    let rightmost_x = selected_elements.iter().map(|(_, x, _)| *x).fold(f32::NEG_INFINITY, f32::max);
    
    // Align all to rightmost position
    for (entity, _, width) in selected_elements {
        if let Some(mut pos) = world.get_mut::<UiElementPosition>(entity) {
            pos.x = rightmost_x - width;
        }
    }
}

fn align_elements_top(world: &mut World) {
    let mut selected_elements = Vec::new();
    
    // Collect selected elements
    let mut query = world.query::<(Entity, &UiElementPosition, &UiElementSelected)>();
    for (entity, pos, selected) in query.iter(world) {
        if selected.selected {
            selected_elements.push((entity, pos.y));
        }
    }
    
    if selected_elements.len() < 2 {
        return;
    }
    
    // Find topmost position
    let topmost_y = selected_elements.iter().map(|(_, y)| *y).fold(f32::INFINITY, f32::min);
    
    // Align all to topmost position
    for (entity, _) in selected_elements {
        if let Some(mut pos) = world.get_mut::<UiElementPosition>(entity) {
            pos.y = topmost_y;
        }
    }
}

fn align_elements_bottom(world: &mut World) {
    let mut selected_elements = Vec::new();
    
    // Collect selected elements with their positions and sizes
    let mut query = world.query::<(Entity, &UiElementPosition, &UiElementSize, &UiElementSelected)>();
    for (entity, pos, size, selected) in query.iter(world) {
        if selected.selected {
            selected_elements.push((entity, pos.y + size.height, size.height));
        }
    }
    
    if selected_elements.len() < 2 {
        return;
    }
    
    // Find bottommost position
    let bottommost_y = selected_elements.iter().map(|(_, y, _)| *y).fold(f32::NEG_INFINITY, f32::max);
    
    // Align all to bottommost position
    for (entity, _, height) in selected_elements {
        if let Some(mut pos) = world.get_mut::<UiElementPosition>(entity) {
            pos.y = bottommost_y - height;
        }
    }
}

