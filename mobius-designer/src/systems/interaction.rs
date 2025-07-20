use bevy_ecs::prelude::*;
use egui::{Pos2, Rect, Vec2};
use crate::components::*;
use crate::resources::*;
use crate::utils::*;

pub fn handle_selection_box(
    ui: &mut egui::Ui,
    world: &mut World,
    selection_state: &mut SelectionState,
) {
    let pointer = ui.ctx().input(|i| i.pointer.clone());
    
    if pointer.primary_pressed() {
        if let Some(pos) = pointer.interact_pos() {
            selection_state.selecting = true;
            selection_state.start_pos = pos;
            selection_state.current_pos = pos;
            clear_all_selections(world);
        }
    }
    
    if selection_state.selecting {
        if let Some(pos) = pointer.interact_pos() {
            selection_state.current_pos = pos;
        }
        
        if pointer.primary_released() {
            let selection_rect = Rect::from_two_pos(
                selection_state.start_pos,
                selection_state.current_pos,
            );
            
            select_elements_in_rect(world, selection_rect);
            selection_state.selecting = false;
            selection_state.selected_entities.clear();
        }
        
        draw_selection_box(ui, selection_state);
    }
}

fn draw_selection_box(ui: &mut egui::Ui, selection_state: &SelectionState) {
    let painter = ui.painter();
    let rect = Rect::from_two_pos(selection_state.start_pos, selection_state.current_pos);
    
    painter.rect_stroke(
        rect,
        0.0,
        (1.0, egui::Color32::BLUE),
        egui::StrokeKind::Outside
    );
    
    painter.rect_filled(
        rect,
        0.0,
        egui::Color32::from_rgba_unmultiplied(0, 100, 255, 50),
    );
}

fn select_elements_in_rect(world: &mut World, selection_rect: Rect) {
    let mut query = world.query::<(Entity, &UiElementPosition, &UiElementSize, &mut UiElementSelected)>();
    
    for (_entity, pos, size, mut selected) in query.iter_mut(world) {
        let element_rect = Rect::from_min_size(
            Pos2::new(pos.x, pos.y),
            Vec2::new(size.width, size.height),
        );
        
        if selection_rect.intersects(element_rect) {
            selected.selected = true;
        }
    }
}

pub fn handle_keyboard_shortcuts(
    ui: &mut egui::Ui,
    world: &mut World,
    grid_settings: &mut GridSettings,
) {
    let input = ui.ctx().input(|i| i.clone());
    
    // Toggle grid with 'G' key
    if input.key_pressed(egui::Key::G) {
        grid_settings.show_grid = !grid_settings.show_grid;
        add_designer_log(
            world,
            &format!("Grid {}", if grid_settings.show_grid { "enabled" } else { "disabled" }),
        );
    }
    
    // Delete selected elements with Delete key
    if input.key_pressed(egui::Key::Delete) {
        delete_selected_elements(world);
    }
    
    // Select all with Ctrl+A
    if input.modifiers.ctrl && input.key_pressed(egui::Key::A) {
        select_all_elements(world);
    }
    
    // Clear selection with Escape
    if input.key_pressed(egui::Key::Escape) {
        clear_all_selections(world);
    }
}

fn delete_selected_elements(world: &mut World) {
    let mut to_delete = Vec::new();
    
    let mut query = world.query::<(Entity, &UiElementSelected)>();
    for (entity, selected) in query.iter(world) {
        if selected.selected {
            to_delete.push(entity);
        }
    }
    
    let deleted_count = to_delete.len();
    for entity in to_delete {
        world.despawn(entity);
    }
    
    if deleted_count > 0 {
        add_designer_log(
            world,
            &format!("Deleted {} elements", deleted_count),
        );
    }
}

fn select_all_elements(world: &mut World) {
    let mut count = 0;
    let mut query = world.query::<&mut UiElementSelected>();
    for mut selected in query.iter_mut(world) {
        selected.selected = true;
        count += 1;
    }
    
    add_designer_log(
        world,
        &format!("Selected {} elements", count),
    );
}

pub fn handle_double_click_edit(
    ui: &mut egui::Ui,
    world: &mut World,
    edit_mode: &mut bool,
) {
    if ui.ctx().input(|i| i.pointer.button_double_clicked(egui::PointerButton::Primary)) {
        *edit_mode = !*edit_mode;
        add_designer_log(
            world,
            &format!("Edit mode {}", if *edit_mode { "enabled" } else { "disabled" }),
        );
    }
}