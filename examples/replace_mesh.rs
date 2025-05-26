use bevy::DefaultPlugins;
use bevy::asset::AssetServer;
use bevy::pbr::{ExtendedMaterial, MaterialExtension};
use bevy::prelude::{
    App, AppExtStates, Asset, Camera3d, Commands, MaterialPlugin, NextState, OnEnter, PointLight,
    Reflect, Res, ResMut, StandardMaterial, States, Transform, Vec3, default,
};
use bevy::render::render_resource::{AsBindGroup, ShaderRef};
use bevy_model_composer::parts_map::{SimpleModelComposerPlugin, SimplePartsMap};

#[derive(States, Default, Copy, Clone, Debug, PartialEq, Eq, Hash)]
enum RuntimeState {
    #[default]
    Preload,
    SceneSetup,
    Done,
}

pub fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(MaterialPlugin::<
            ExtendedMaterial<StandardMaterial, MyMaterialExtension>,
        >::default())
        .init_state::<RuntimeState>()
        .add_plugins(
            SimpleModelComposerPlugin::default()
                .load_single(
                    "./config.json",
                    RuntimeState::Preload,
                    RuntimeState::SceneSetup,
                )
                .register_material_extension::<MyMaterialExtension>(),
        )
        .add_systems(OnEnter(RuntimeState::SceneSetup), setup_scene)
        .run();
}

fn setup_scene(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    parts: Res<SimplePartsMap>,
    mut next_state: ResMut<NextState<RuntimeState>>,
) {
    // Spawn part
    parts["DebugPart"]
        .load()
        .extend_material(MyMaterialExtension { quantize_steps: 6 })
        .build(&mut commands, &asset_server);

    // light
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));
    // camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    next_state.set(RuntimeState::Done);
}

// Mesh code
/// This example uses a shader source file from the assets subdirectory
const SHADER_ASSET_PATH: &str = "shaders/extended_material.wgsl";

#[derive(Asset, AsBindGroup, Reflect, Debug, Clone)]
struct MyMaterialExtension {
    // We need to ensure that the bindings of the base material and the extension do not conflict,
    // so we start from binding slot 100, leaving slots 0-99 for the base material.
    #[uniform(100)]
    quantize_steps: u32,
}

impl MaterialExtension for MyMaterialExtension {
    fn fragment_shader() -> ShaderRef {
        SHADER_ASSET_PATH.into()
    }

    fn deferred_fragment_shader() -> ShaderRef {
        SHADER_ASSET_PATH.into()
    }
}
