use bevy_ecs::prelude::*;
use crate::components::*;
use crate::resources::*;
use crate::utils::*;

pub fn distribute_items_vertically(world: &mut World, settings: &DistributionSettings) {
    let mut selected_entities = Vec::new();
    
    // Collect selected entities with their positions
    let mut query = world.query::<(Entity, &UiElementPosition, &UiElementSelected)>();
    for (entity, pos, selected) in query.iter(world) {
        if selected.selected {
            selected_entities.push((entity, pos.y));
        }
    }
    
    if selected_entities.len() < 2 {
        add_designer_log(
            world,
            "Need at least 2 selected elements to distribute",
        );
        return;
    }
    
    // Sort by Y position
    selected_entities.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
    
    // Calculate distribution
    let start_y = selected_entities[0].1;
    let spacing = settings.vertical_spacing;
    
    // Apply new positions
    for (i, (entity, _)) in selected_entities.iter().enumerate() {
        let new_y = start_y + (i as f32 * spacing);
        
        if let Some(mut pos) = world.get_mut::<UiElementPosition>(*entity) {
            pos.y = new_y;
        }
    }
    
    add_designer_log(
        world,
        &format!("Distributed {} elements vertically", selected_entities.len()),
    );
}

pub fn distribute_items_horizontally(world: &mut World, settings: &DistributionSettings) {
    let mut selected_entities = Vec::new();
    
    // Collect selected entities with their positions
    let mut query = world.query::<(Entity, &UiElementPosition, &UiElementSelected)>();
    for (entity, pos, selected) in query.iter(world) {
        if selected.selected {
            selected_entities.push((entity, pos.x));
        }
    }
    
    if selected_entities.len() < 2 {
        add_designer_log(
            world,
            "Need at least 2 selected elements to distribute",
        );
        return;
    }
    
    // Sort by X position
    selected_entities.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
    
    // Calculate distribution
    let start_x = selected_entities[0].1;
    let spacing = settings.horizontal_spacing;
    
    // Apply new positions
    for (i, (entity, _)) in selected_entities.iter().enumerate() {
        let new_x = start_x + (i as f32 * spacing);
        
        if let Some(mut pos) = world.get_mut::<UiElementPosition>(*entity) {
            pos.x = new_x;
        }
    }
    
    add_designer_log(
        world,
        &format!("Distributed {} elements horizontally", selected_entities.len()),
    );
}

pub fn align_selected_elements_left(world: &mut World) {
    let mut left_most_x = f32::MAX;
    let mut selected_entities = Vec::new();
    
    // Find leftmost position and collect selected entities
    let mut query = world.query::<(Entity, &UiElementPosition, &UiElementSelected)>();
    for (entity, pos, selected) in query.iter(world) {
        if selected.selected {
            left_most_x = left_most_x.min(pos.x);
            selected_entities.push(entity);
        }
    }
    
    if selected_entities.is_empty() {
        return;
    }
    
    // Align all selected elements to the leftmost position
    for entity in selected_entities.iter() {
        if let Some(mut pos) = world.get_mut::<UiElementPosition>(*entity) {
            pos.x = left_most_x;
        }
    }
    
    add_designer_log(
        world,
        &format!("Aligned {} elements to left", selected_entities.len()),
    );
}

pub fn align_selected_elements_right(world: &mut World) {
    let mut right_most_x = f32::MIN;
    let mut selected_entities = Vec::new();
    
    // Find rightmost position and collect selected entities
    let mut query = world.query::<(Entity, &UiElementPosition, &UiElementSize, &UiElementSelected)>();
    for (entity, pos, size, selected) in query.iter(world) {
        if selected.selected {
            right_most_x = right_most_x.max(pos.x + size.width);
            selected_entities.push((entity, size.width));
        }
    }
    
    if selected_entities.is_empty() {
        return;
    }
    
    // Align all selected elements to the rightmost position
    for (entity, width) in selected_entities.iter() {
        if let Some(mut pos) = world.get_mut::<UiElementPosition>(*entity) {
            pos.x = right_most_x - width;
        }
    }
    
    add_designer_log(
        world,
        &format!("Aligned {} elements to right", selected_entities.len()),
    );
}

