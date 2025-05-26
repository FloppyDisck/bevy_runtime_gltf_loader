use crate::config_singleton::config_singleton;
use crate::{PartsMap, replace_material};
use bevy::asset::Asset;
use bevy::pbr::MaterialExtension;
use bevy::prelude::{Plugin, Reflect, States};
use bevy::render::render_resource::AsBindGroup;
use bevy::state::state::FreelyMutableState;
use bevy_common_assets::json::JsonAssetPlugin;
use serde::Deserialize;
use std::marker::PhantomData;

pub type SimpleModelComposerPlugin = ModelComposerPlugin<EmptyExtension>;

pub struct ModelComposerPlugin<EXTENSION, STATE = EmptyState, MATERIAL = EmptyMaterialExtension> {
    file_ending: &'static str,
    // Load a single config as a resource
    load_single: Option<LoadSingleConfig<STATE>>,
    register_material_extension: bool,
    phantom_extension: PhantomData<EXTENSION>,
    phantom_material: PhantomData<MATERIAL>,
}

#[derive(Clone)]
struct LoadSingleConfig<STATE> {
    file: &'static str,
    run_in: STATE,
    to_state: STATE,
}

impl<EXTENSION, STATE, MATERIAL> ModelComposerPlugin<EXTENSION, STATE, MATERIAL> {
    pub fn new(file_ending: &'static str) -> Self {
        Self {
            file_ending,
            load_single: None,
            register_material_extension: false,
            phantom_extension: Default::default(),
            phantom_material: Default::default(),
        }
    }

    /// Register a set of systems that will register a single config as a resource you can then use
    /// This registered resource can then be found as `Res<PartsMap<EXTENSION>>`
    /// If using `SimpleModelComposerPlugin` then the resource can also be accessed as `Res<SimplePartsMap>`
    pub fn load_single<NewState>(
        self,
        file: &'static str,
        run_in: NewState,
        to_state: NewState,
    ) -> ModelComposerPlugin<EXTENSION, NewState, MATERIAL> {
        ModelComposerPlugin {
            file_ending: self.file_ending,
            register_material_extension: self.register_material_extension,
            load_single: Some(LoadSingleConfig {
                file,
                run_in,
                to_state,
            }),
            phantom_extension: Default::default(),
            phantom_material: Default::default(),
        }
    }

    pub fn register_material_extension<NewMaterial>(
        self,
    ) -> ModelComposerPlugin<EXTENSION, STATE, NewMaterial> {
        ModelComposerPlugin {
            file_ending: self.file_ending,
            load_single: self.load_single,
            register_material_extension: true,
            phantom_extension: Default::default(),
            phantom_material: Default::default(),
        }
    }
}

impl<EXTENSION, STATE, MATERIAL> Default for ModelComposerPlugin<EXTENSION, STATE, MATERIAL> {
    fn default() -> Self {
        Self::new(".json")
    }
}

impl<EXTENSION, STATE, MATERIAL> Plugin for ModelComposerPlugin<EXTENSION, STATE, MATERIAL>
where
    for<'de> EXTENSION: serde::Deserialize<'de> + Asset,
    STATE: States + FreelyMutableState + Clone,
    MATERIAL: MaterialExtension + Clone,
{
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins(JsonAssetPlugin::<PartsMap<EXTENSION>>::new(&[
            self.file_ending
        ]));

        if let Some(LoadSingleConfig {
            file,
            run_in,
            to_state,
        }) = self.load_single.clone()
        {
            config_singleton::<EXTENSION, STATE>(app, file, run_in, to_state);
        }

        if self.register_material_extension {
            app.add_observer(replace_material::<MATERIAL>);
        }
    }
}

#[derive(States, Copy, Clone, Debug, PartialEq, Eq, Hash)]
// Generic state for when users dont want to implement custom state loading
pub enum EmptyState {}

// For when you dont want to use the data field
#[derive(Asset, Reflect, Debug, Deserialize, Copy, Clone)]
pub struct EmptyExtension;

pub type SimplePartsMap = PartsMap<EmptyExtension>;

#[derive(Asset, AsBindGroup, Reflect, Debug, Clone)]
pub struct EmptyMaterialExtension {}

impl MaterialExtension for EmptyMaterialExtension {}
