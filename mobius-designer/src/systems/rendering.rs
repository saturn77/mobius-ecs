use bevy_ecs::prelude::*;
use egui::{Area, Color32, Id, Order, Pos2, Rect, Response, Ui, Vec2};
use mobius_ecs::*;
use crate::components::*;
use crate::integration::*;
use crate::utils::*;
// use crate::resources::{LengthUnit, Tool}; // Now handled in tabs.rs

pub fn render_dynamic_ui_elements(
    ui: &mut Ui,
    world: &mut World,
    edit_mode: bool,
    grid_settings: &GridSettings,
    renaming_entity: &mut Option<Entity>,
    rename_buffer: &mut String,
    resizing_entity: &mut Option<Entity>,
) {
    reset_button_clicks(world);
    
    let mut updates = Vec::new();
    let mut log_messages = Vec::new();
    
    render_buttons(ui, world, edit_mode, grid_settings, &mut updates, &mut log_messages, renaming_entity, rename_buffer, resizing_entity);
    render_text_inputs(ui, world, edit_mode, grid_settings, &mut updates, &mut log_messages, resizing_entity);
    render_checkboxes(ui, world, edit_mode, grid_settings, &mut updates, &mut log_messages, resizing_entity);
    render_radio_buttons(ui, world, edit_mode, grid_settings, &mut updates, &mut log_messages, resizing_entity);
    render_group_boxes(ui, world, edit_mode, grid_settings, &mut updates, &mut log_messages, resizing_entity);
    
    apply_updates(world, updates);
    
    // Emit all log messages to event logger
    for message in log_messages {
        add_designer_log(world, &message);
    }
    
    if edit_mode {
        draw_resize_handles(ui, world);
    }
}

