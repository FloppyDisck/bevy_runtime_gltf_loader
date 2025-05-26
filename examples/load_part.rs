use bevy::DefaultPlugins;
use bevy::asset::AssetServer;
use bevy::prelude::{
    App, AppExtStates, Camera3d, Commands, NextState, OnEnter, PointLight, Res, ResMut, States,
    Transform, Vec3, default,
};
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
        .init_state::<RuntimeState>()
        .add_plugins(SimpleModelComposerPlugin::default().load_single(
            "./config.json",
            RuntimeState::Preload,
            RuntimeState::SceneSetup,
        ))
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
