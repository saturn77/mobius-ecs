use bevy_ecs::prelude::*;
use egui::Pos2;

#[derive(Default)]
pub struct SelectionState {
    pub selecting: bool,
    pub start_pos: Pos2,
    pub current_pos: Pos2,
    pub selected_entities: Vec<Entity>,
}