fn render_buttons(
    ui: &mut Ui,
    world: &mut World,
    edit_mode: bool,
    grid_settings: &GridSettings,
    updates: &mut Vec<Box<dyn FnOnce(&mut World) + Send>>,
    log_messages: &mut Vec<String>,
    renaming_entity: &mut Option<Entity>,
    rename_buffer: &mut String,
    resizing_entity: &mut Option<Entity>,
) {
    let mut query = world.query::<(Entity, &UiButton, &UiElementPosition, &UiElementSize, &UiElementSelected)>();
    let buttons: Vec<_> = query.iter(world).map(|(e, b, p, s, sel)| (e, b.clone(), p.clone(), s.clone(), sel.clone())).collect();
    
    for (entity, button, pos, size, selected) in buttons {
        let area_id = Id::new(format!("button_area_{:?}", entity));
        let area_response = Area::new(area_id)
            .order(Order::Middle)
            .fixed_pos(Pos2::new(pos.x, pos.y))
            .show(ui.ctx(), |ui| {
                if edit_mode {
                    create_edit_frame(ui, selected.selected);
                }
                
                let button_response = if *renaming_entity == Some(entity) {
                    // Show text input for renaming
                    let response = ui.text_edit_singleline(rename_buffer);
                    if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                        // Finish renaming
                        let entity_copy = entity;
                        let new_label = rename_buffer.clone();
                        log_messages.push(format!("Button renamed to '{}'", new_label));
                        updates.push(Box::new(move |world: &mut World| {
                            if let Some(mut btn) = world.get_mut::<UiButton>(entity_copy) {
                                btn.label = new_label;
                            }
                        }));
                        *renaming_entity = None;
                        rename_buffer.clear();
                    } else if response.lost_focus() {
                        // Cancel renaming
                        *renaming_entity = None;
                        rename_buffer.clear();
                    }
                    if response.gained_focus() {
                        response.request_focus();
                    }
                    response
                } else {
                    // Show normal button
                    let button_text = if button.font_size > 0.0 {
                        egui::RichText::new(&button.label).size(button.font_size)
                    } else {
                        egui::RichText::new(&button.label)
                    };
                    
                    let btn_response = if size.width > 10.0 || size.height > 10.0 {
                        // Manual button rendering with exact size
                        let actual_width = if size.width > 10.0 { size.width } else { 80.0 };
                        let actual_height = if size.height > 10.0 { size.height } else { 25.0 };
                        
                        let desired_size = Vec2::new(actual_width, actual_height);
                        let (rect, response) = ui.allocate_exact_size(desired_size, egui::Sense::click());
                        
                        if ui.is_rect_visible(rect) {
                            let visuals = ui.style().interact(&response);
                            
                            // Draw button background
                            ui.painter().rect_filled(rect, visuals.corner_radius, visuals.bg_fill);
                            ui.painter().rect_stroke(rect, visuals.corner_radius, visuals.bg_stroke, egui::StrokeKind::Outside);
                            
                            // Draw text centered using galley for proper layout
                            let text_color = if button.enabled { visuals.text_color() } else { ui.style().visuals.weak_text_color() };
                            let font_id = egui::FontId::proportional(button.font_size);
                            let galley = ui.fonts(|f| f.layout_no_wrap(button.label.clone(), font_id, text_color));
                            let text_pos = rect.center() - galley.size() * 0.5;
                            ui.painter().galley(text_pos, galley, text_color);
                        }
                        
                        response
                    } else {
                        ui.add_enabled_ui(button.enabled, |ui| {
                            ui.button(button_text)
                        }).inner
                    };
                    
                    if button.clicked {
                        ui.colored_label(Color32::GREEN, "✓");
                    }
                    
                    btn_response
                };
                
                button_response
            });
        
        if edit_mode {
            // Handle dragging directly on the button response
            let drag_response = area_response.inner.interact(egui::Sense::drag());
            if drag_response.dragged() {
                let delta = drag_response.drag_delta();
                let snap_enabled = grid_settings.snap_enabled;
                let spacing = grid_settings.spacing_pixels;
                
                // If this element is selected, move all selected elements
                if selected.selected {
                    updates.push(Box::new(move |world: &mut World| {
                        move_selected_elements(world, delta, snap_enabled, spacing);
                    }));
                } else {
                    // Move only this element
                    let entity_copy = entity;
                    updates.push(Box::new(move |world: &mut World| {
                        if let Some(mut pos) = world.get_mut::<UiElementPosition>(entity_copy) {
                            let new_pos = Pos2::new(pos.x + delta.x, pos.y + delta.y);
                            let final_pos = if snap_enabled {
                                snap_to_grid(new_pos, spacing)
                            } else {
                                new_pos
                            };
                            pos.x = final_pos.x;
                            pos.y = final_pos.y;
                        }
                    }));
                }
            }
            if drag_response.hovered() {
                ui.ctx().set_cursor_icon(egui::CursorIcon::Grab);
            }
            
            // Right-click context menu  
            area_response.inner.context_menu(|ui| {
                ui.label(format!("Button: {}", button.label));
                ui.separator();
                
                if ui.button("✏️ Rename").clicked() {
                    // Start rename mode
                    *renaming_entity = Some(entity);
                    *rename_buffer = button.label.clone();
                    ui.close_kind(egui::UiKind::Menu);
                }
                
                let resize_text = if *resizing_entity == Some(entity) { "🔒 Stop Resize" } else { "📏 Resize" };
                if ui.button(resize_text).clicked() {
                    *resizing_entity = if *resizing_entity == Some(entity) { None } else { Some(entity) };
                    ui.close_kind(egui::UiKind::Menu);
                }
                
                if ui.button("🗑️ Delete").clicked() {
                    let entity_copy = entity;
                    let button_label = button.label.clone();
                    log_messages.push(format!("Button '{}' deleted", button_label));
                    updates.push(Box::new(move |world: &mut World| {
                        world.despawn(entity_copy);
                    }));
                    ui.close_kind(egui::UiKind::Menu);
                }
            });
            
            // Show resize handles if this entity is being resized
            if edit_mode && *resizing_entity == Some(entity) {
                draw_resize_handles_for_entity(ui, area_response.inner.rect, updates, entity);
            }
        } else if area_response.inner.clicked() {
            if edit_mode {
                // In edit mode, clicking selects/deselects the element
                let entity_copy = entity;
                updates.push(Box::new(move |world: &mut World| {
                    if let Some(mut selected) = world.get_mut::<UiElementSelected>(entity_copy) {
                        // Toggle selection, or if Ctrl is held, add to selection
                        selected.selected = !selected.selected;
                    }
                }));
            } else if !button.clicked {
                // In view mode, clicking activates the button
                let entity_copy = entity;
                let button_label = button.label.clone();
                log_messages.push(format!("Button '{}' clicked", button_label));
                updates.push(Box::new(move |world: &mut World| {
                    if let Some(mut button) = world.get_mut::<UiButton>(entity_copy) {
                        button.clicked = true;
                        button.click_time = Some(std::time::Instant::now());
                    }
                }));
            }
        }
    }
}

