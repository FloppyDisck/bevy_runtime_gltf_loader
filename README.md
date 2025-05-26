# bevy_runtime_gltf_loader

A Bevy plugin for loading GLTF models at runtime using JSON configuration files. This crate allows you to define
collections of 3D model parts in JSON and load them dynamically in your Bevy applications.

## Features

- **Runtime GLTF Loading**: Load GLTF models from JSON configuration files
- **Configurable Parts System**: Define reusable model parts with custom data
- **Material Extensions**: Support for custom material extensions and shaders
- **State-Based Loading**: Integrate with Bevy's state system for controlled loading
- **Flexible Configuration**: Support for custom data extensions in your JSON configs

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
bevy_runtime_gltf_loader = "0.1.0"
bevy = "0.16.0"
```

## Quick Start

### Basic Usage

1. **Create a JSON configuration file** (`config.json`):

```json
{
  "DebugPart": {
    "path": "models/my_model.gltf"
  }
}
```

2. **Set up your Bevy app**:

```rust
use bevy::prelude::*;
use bevy_runtime_gltf_loader::{SimpleModelComposerPlugin, SimplePartsMap};

#[derive(States, Default, Copy, Clone, Debug, PartialEq, Eq, Hash)]
enum GameState {
    #[default]
    Loading,
    Playing,
    Done,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_state::<GameState>()
        .add_plugins(
            SimpleModelComposerPlugin::default().load_single(
                "./config.json",
                GameState::Loading,
                GameState::Playing,
            )
        )
        .add_systems(OnEnter(GameState::Playing), setup_scene)
        .run();
}

fn setup_scene(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    parts: Res<SimplePartsMap>,
) {
    // Load and spawn a part from your JSON config
    parts["DebugPart"]
        .load()
        .build(&mut commands, &asset_server);

    // Add lighting and camera
    commands.spawn((
        PointLight::default(),
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));

    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}
```

## Advanced Usage

### Custom Data Extensions

You can extend your JSON configuration with custom data:

```rust
use bevy::prelude::*;
use bevy_runtime_gltf_loader::{ModelComposerPlugin, PartsMap};
use serde::Deserialize;

#[derive(Asset, Reflect, Deserialize)]
struct CustomData {
    hello: String,
    scale: f32,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_state::<GameState>()
        .add_plugins(
            ModelComposerPlugin::<CustomData>::default().load_single(
                "./extended_config.json",
                GameState::Loading,
                GameState::Playing,
            )
        )
        .add_systems(OnEnter(GameState::Playing), setup_scene_with_data)
        .run();
}

fn setup_scene_with_data(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    parts: Res<PartsMap<CustomData>>,
) {
    let part = &parts["DebugPart"];

    if let Some(data) = &part.data {
        println!("Hello {}", data.hello);
        println!("Scale: {}", data.scale);
    }

    part.load().build(&mut commands, &asset_server);
}
```

With corresponding JSON (`extended_config.json`):

```json
{
  "DebugPart": {
    "path": "models/my_model.gltf",
    "data": {
      "hello": "World",
      "scale": 2.0
    }
  }
}
```

### Material Extensions

Apply custom materials to your loaded models:

```rust
use bevy::pbr::{ExtendedMaterial, MaterialExtension};
use bevy::render::render_resource::{AsBindGroup, ShaderRef};

#[derive(Asset, AsBindGroup, Reflect, Debug, Clone)]
struct MyMaterialExtension {
    #[uniform(100)]
    quantize_steps: u32,
}

impl MaterialExtension for MyMaterialExtension {
    fn fragment_shader() -> ShaderRef {
        "shaders/my_shader.wgsl".into()
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(MaterialPlugin::<
            ExtendedMaterial<StandardMaterial, MyMaterialExtension>
        >::default())
        .init_state::<GameState>()
        .add_plugins(
            SimpleModelComposerPlugin::default()
                .load_single("./config.json", GameState::Loading, GameState::Playing)
                .register_material_extension::<MyMaterialExtension>()
        )
        .add_systems(OnEnter(GameState::Playing), setup_scene_with_materials)
        .run();
}

fn setup_scene_with_materials(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    parts: Res<SimplePartsMap>,
) {
    parts["DebugPart"]
        .load()
        .extend_material(MyMaterialExtension { quantize_steps: 6 })
        .build(&mut commands, &asset_server);
}
```

## API Reference

### Core Types

- **`SimpleModelComposerPlugin`**: Basic plugin for simple usage without custom data
- **`ModelComposerPlugin<T>`**: Generic plugin that supports custom data types
- **`SimplePartsMap`**: Resource containing loaded parts (for simple usage)
- **`PartsMap<T>`**: Generic resource containing loaded parts with custom data
- **`PartPath<T>`**: Represents a single loadable part with optional custom data
- **`PartLoader<T>`**: Builder for configuring how parts are loaded

### Plugin Configuration

```rust
SimpleModelComposerPlugin::default ()
.load_single(file_path, loading_state, target_state)
.register_material_extension::<MaterialType>()
```

### Part Loading

```rust
// Basic loading
parts["PartName"].load().build( & mut commands, & asset_server);

// Loading on specific entity
parts["PartName"].load()
.on(entity)
.build( & mut commands, & asset_server);

// Loading with material extension
parts["PartName"].load()
.extend_material(my_material)
.build( & mut commands, & asset_server);

// Loading specific asset label from GLTF
parts["PartName"].load()
.asset_label(GltfAssetLabel::Scene(1))
.build( & mut commands, & asset_server);
```

## JSON Configuration Format

```json
{
  "PartName": {
    "path": "path/to/model.gltf",
    "data": {
      // Optional custom data (must match your Rust type)
    }
  }
}
```

## Examples

The crate includes several examples:

- **`load_part.rs`**: Basic part loading
- **`config_extension.rs`**: Using custom data extensions
- **`replace_mesh.rs`**: Applying material extensions

Run examples with:

```bash
cargo run --example load_part
cargo run --example config_extension  
cargo run --example replace_mesh
```

## Requirements

- Bevy 0.16.0
- Rust Edition 2024

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.