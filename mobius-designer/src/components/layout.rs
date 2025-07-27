use bevy_ecs::prelude::*;

#[derive(Component, Clone, Debug)]
pub struct UiElementPosition {
    pub x: f32,
    pub y: f32,
}

impl std::hash::Hash for UiElementPosition {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.x.to_bits().hash(state);
        self.y.to_bits().hash(state);
    }
}

#[derive(Component, Clone, Default, Debug)]
pub struct UiElementSize {
    pub width: f32,
    pub height: f32,
}

impl std::hash::Hash for UiElementSize {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.width.to_bits().hash(state);
        self.height.to_bits().hash(state);
    }
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

#[derive(Component, Clone, Debug, Hash)]
pub struct UiElementTab {
    pub tab_kind: crate::integration::tabs::TabKind,
    pub position: usize,
}