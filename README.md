<div align="center">

# mobius-designer

*Visual UI Designer for egui applications with ECS architecture*

[![Version](https://img.shields.io/badge/version-0.2.0-blue)](https://github.com/saturn77/mobius-ecs)
[![Author](https://img.shields.io/badge/author-saturn77-orange)](https://github.com/saturn77)

[![egui](https://img.shields.io/badge/egui-0.32-blue)](https://github.com/emilk/egui)
[![bevy_ecs](https://img.shields.io/badge/bevy__ecs-0.16.1-blue)](https://github.com/bevyengine/bevy)
[![egui_mobius](https://img.shields.io/badge/egui__mobius-0.3.0--alpha.32-blue)](https://github.com/saturn77/egui_mobius)

</div>

**Mobius Designer** is a visual UI design tool for egui applications, similar to Qt Designer. It combines Entity Component System (ECS) architecture with egui to enable drag-and-drop UI creation with real-time code generation for Rust/egui applications.

Built on the mobius-ecs framework, the designer provides a complete solution for building structured, template-based GUI applications with minimal boilerplate. It features advanced layout tools, alignment controls, distribution options, and generates production-ready egui code with integrated signals/slots for thread-safe communication.

## Goals

The overall goal of mobius-ecs is to provide a powerful, flexible framework for building GUI applications using ECS principles. Key objectives include:
- **Modular UI Components**: Create reusable, composable UI components that can be easily integrated into applications.This builds upon the egui_mobius ethos, which is to create reusable, modular UI components that can be easily integrated into applications.
  
- **Template System**: Develop a robust templating system that allows developers to define UI structures and behaviors declaratively.
- **Visual Designer**: Implement a visual design tool that simplifies UI creation and allows for real-time preview and editing of interfaces.
- **ECS Integration**: Leverage the ECS architecture to enable data-driven UI development, where UI components can be dynamically created and modified based on application state.
- **egui Foundation**: Build on the egui framework to provide a modern, responsive UI experience with support for advanced features like docking, drag-and-drop, and real-time updates. egui is considered best in class for Rust GUI development, providing a simple and efficient way to create **Enterprise Level, Desktop** applications within the Rust ecosystem. 


## Key Features

### Visual Designer Capabilities
- **Drag-and-drop UI creation**: Build interfaces visually with intuitive controls
- **Real-time code generation**: Automatically generates Rust/egui code as you design
- **Advanced alignment tools**: Vertical and horizontal alignment with distribution controls
- **Grid snapping system**: Precise layout control with configurable grid
- **Live preview**: See your UI in real-time with instant updates
- **egui_mobius signals/slots**: Thread-safe communication for generated code

### Framework Features
- **ECS Architecture**: Built on `bevy_ecs` for modular, data-driven UI development
- **Component library**: Extensive set of reusable UI components
- **Template system**: Ready-to-use templates for common application patterns
- **Project export**: Generate complete Cargo projects from your designs
- **Hot-reload support**: Instant UI updates during development

## Getting Started

### Quick Start - Run Mobius Designer

1. Clone the repository:
   ```bash
   git clone https://github.com/saturn77/mobius-ecs.git
   cd mobius-ecs
   ```

2. Run the designer directly:
   ```bash
   cd mobius-designer
   cargo run --release
   ```

### Building from Source

1. Build the entire project:
   ```bash
   cargo build --release
   ```

2. Run different components:
   ```bash
   # Run the main Mobius Designer
   cd mobius-designer && cargo run --release

   # Or run examples from the root directory:
   cargo run --example designer    # Simplified designer example
   cargo run --example demo        # Basic framework demo
   cargo run --example windows     # Advanced windowing demo
   ```

## Examples

### Mobius Designer (`mobius-designer/`)
The main visual UI design application featuring:
- **Advanced Designer Features**:
  - Vertical alignment and distribution controls
  - Horizontal alignment and distribution controls  
  - Real-time code generation for ui_panels in Rust/egui
  - egui_mobius signals/slots integration for threading
  - Comprehensive control panel with all design tools
- **Visual Design**: Drag-and-drop interface building
- **Code Generation**: Automatic Rust/egui code generation
- **Live Preview**: Real-time UI preview as you design
- **Project Export**: Generate complete Cargo projects

To run the full designer:
```bash
cd mobius-designer
cargo run --release
```

### Designer Example (`examples/designer.rs`)
A simplified example demonstrating designer concepts:
- Basic drag-and-drop functionality
- Grid alignment system
- Component property editing
- ECS introspection capabilities

### Framework Examples
Additional examples showcasing the underlying framework:

**Basic Demo (`demo.rs`)** - Core framework features:
- Template switching between different UI layouts
- Tab-based navigation system
- Event logging with colored output
- Settings panel with timezone support

**Windows Example (`windows.rs`)** - Advanced layout capabilities:
- Dockable window system using `egui_dock`
- Multiple panel layouts
- ECS world inspector
- Floating tool windows

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
├── mobius-designer/        # Main visual UI designer application
│   ├── src/
│   │   ├── codegen/        # Code generation for Rust/egui
│   │   ├── components/     # UI components and panels
│   │   ├── systems/        # ECS systems for rendering and interaction
│   │   └── integration/    # egui_mobius signals/slots integration
│   └── Cargo.toml
├── src/
│   └── lib.rs              # Core framework library
├── examples/
│   ├── designer.rs         # Simplified designer example
│   ├── demo.rs             # Basic framework demo
│   └── windows.rs          # Advanced windowing example
└── Cargo.toml
```

## Creating Your Own Project

### Using Mobius Designer

1. Launch Mobius Designer:
   ```bash
   cd mobius-designer && cargo run --release
   ```

2. Design your UI using the visual tools:
   - Drag and drop components from the panel
   - Use alignment and distribution controls
   - Configure component properties
   - Preview your UI in real-time

3. Export your project:
   - Click "Export Project" in the designer
   - Choose your project location
   - The designer generates a complete Cargo project with your UI

### Programmatic Project Generation

```rust
use mobius_ecs::generate_mobius_project;

// Generate a new project
generate_mobius_project("my_app", "/path/to/project");
```

This creates a complete Cargo project with:
- Pre-configured dependencies
- Generated UI code from your design
- egui_mobius signals/slots integration
- Build and run scripts

## Dependencies

### Core Dependencies
- `egui` & `eframe` (0.32) - Core GUI framework
- `bevy_ecs` (0.16.1) - Entity Component System
- `egui_mobius` (0.3.0-alpha.32) - Signals/slots and UI components
- `egui_dock` (0.17.0) - Docking system

### Mobius Designer Dependencies
- `syntect` (5.1) - Syntax highlighting for code generation
- `egui-file-dialog` (0.11.0) - File dialogs for project management
- `uuid` (1.0) - Unique identifiers for components
- `rand` (0.8) - Random generation utilities

### Utility Dependencies
- `serde` & `serde_json` - Serialization support
- `chrono` & `chrono-tz` - Time and timezone handling
- `image` (0.25.6) - Image processing
- `egui_tool_windows` (0.1.3) - Tool window support

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please feel free to submit issues or pull requests.

## Author

Created and maintained by [saturn77](https://github.com/saturn77)
