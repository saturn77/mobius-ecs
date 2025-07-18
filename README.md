<div align="center">

# mobius-ecs

*ECS-based UI framework for egui applications*

[![Version](https://img.shields.io/badge/version-0.1.0-blue)](https://github.com/saturn77/mobius-ecs)
[![Author](https://img.shields.io/badge/author-saturn77-orange)](https://github.com/saturn77)

[![egui](https://img.shields.io/badge/egui-0.32-blue)](https://github.com/emilk/egui)
[![bevy_ecs](https://img.shields.io/badge/bevy__ecs-0.14-blue)](https://github.com/bevyengine/bevy)
[![egui_dock](https://img.shields.io/badge/egui__dock-0.17.0-blue)](https://github.com/luca-della-vedova/egui_dock)

</div>

An **exploratory** approach / framework that combines Entity Component System (ECS) architecture with egui for building structured, template-based egui GUI applications. 

The architectural approach is to provide templating or meta-templating, allowing developers to create complex UIs with minimal boilerplate. It leverages the flexibility of ECS to enable modular, parallel processing of UI components, making it suitable for both simple and complex applications.

The next step is to enhance the framework with a visual designer similar to Qt Designer, enabling drag-and-drop UI creation. This will allow developers to build interfaces visually and generate complete projects from their designs. An initial exploratory version of the designer is included, allowing for introspection and modification of UI components.

## Goals

The overall goal of mobius-ecs is to provide a powerful, flexible framework for building GUI applications using ECS principles. Key objectives include:
- **Modular UI Components**: Create reusable, composable UI components that can be easily integrated into applications.This builds upon the egui_mobius ethos, which is to create reusable, modular UI components that can be easily integrated into applications.
  
- **Template System**: Develop a robust templating system that allows developers to define UI structures and behaviors declaratively.
- **Visual Designer**: Implement a visual design tool that simplifies UI creation and allows for real-time preview and editing of interfaces.
- **ECS Integration**: Leverage the ECS architecture to enable data-driven UI development, where UI components can be dynamically created and modified based on application state.
- **egui Foundation**: Build on the egui framework to provide a modern, responsive UI experience with support for advanced features like docking, drag-and-drop, and real-time updates. egui is considered best in class for Rust GUI development, providing a simple and efficient way to create **Enterprise Level, Desktop** applications within the Rust ecosystem. 


## Features

- **ECS Architecture**: Built on `bevy_ecs` for modular, data-driven UI development
- **Pre-built Templates**: Ready-to-use templates for common application patterns
- **Visual Designer**: Qt Designer-like tool for drag-and-drop UI creation
- **Component Library**: Extensive set of reusable UI components
- **Project Generation**: Automatically generate complete Cargo projects from templates

## Getting Started

1. Clone the repository:
   ```bash
   git clone https://github.com/saturn77/mobius-ecs.git
   cd mobius-ecs
   ```

2. Build the project:
   ```bash
   cargo build --release
   ```

3. Run the examples:
   ```bash
   # Visual UI Designer (introspect and modify UI)
   cargo run --example designer

   # Basic demo with template switching
   cargo run --example demo
   
   # Advanced windowing with dockable panels
   cargo run --example windows
   
   ```

## Examples

### Basic Demo (`demo.rs`)
Demonstrates the core framework features:
- Template switching between Gerber viewer and text editor
- Tab-based navigation system
- Interactive event logging with colored output
- Settings panel with timezone and localization

### Windows Example (`windows.rs`)
Advanced UI layout capabilities:
- Dockable window system using `egui_dock`
- Multiple panel layouts (sidebar, main area, bottom panel)
- ECS world inspector for live introspection
- Floating tool windows

### Visual Designer (`designer.rs`)
A comprehensive UI design tool:
- **Drag-and-drop UI creation**: Build interfaces visually
- **Live preview**: See your UI in real-time
- **Grid alignment system**: Snap elements to grid for precise layouts
- **Project export**: Generate complete Cargo projects from your designs
- **ECS introspection**: Inspect and modify component properties

To run the designer:
```bash
cargo run --example designer
```

## Core Components

The framework provides several pre-built ECS components:

- `MobiusApp` - Main application container
- `MainWorkArea` - Primary workspace for content
- `SettingsPanel` - Configuration panel with timezone, units, and localization
- `EventLoggerPanel` - Event logging with severity levels
- `ControlsPanel` - Customizable control buttons
- `GenericTab` - Extensible tab system for content organization

## Project Structure

```
mobius-ecs/
├── src/
│   └── lib.rs              # Core library functionality
├── examples/
│   ├── demo.rs             # Basic framework demo
│   ├── windows.rs          # Advanced windowing example
│   └── designer.rs           # Visual UI Designer
```

## Creating Your Own Project

Use the built-in project generation:

```rust
use mobius_ecs::generate_mobius_project;

// Generate a new project
generate_mobius_project("my_app", "/path/to/project");
```

This creates a complete Cargo project with:
- Pre-configured dependencies
- Basic application structure
- Example UI components
- Build and run scripts

## Dependencies

- `egui` & `eframe` (0.32) - Core GUI framework
- `bevy_ecs` (0.14) - Entity Component System
- `serde` - Serialization support
- `chrono` & `chrono-tz` - Time and timezone handling
- `egui_dock` (0.17.0) - Docking system
- `image` (0.25.6) - Image processing

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please feel free to submit issues or pull requests.

## Author

Created and maintained by [saturn77](https://github.com/saturn77)