fn render_text_inputs(
    ui: &mut Ui,
    world: &mut World,
    edit_mode: bool,
    _grid_settings: &GridSettings,
    updates: &mut Vec<Box<dyn FnOnce(&mut World) + Send>>,
    log_messages: &mut Vec<String>,
    _resizing_entity: &mut Option<Entity>,
) {
    let mut query = world.query::<(Entity, &UiTextInput, &UiElementPosition, &UiElementSize, &UiElementSelected)>();
    let inputs: Vec<_> = query.iter(world).map(|(e, t, p, s, sel)| (e, t.clone(), p.clone(), s.clone(), sel.clone())).collect();
    
    for (entity, text_input, pos, size, selected) in inputs {
        let area_id = Id::new(format!("text_input_area_{:?}", entity));
        let area_response = Area::new(area_id)
            .order(Order::Middle)
            .fixed_pos(Pos2::new(pos.x, pos.y))
            .show(ui.ctx(), |ui| {
                if edit_mode {
                    create_edit_frame(ui, selected.selected);
                }
                
                let response = ui.vertical(|ui| {
                    let label_text = if text_input.font_size > 0.0 {
                        egui::RichText::new(&text_input.label).size(text_input.font_size)
                    } else {
                        egui::RichText::new(&text_input.label)
                    };
                    ui.label(label_text);
                    
                    let mut value = text_input.value.clone();
                    let text_edit = egui::TextEdit::singleline(&mut value)
                        .desired_width(size.width.max(100.0));
                    
                    let response = ui.add_enabled_ui(text_input.enabled, |ui| {
                        ui.add(text_edit)
                    }).inner;
                    
                    if response.changed() {
                        let entity_copy = entity;
                        let input_label = text_input.label.clone();
                        let new_value = value.clone();
                        log_messages.push(format!("Text input '{}' changed to '{}'", input_label, new_value));
                        updates.push(Box::new(move |world: &mut World| {
                            if let Some(mut input) = world.get_mut::<UiTextInput>(entity_copy) {
                                input.value = value;
                            }
                        }));
                    }
                    
                    response
                }).inner;
                
                response
            });
        
        if edit_mode {
            // Handle dragging directly on the element response
            let drag_response = area_response.inner.interact(egui::Sense::drag());
            if drag_response.dragged() {
                let delta = drag_response.drag_delta();
                let entity_copy = entity;
                updates.push(Box::new(move |world: &mut World| {
                    if let Some(mut pos) = world.get_mut::<UiElementPosition>(entity_copy) {
                        let new_pos = Pos2::new(pos.x + delta.x, pos.y + delta.y);
                        pos.x = new_pos.x;
                        pos.y = new_pos.y;
                    }
                }));
            }
            if drag_response.hovered() {
                ui.ctx().set_cursor_icon(egui::CursorIcon::Grab);
            }
        }
    }
}

