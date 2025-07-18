# Mobius UI Designer

A Qt Designer-like visual UI designer for creating egui applications using the Mobius ECS framework.

## Features

- **Visual UI Design**: Drag and drop UI elements (buttons, text inputs, checkboxes, radio buttons, group boxes)
- **Qt Designer-like Experience**: Right-click to rename, drag to move, resize with handles
- **Edit Mode**: Double-click to toggle edit mode or use the menu
- **Grid System**: Press 'G' to toggle grid visibility and snap-to-grid
- **Group Boxes**: Create containers that group widgets together (like Qt Designer)
- **Project Export**: Save your designed UI as a complete Cargo project
- **Dockable Interface**: Flexible window layout with tabs
- **Real-time Preview**: See your UI as you design it

## Controls

### Keyboard Shortcuts
- **G**: Toggle grid system (enables grid if not already enabled)
- **Double-click**: Toggle edit mode

### Mouse Controls
- **Drag**: Move UI elements (in edit mode)
- **Right-click**: Context menu for renaming/deleting elements
- **Resize handles**: Drag the corners to resize elements (in edit mode)

### Menu Options
- **Ui Designer → Add [Element]**: Add new UI elements
- **Ui Designer → Clear All UI Elements**: Remove all elements (edit mode only)
- **File → Save Project**: Export as runnable Cargo project

## UI Elements

1. **Button**: Clickable buttons with customizable labels
2. **Text Input**: Text input fields with labels  
3. **Checkbox**: Toggle checkboxes
4. **Radio Button**: Mutually exclusive radio buttons
5. **Group Box**: Container for grouping widgets (like Qt Designer)

## Grid System

- **Grid Spacing**: Adjustable from 5-100 pixels
- **Snap to Grid**: Automatically align elements to grid points
- **Visual Feedback**: Shows grid dots when enabled
- **Smart Handling**: Press 'G' to instantly enable and show grid

## Group Box Functionality

Group boxes work like Qt Designer:
- Create containers for organizing widgets
- Widgets inside group boxes move together
- Visual grouping with labeled frames
- Resizable containers

## Getting Started

1. Run the designer:
   ```bash
   cargo run
   ```

2. Enable edit mode (double-click or menu)

3. Add UI elements from the "Ui Designer" menu

4. Use 'G' to enable grid for precise alignment

5. Right-click elements to rename or customize

6. Save your design as a project when ready

## Project Structure

The designer creates complete Cargo projects with:
- `Cargo.toml` with all necessary dependencies
- `src/main.rs` with your designed UI
- `README.md` with usage instructions

Generated projects are standalone and can be run immediately with `cargo run`.