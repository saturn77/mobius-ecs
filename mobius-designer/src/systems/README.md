# Rendering Systems

This module contains the rendering systems for the Mobius Designer UI framework.

## Main Components

### `rendering.rs`
Contains the main rendering functions extracted and refactored from the designer.rs example:

- **`render_dynamic_ui_elements`**: Main rendering system that handles all UI element types
- **`draw_grid`**: Renders the grid overlay with performance optimizations
- **`draw_resize_handles`**: Renders resize handles for UI elements in edit mode
- **`snap_to_grid`**: Utility function for grid snapping
- **`render_tab_content`**: Renders content for different tab types

### Supported UI Element Types

The rendering system supports the following UI element types:

1. **Buttons** (`UiButton`)
   - Auto-sizing and fixed sizing
   - Font size customization
   - Enable/disable states
   - Click states with visual feedback
   - Context menu with rename, resize, delete options

2. **Text Inputs** (`UiTextInput`)
   - Label and value editing
   - Enable/disable states
   - Context menu with rename, delete options

3. **Checkboxes** (`UiCheckbox`)
   - Font size customization
   - Enable/disable states
   - Context menu with rename, font size, delete options

4. **Radio Buttons** (`UiRadioButton`)
   - Group-based mutual exclusion
   - Font size customization
   - Enable/disable states
   - Context menu with rename, font size, delete options

5. **Group Boxes** (`UiGroupBox`)
   - Container for other widgets
   - Resizable with handles
   - Font size customization for titles
   - Context menu with rename, resize, delete options

### Features

- **Edit Mode**: Drag elements to reposition, show resize handles, enable context menus
- **View Mode**: Interactive elements work normally, no dragging or editing
- **Grid System**: Visual grid with snapping functionality
- **Context Menus**: Right-click menus for each element type
- **Performance Optimizations**: Grid rendering optimizations, batched updates
- **Absolute Positioning**: Uses egui Areas for precise positioning

### Usage

```rust
use mobius_designer::systems::rendering::*;

// Main rendering call
let result = render_dynamic_ui_elements(
    ui,
    world,
    &tab_kind,
    edit_mode,
    renaming_entity,
    rename_buffer,
    resizing_entity,
);

// Grid rendering
draw_grid(ui, grid_settings);

// Tab content rendering
render_tab_content(ui, world, tab);
```

### Architecture

The rendering system is designed to work with the Bevy ECS architecture:
- Components define UI element data
- Systems query and render components
- World updates are batched for performance
- Mutable borrows are handled safely