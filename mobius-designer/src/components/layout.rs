use bevy_ecs::prelude::*;

#[derive(Component, Clone)]
pub struct UiElementPosition {
    pub x: f32,
    pub y: f32,
}

#[derive(Component, Clone, Default)]
pub struct UiElementSize {
    pub width: f32,
    pub height: f32,
}

#[derive(Component, Clone)]
pub struct UiElementDragging {
    pub offset_x: f32,
    pub offset_y: f32,
}

#[derive(Component, Clone)]
pub struct UiElementContainer {
    pub parent_group: Option<Entity>,
}

#[derive(Component, Clone, Default)]
pub struct UiElementSelected {
    pub selected: bool,
}

#[derive(Component, Clone)]
pub struct UiElementTab {
    pub tab_kind: crate::integration::tabs::TabKind,
    pub position: usize,
}