fn render_checkboxes(
    ui: &mut Ui,
    world: &mut World,
    edit_mode: bool,
    _grid_settings: &GridSettings,
    updates: &mut Vec<Box<dyn FnOnce(&mut World) + Send>>,
    log_messages: &mut Vec<String>,
    _resizing_entity: &mut Option<Entity>,
) {
    let mut query = world.query::<(Entity, &UiCheckbox, &UiElementPosition, &UiElementSize, &UiElementSelected)>();
    let checkboxes: Vec<_> = query.iter(world).map(|(e, c, p, s, sel)| (e, c.clone(), p.clone(), s.clone(), sel.clone())).collect();
    
    for (entity, checkbox, pos, _size, selected) in checkboxes {
        let area_id = Id::new(format!("checkbox_area_{:?}", entity));
        let area_response = Area::new(area_id)
            .order(Order::Middle)
            .fixed_pos(Pos2::new(pos.x, pos.y))
            .show(ui.ctx(), |ui| {
                if edit_mode {
                    create_edit_frame(ui, selected.selected);
                }
                
                let mut checked = checkbox.checked;
                let label_text = if checkbox.font_size > 0.0 {
                    egui::RichText::new(&checkbox.label).size(checkbox.font_size)
                } else {
                    egui::RichText::new(&checkbox.label)
                };
                
                let response = ui.add_enabled_ui(checkbox.enabled, |ui| {
                    ui.checkbox(&mut checked, label_text)
                }).inner;
                
                if response.changed() {
                    let entity_copy = entity;
                    let checkbox_label = checkbox.label.clone();
                    log_messages.push(format!("Checkbox '{}' {}", checkbox_label, if checked { "checked" } else { "unchecked" }));
                    updates.push(Box::new(move |world: &mut World| {
                        if let Some(mut cb) = world.get_mut::<UiCheckbox>(entity_copy) {
                            cb.checked = checked;
                        }
                    }));
                }
                
                response
            });
        
        if edit_mode {
            // Handle dragging directly on the element response
            let drag_response = area_response.inner.interact(egui::Sense::drag());
            if drag_response.dragged() {
                let delta = drag_response.drag_delta();
                let entity_copy = entity;
                updates.push(Box::new(move |world: &mut World| {
                    if let Some(mut pos) = world.get_mut::<UiElementPosition>(entity_copy) {
                        let new_pos = Pos2::new(pos.x + delta.x, pos.y + delta.y);
                        pos.x = new_pos.x;
                        pos.y = new_pos.y;
                    }
                }));
            }
            if drag_response.hovered() {
                ui.ctx().set_cursor_icon(egui::CursorIcon::Grab);
            }
        }
    }
}

fn render_radio_buttons(
    ui: &mut Ui,
    world: &mut World,
    edit_mode: bool,
    _grid_settings: &GridSettings,
    updates: &mut Vec<Box<dyn FnOnce(&mut World) + Send>>,
    log_messages: &mut Vec<String>,
    _resizing_entity: &mut Option<Entity>,
) {
    let mut query = world.query::<(Entity, &UiRadioButton, &UiElementPosition, &UiElementSize, &UiElementSelected)>();
    let radio_buttons: Vec<_> = query.iter(world).map(|(e, r, p, s, sel)| (e, r.clone(), p.clone(), s.clone(), sel.clone())).collect();
    
    for (entity, radio_button, pos, _size, selected) in radio_buttons {
        let area_id = Id::new(format!("radio_area_{:?}", entity));
        let area_response = Area::new(area_id)
            .order(Order::Middle)
            .fixed_pos(Pos2::new(pos.x, pos.y))
            .show(ui.ctx(), |ui| {
                if edit_mode {
                    create_edit_frame(ui, selected.selected);
                }
                
                let label_text = if radio_button.font_size > 0.0 {
                    egui::RichText::new(&radio_button.label).size(radio_button.font_size)
                } else {
                    egui::RichText::new(&radio_button.label)
                };
                
                let response = ui.add_enabled_ui(radio_button.enabled, |ui| {
                    ui.radio_value(&mut true, radio_button.selected, label_text)
                }).inner;
                
                if response.clicked() && !radio_button.selected {
                    let group_id = radio_button.group_id.clone();
                    let entity_copy = entity;
                    let radio_label = radio_button.label.clone();
                    log_messages.push(format!("Radio button '{}' selected", radio_label));
                    updates.push(Box::new(move |world: &mut World| {
                        let mut radio_query = world.query::<(Entity, &mut UiRadioButton)>();
                        for (e, mut rb) in radio_query.iter_mut(world) {
                            if rb.group_id == group_id {
                                rb.selected = e == entity_copy;
                            }
                        }
                    }));
                }
                
                response
            });
        
        if edit_mode {
            // Handle dragging directly on the element response
            let drag_response = area_response.inner.interact(egui::Sense::drag());
            if drag_response.dragged() {
                let delta = drag_response.drag_delta();
                let entity_copy = entity;
                updates.push(Box::new(move |world: &mut World| {
                    if let Some(mut pos) = world.get_mut::<UiElementPosition>(entity_copy) {
                        let new_pos = Pos2::new(pos.x + delta.x, pos.y + delta.y);
                        pos.x = new_pos.x;
                        pos.y = new_pos.y;
                    }
                }));
            }
            if drag_response.hovered() {
                ui.ctx().set_cursor_icon(egui::CursorIcon::Grab);
            }
        }
    }
}

