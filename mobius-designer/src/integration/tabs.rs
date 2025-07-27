use egui_dock::TabViewer;
use bevy_ecs::prelude::*;
use egui::Ui;

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Tab {
    pub name: String,
    pub kind: TabKind,
    pub id: usize,
}

#[derive(Clone, Debug, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
pub enum TabKind {
    MainWork,
    Settings,
    EventLogger,
    Controls,
    Inspector,
    Preview,
}

pub struct MobiusTabViewer {
    pub world_ptr: Option<*mut World>,
    pub codegen_state_ptr: Option<*mut crate::events::CodeGenState>,
    pub renaming_entity: Option<Entity>,
    pub rename_buffer: String,
    pub show_add_menu: bool,
    pub add_menu_pos: egui::Pos2,
    pub resizing_entity: Option<Entity>,
    pub drag_selection: Option<DragSelection>,
    pub file_dialog: egui_file_dialog::FileDialog,
}

impl Default for MobiusTabViewer {
    fn default() -> Self {
        Self {
            world_ptr: None,
            codegen_state_ptr: None,
            renaming_entity: None,
            rename_buffer: String::new(),
            show_add_menu: false,
            add_menu_pos: egui::Pos2::ZERO,
            resizing_entity: None,
            drag_selection: None,
            file_dialog: egui_file_dialog::FileDialog::new(),
        }
    }
}

#[derive(Default)]
pub struct DragSelection {
    pub start_pos: egui::Pos2,
    pub current_pos: egui::Pos2,
    pub is_active: bool,
}

impl MobiusTabViewer {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn set_world(&mut self, world: *mut World) {
        self.world_ptr = Some(world);
    }
    
    pub fn set_codegen_state(&mut self, codegen_state: *mut crate::events::CodeGenState) {
        self.codegen_state_ptr = Some(codegen_state);
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
            let codegen_state = if let Some(codegen_state_ptr) = self.codegen_state_ptr {
                Some(unsafe { &mut *codegen_state_ptr })
            } else {
                None
            };
            crate::systems::tabs::render_tab_content(ui, world, tab, &mut self.renaming_entity, &mut self.rename_buffer, &mut self.show_add_menu, &mut self.add_menu_pos, &mut self.resizing_entity, &mut self.drag_selection, codegen_state, &mut self.file_dialog);
        }
    }
}