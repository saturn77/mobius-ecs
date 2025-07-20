use egui_dock::TabViewer;
use bevy_ecs::prelude::*;
use egui::Ui;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Tab {
    pub name: String,
    pub kind: TabKind,
    pub id: usize,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum TabKind {
    MainWork,
    Settings,
    EventLogger,
    Controls,
    Inspector,
}

#[derive(Default)]
pub struct MobiusTabViewer {
    pub world_ptr: Option<*mut World>,
    pub edit_mode: bool,
    pub renaming_entity: Option<Entity>,
    pub rename_buffer: String,
    pub show_add_menu: bool,
    pub add_menu_pos: egui::Pos2,
    pub resizing_entity: Option<Entity>,
    pub drag_selection: Option<DragSelection>,
}

#[derive(Default)]
pub struct DragSelection {
    pub start_pos: egui::Pos2,
    pub current_pos: egui::Pos2,
    pub is_active: bool,
}

impl MobiusTabViewer {
    pub fn new() -> Self {
        Self { 
            world_ptr: None, 
            edit_mode: false,
            renaming_entity: None,
            rename_buffer: String::new(),
            show_add_menu: false,
            add_menu_pos: egui::Pos2::ZERO,
            resizing_entity: None,
            drag_selection: None,
        }
    }
    
    pub fn set_world(&mut self, world: *mut World) {
        self.world_ptr = Some(world);
    }
}

impl TabViewer for MobiusTabViewer {
    type Tab = Tab;

    fn title(&mut self, tab: &mut Self::Tab) -> egui::WidgetText {
        tab.name.clone().into()
    }

    fn ui(&mut self, ui: &mut Ui, tab: &mut Self::Tab) {
        if let Some(world_ptr) = self.world_ptr {
            let world = unsafe { &mut *world_ptr };
            crate::systems::tabs::render_tab_content(ui, world, tab, &mut self.edit_mode, &mut self.renaming_entity, &mut self.rename_buffer, &mut self.show_add_menu, &mut self.add_menu_pos, &mut self.resizing_entity, &mut self.drag_selection);
        }
    }
}