fn render_group_boxes(
    ui: &mut Ui,
    world: &mut World,
    edit_mode: bool,
    _grid_settings: &GridSettings,
    updates: &mut Vec<Box<dyn FnOnce(&mut World) + Send>>,
    _log_messages: &mut Vec<String>,
    _resizing_entity: &mut Option<Entity>,
) {
    let mut query = world.query::<(Entity, &UiGroupBox, &UiElementPosition, &UiElementSize, &UiElementSelected)>();
    let group_boxes: Vec<_> = query.iter(world).map(|(e, g, p, s, sel)| (e, g.clone(), p.clone(), s.clone(), sel.clone())).collect();
    
    for (entity, group_box, pos, size, selected) in group_boxes {
        let area_id = Id::new(format!("groupbox_area_{:?}", entity));
        let area_response = Area::new(area_id)
            .order(Order::Middle)
            .fixed_pos(Pos2::new(pos.x, pos.y))
            .show(ui.ctx(), |ui| {
                if edit_mode {
                    create_edit_frame(ui, selected.selected);
                }
                
                let title_text = if group_box.font_size > 0.0 {
                    egui::RichText::new(&group_box.label).size(group_box.font_size)
                } else {
                    egui::RichText::new(&group_box.label)
                };
                
                let response = ui.group(|ui| {
                    ui.set_min_size(Vec2::new(size.width.max(100.0), size.height.max(50.0)));
                    ui.label(title_text)
                }).response;
                
                response
            });
        
        if edit_mode {
            // Handle dragging directly on the element response
            let drag_response = area_response.inner.interact(egui::Sense::drag());
            if drag_response.dragged() {
                let delta = drag_response.drag_delta();
                let entity_copy = entity;
                updates.push(Box::new(move |world: &mut World| {
                    if let Some(mut pos) = world.get_mut::<UiElementPosition>(entity_copy) {
                        let new_pos = Pos2::new(pos.x + delta.x, pos.y + delta.y);
                        pos.x = new_pos.x;
                        pos.y = new_pos.y;
                    }
                }));
            }
            if drag_response.hovered() {
                ui.ctx().set_cursor_icon(egui::CursorIcon::Grab);
            }
        }
    }
}

fn handle_edit_interactions(
    ui: &mut Ui,
    world: &World,
    entity: Entity,
    area_response: egui::InnerResponse<Response>,
    grid_settings: &GridSettings,
    updates: &mut Vec<Box<dyn FnOnce(&mut World) + Send>>,
) {
    let response = &area_response.response;
    
    if response.dragged() {
        handle_drag(entity, response, grid_settings, updates);
    }
    
    if response.clicked_by(egui::PointerButton::Secondary) {
        show_context_menu(ui, world, entity, updates);
    }
}

