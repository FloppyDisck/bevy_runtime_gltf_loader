use crate::parts_map::PartsMap;
use bevy::app::PreStartup;
use bevy::asset::{Asset, AssetServer, Assets, Handle};
use bevy::prelude::{
    Commands, FixedPreUpdate, IntoScheduleConfigs, NextState, Res, ResMut, Resource, States,
    in_state,
};
use bevy::state::state::FreelyMutableState;
use std::marker::PhantomData;

pub(crate) fn config_singleton<EXTENSION, STATE>(
    app: &mut bevy::prelude::App,
    file: &'static str,
    run_in: STATE,
    to_state: STATE,
) where
    for<'de> EXTENSION: serde::Deserialize<'de> + Asset,
    STATE: States + FreelyMutableState + Clone,
{
    app.insert_resource(ConfigLoadTarget::<EXTENSION>::new(file))
        .insert_resource(PartsMap::<EXTENSION>::default())
        .insert_resource(MoveToState::<EXTENSION, STATE>::new(to_state))
        .add_systems(PreStartup, preload_single::<EXTENSION>)
        .add_systems(
            FixedPreUpdate,
            load_config::<EXTENSION, STATE>.run_if(in_state(run_in)),
        );
}

#[derive(Resource)]
struct MoveToState<EXTENSION, STATE> {
    next: STATE,
    phantom_data: PhantomData<EXTENSION>,
}

impl<EXTENSION, STATE> MoveToState<EXTENSION, STATE> {
    pub fn new(next: STATE) -> Self {
        Self {
            next,
            phantom_data: Default::default(),
        }
    }
}

#[derive(Resource)]
struct ConfigLoadTarget<EXTENSION> {
    path: String,
    phantom_data: PhantomData<EXTENSION>,
}

impl<EXTENSION> ConfigLoadTarget<EXTENSION> {
    pub fn new(path: &str) -> Self {
        Self {
            path: path.to_string(),
            phantom_data: Default::default(),
        }
    }
}

fn preload_single<EXTENSION>(
    mut commands: Commands,
    load: Res<ConfigLoadTarget<EXTENSION>>,
    asset_server: Res<AssetServer>,
) where
    for<'de> EXTENSION: serde::Deserialize<'de> + Asset + Sync + Send + 'static,
{
    let parts = ConfigLoadHandle::<EXTENSION>(asset_server.load(load.path.as_str()));
    commands.insert_resource(parts);
    commands.remove_resource::<ConfigLoadTarget<EXTENSION>>();
}

#[derive(Resource)]
pub struct ConfigLoadHandle<EXTENSION: Asset>(Handle<PartsMap<EXTENSION>>);

// Assumed the file is already loaded, this should work for most scenarios
#[allow(private_interfaces)]
pub fn load_config<EXTENSION, STATE: States + FreelyMutableState + Clone>(
    mut commands: Commands,
    mut parts_map: ResMut<PartsMap<EXTENSION>>,
    load: Option<Res<ConfigLoadHandle<EXTENSION>>>,
    next_state_res: Res<MoveToState<EXTENSION, STATE>>,
    mut config: ResMut<Assets<PartsMap<EXTENSION>>>,
    mut next_state: ResMut<NextState<STATE>>,
) where
    for<'de> EXTENSION: serde::Deserialize<'de> + Asset + Sync + Send + 'static,
{
    if let Some(asset) = load.and_then(|res| config.remove(res.0.id())) {
        commands.remove_resource::<ConfigLoadTarget<EXTENSION>>();
        parts_map.0 = asset.0;

        next_state.set(next_state_res.next.clone());
    }
}
