use bevy::DefaultPlugins;
use bevy::asset::{Asset, AssetServer};
use bevy::prelude::{
    App, AppExtStates, Camera3d, Commands, NextState, OnEnter, PointLight, Reflect, Res, ResMut,
    States, Transform, Vec3, default,
};
use bevy_runtime_gltf_loader::{ModelComposerPlugin, PartsMap};
use serde::Deserialize;

#[derive(States, Default, Copy, Clone, Debug, PartialEq, Eq, Hash)]
enum RuntimeState {
    #[default]
    Preload,
    SceneSetup,
    Done,
}

// Minimum required traits
#[derive(Asset, Reflect, Deserialize)]
struct CustomExtension {
    hello: String,
}

pub fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_state::<RuntimeState>()
        .add_plugins(
            ModelComposerPlugin::<CustomExtension>::default().load_single(
                "./extended_config.json",
                RuntimeState::Preload,
                RuntimeState::SceneSetup,
            ),
        )
        .add_systems(OnEnter(RuntimeState::SceneSetup), setup_scene)
        .run();
}

fn setup_scene(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    parts: Res<PartsMap<CustomExtension>>,
    mut next_state: ResMut<NextState<RuntimeState>>,
) {
    let part = &parts["DebugPart"];

    println!("Hello {}", part.data.as_ref().unwrap().hello);

    // Spawn part
    part.load().build(&mut commands, &asset_server);

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