fn handle_drag(
    entity: Entity,
    response: &Response,
    grid_settings: &GridSettings,
    updates: &mut Vec<Box<dyn FnOnce(&mut World) + Send>>,
) {
    let delta = response.drag_delta();
    let snap_enabled = grid_settings.snap_enabled;
    let spacing = grid_settings.spacing_pixels;
    
    updates.push(Box::new(move |world: &mut World| {
        if let Some(mut pos) = world.get_mut::<UiElementPosition>(entity) {
            let new_pos = Pos2::new(pos.x + delta.x, pos.y + delta.y);
            let final_pos = if snap_enabled {
                snap_to_grid(new_pos, spacing)
            } else {
                new_pos
            };
            pos.x = final_pos.x;
            pos.y = final_pos.y;
        }
    }));
}

fn show_context_menu(
    ui: &mut Ui,
    _world: &World,
    _entity: Entity,
    _updates: &mut Vec<Box<dyn FnOnce(&mut World) + Send>>,
) {
    // TODO: Implement context menu with newer egui API
    // For now, just show a placeholder
    ui.label("Right-click context menu not yet implemented");
}


fn handle_rename(
    _entity: Entity,
    _updates: &mut Vec<Box<dyn FnOnce(&mut World) + Send>>,
) {
    // Placeholder for rename functionality
}

pub fn draw_grid(ui: &mut Ui, grid_settings: &GridSettings, zoom: f32) {
    if !grid_settings.show_grid {
        return;
    }
    
    let grid_status = get_grid_status(zoom, grid_settings.spacing_pixels);
    
    match grid_status {
        GridStatus::TooFine | GridStatus::TooCoarse => return,
        GridStatus::Visible(screen_spacing) => {
            let painter = ui.painter();
            let rect = ui.max_rect();
            
            let dot_color = Color32::from_gray(100);
            let dot_size = grid_settings.dot_size;
            
            let start_x = (rect.left() / screen_spacing).floor() * screen_spacing;
            let start_y = (rect.top() / screen_spacing).floor() * screen_spacing;
            
            let mut point_count = 0;
            let max_points = 2000;
            
            let mut y = start_y;
            while y <= rect.bottom() && point_count < max_points {
                let mut x = start_x;
                while x <= rect.right() && point_count < max_points {
                    painter.circle_filled(Pos2::new(x, y), dot_size, dot_color);
                    x += screen_spacing;
                    point_count += 1;
                }
                y += screen_spacing;
            }
        }
    }
}

pub fn draw_resize_handles(ui: &mut Ui, world: &mut World) {
    let mut query = world.query::<(&UiElementPosition, &UiElementSize, &UiElementSelected)>();
    
    for (pos, size, selected) in query.iter(world) {
        if selected.selected {
            let rect = Rect::from_min_size(
                Pos2::new(pos.x, pos.y),
                Vec2::new(size.width, size.height)
            );
            
            let painter = ui.painter();
            let handle_size = 6.0;
            let handle_color = Color32::BLUE;
            
            // Corner handles
            painter.circle_filled(rect.min, handle_size, handle_color);
            painter.circle_filled(Pos2::new(rect.max.x, rect.min.y), handle_size, handle_color);
            painter.circle_filled(rect.max, handle_size, handle_color);
            painter.circle_filled(Pos2::new(rect.min.x, rect.max.y), handle_size, handle_color);
            
            // Edge handles
            painter.circle_filled(Pos2::new(rect.center().x, rect.min.y), handle_size, handle_color);
            painter.circle_filled(Pos2::new(rect.max.x, rect.center().y), handle_size, handle_color);
            painter.circle_filled(Pos2::new(rect.center().x, rect.max.y), handle_size, handle_color);
            painter.circle_filled(Pos2::new(rect.min.x, rect.center().y), handle_size, handle_color);
        }
    }
}