pub fn align_selected_elements_top(world: &mut World) {
    let mut top_most_y = f32::MAX;
    let mut selected_entities = Vec::new();
    
    // Find topmost position and collect selected entities
    let mut query = world.query::<(Entity, &UiElementPosition, &UiElementSelected)>();
    for (entity, pos, selected) in query.iter(world) {
        if selected.selected {
            top_most_y = top_most_y.min(pos.y);
            selected_entities.push(entity);
        }
    }
    
    if selected_entities.is_empty() {
        return;
    }
    
    // Align all selected elements to the topmost position
    for entity in selected_entities.iter() {
        if let Some(mut pos) = world.get_mut::<UiElementPosition>(*entity) {
            pos.y = top_most_y;
        }
    }
    
    add_designer_log(
        world,
        &format!("Aligned {} elements to top", selected_entities.len()),
    );
}

pub fn align_selected_elements_bottom(world: &mut World) {
    let mut bottom_most_y = f32::MIN;
    let mut selected_entities = Vec::new();
    
    // Find bottommost position and collect selected entities
    let mut query = world.query::<(Entity, &UiElementPosition, &UiElementSize, &UiElementSelected)>();
    for (entity, pos, size, selected) in query.iter(world) {
        if selected.selected {
            bottom_most_y = bottom_most_y.max(pos.y + size.height);
            selected_entities.push((entity, size.height));
        }
    }
    
    if selected_entities.is_empty() {
        return;
    }
    
    // Align all selected elements to the bottommost position
    for (entity, height) in selected_entities.iter() {
        if let Some(mut pos) = world.get_mut::<UiElementPosition>(*entity) {
            pos.y = bottom_most_y - height;
        }
    }
    
    add_designer_log(
        world,
        &format!("Aligned {} elements to bottom", selected_entities.len()),
    );
}

pub fn arrange_elements_in_column(world: &mut World) {
    arrange_elements_in_column_with_spacing(world, 20.0)
}

pub fn arrange_elements_in_row(world: &mut World) {
    arrange_elements_in_row_with_spacing(world, 20.0)
}

pub fn arrange_elements_in_column_with_spacing(world: &mut World, spacing: f32) {
    let mut selected_entities = Vec::new();
    
    // Collect selected entities with their positions
    let mut query = world.query::<(Entity, &UiElementPosition, &UiElementSize, &UiElementSelected)>();
    for (entity, pos, size, selected) in query.iter(world) {
        if selected.selected {
            selected_entities.push((entity, pos.x, pos.y, size.height));
        }
    }
    
    if selected_entities.len() < 2 {
        add_designer_log(
            world,
            "Need at least 2 selected elements for column layout",
        );
        return;
    }
    
    // Sort by Y position (top to bottom)
    selected_entities.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap());
    
    // Find the leftmost X position to align the column
    let left_x = selected_entities.iter().map(|(_, x, _, _)| *x).fold(f32::INFINITY, f32::min);
    
    // Arrange elements in column
    let mut current_y = selected_entities[0].2; // Start at the topmost element's Y
    
    for (entity, _, _, height) in selected_entities.iter() {
        if let Some(mut pos) = world.get_mut::<UiElementPosition>(*entity) {
            pos.x = left_x; // Align to left edge
            pos.y = current_y;
        }
        current_y += height + spacing; // Move down by element height plus spacing
    }
    
    add_designer_log(
        world,
        &format!("Arranged {} elements in column layout", selected_entities.len()),
    );
}

pub fn arrange_elements_in_row_with_spacing(world: &mut World, spacing: f32) {
    let mut selected_entities = Vec::new();
    
    // Collect selected entities with their positions
    let mut query = world.query::<(Entity, &UiElementPosition, &UiElementSize, &UiElementSelected)>();
    for (entity, pos, size, selected) in query.iter(world) {
        if selected.selected {
            selected_entities.push((entity, pos.x, pos.y, size.width));
        }
    }
    
    if selected_entities.len() < 2 {
        add_designer_log(
            world,
            "Need at least 2 selected elements for row layout",
        );
        return;
    }
    
    // Sort by X position (left to right)
    selected_entities.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
    
    // Find the topmost Y position to align the row
    let top_y = selected_entities.iter().map(|(_, _, y, _)| *y).fold(f32::INFINITY, f32::min);
    
    // Arrange elements in row
    let mut current_x = selected_entities[0].1; // Start at the leftmost element's X
    
    for (entity, _, _, width) in selected_entities.iter() {
        if let Some(mut pos) = world.get_mut::<UiElementPosition>(*entity) {
            pos.x = current_x;
            pos.y = top_y; // Align to top edge
        }
        current_x += width + spacing; // Move right by element width plus spacing
    }
    
    add_designer_log(
        world,
        &format!("Arranged {} elements in row layout", selected_entities.len()),
    );
}