pub fn render_tab_content_old(ui: &mut Ui, world: &mut World, tab: &Tab) {
    match tab.kind {
        TabKind::MainWork => {
            if let Ok(mut main_work_area) = world.query::<&mut MainWorkArea>().get_single_mut(world) {
                ui.text_edit_multiline(&mut main_work_area.content);
            }
        }
        TabKind::Settings => {
            // Settings panel rendering moved to tabs.rs
            ui.label("See tabs.rs for settings panel rendering");
        }
        TabKind::EventLogger => {
            if let Ok(logger) = world.query::<&EventLoggerPanel>().get_single(world) {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    for event in &logger.entries {
                        let color = match event.level {
                            LogLevel::Debug => Color32::GRAY,
                            LogLevel::Info => Color32::WHITE,
                            LogLevel::Warn => Color32::YELLOW,
                            LogLevel::Error => Color32::RED,
                        };
                        ui.colored_label(color, format!("[{}] {}", event.timestamp, event.message));
                    }
                });
            }
        }
        TabKind::Controls => {
            // Controls panel rendering moved to tabs.rs
            ui.label("See tabs.rs for controls panel rendering");
        }
        TabKind::Inspector => {
            ui.label("Inspector");
        }
    }
}

fn create_edit_frame(ui: &mut Ui, selected: bool) {
    if selected {
        let rect = ui.max_rect();
        let painter = ui.painter();
        painter.rect_stroke(rect, 0.0, (2.0, Color32::BLUE), egui::StrokeKind::Outside);
    }
}

fn apply_updates(world: &mut World, updates: Vec<Box<dyn FnOnce(&mut World) + Send>>) {
    for update in updates {
        update(world);
    }
}

// Helper function to draw resize handles for a specific entity
fn draw_resize_handles_for_entity(ui: &mut egui::Ui, rect: egui::Rect, updates: &mut Vec<Box<dyn FnOnce(&mut World) + Send>>, entity: Entity) {
    let handle_size = 10.0;
    let bottom_right = rect.right_bottom() - egui::vec2(handle_size / 2.0, handle_size / 2.0);
    let handle_rect = egui::Rect::from_center_size(bottom_right, egui::vec2(handle_size, handle_size));
    
    let handle_response = ui.interact(handle_rect, ui.id().with(entity).with("resize_handle"), egui::Sense::drag());
    
    // Draw the handle with better visibility
    ui.painter().rect_filled(handle_rect, 2.0, egui::Color32::from_rgb(0, 100, 255));
    ui.painter().rect_stroke(handle_rect, 2.0, egui::Stroke::new(1.0, egui::Color32::WHITE), egui::StrokeKind::Outside);
    
    if handle_response.dragged() {
        let new_size = rect.size() + handle_response.drag_delta();
        let min_size = egui::vec2(60.0, 25.0); // Minimum size for buttons
        let max_size = egui::vec2(300.0, 100.0); // Maximum size to prevent huge buttons
        let clamped_size = egui::vec2(
            new_size.x.clamp(min_size.x, max_size.x),
            new_size.y.clamp(min_size.y, max_size.y)
        );
        
        let entity_copy = entity;
        updates.push(Box::new(move |world: &mut World| {
            if let Some(mut size) = world.get_mut::<UiElementSize>(entity_copy) {
                size.width = clamped_size.x;
                size.height = clamped_size.y;
            }
        }));
    }
    
    if handle_response.hovered() {
        ui.ctx().set_cursor_icon(egui::CursorIcon::ResizeNwSe);
    }
    
    // Draw resize indicator
    ui.painter().text(
        handle_rect.center() + egui::vec2(0.0, -15.0),
        egui::Align2::CENTER_CENTER,
        "↘",
        egui::FontId::proportional(12.0),
        egui::Color32::WHITE,
    );
}

// Move all selected elements by the given delta
fn move_selected_elements(world: &mut World, delta: egui::Vec2, snap_enabled: bool, spacing: f32) {
    let mut query = world.query::<(&mut UiElementPosition, &UiElementSelected)>();
    for (mut pos, selected) in query.iter_mut(world) {
        if selected.selected {
            let new_pos = Pos2::new(pos.x + delta.x, pos.y + delta.y);
            let final_pos = if snap_enabled {
                snap_to_grid(new_pos, spacing)
            } else {
                new_pos
            };
            pos.x = final_pos.x;
            pos.y = final_pos.y;
        }
    }
}