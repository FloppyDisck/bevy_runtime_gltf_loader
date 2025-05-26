use crate::{EmptyMaterialExtension, WithMaterialExtension};
use bevy::asset::AssetServer;
use bevy::pbr::MaterialExtension;
use bevy::prelude::{Commands, Entity, GltfAssetLabel, Res, SceneRoot};
use std::marker::PhantomData;

pub struct PartLoader<'a, EXTENSION, MATERIAL = EmptyMaterialExtension> {
    on: Option<Entity>,
    path: &'a String,
    extend_material: Option<MATERIAL>,
    asset_label: Option<GltfAssetLabel>,
    phantom: PhantomData<EXTENSION>,
}

impl<'a, EXTENSION, MATERIAL> PartLoader<'a, EXTENSION, MATERIAL>
where
    MATERIAL: MaterialExtension + Clone,
{
    pub fn new(path: &'a String) -> Self {
        PartLoader {
            on: None,
            asset_label: None,
            path,
            phantom: Default::default(),
            extend_material: None,
        }
    }

    pub fn on(mut self, entity: Entity) -> Self {
        self.on = Some(entity);
        self
    }

    pub fn extend_material<NewMaterial>(
        self,
        material: NewMaterial,
    ) -> PartLoader<'a, EXTENSION, NewMaterial> {
        PartLoader {
            on: self.on,
            path: self.path,
            extend_material: Some(material),
            asset_label: self.asset_label,
            phantom: Default::default(),
        }
    }

    pub fn asset_label(mut self, asset_label: GltfAssetLabel) -> Self {
        self.asset_label = Some(asset_label);
        self
    }

    pub fn build(self, commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
        let Self {
            on,
            path,
            asset_label,
            extend_material,
            ..
        } = self;

        let entity = on.unwrap_or(commands.spawn_empty().id());
        let mut entity_commands = commands.entity(entity);

        let path = asset_label
            .unwrap_or(GltfAssetLabel::Scene(0))
            .from_asset(path.clone());

        let scene = asset_server.load(path);

        entity_commands.insert(SceneRoot(scene));

        if let Some(material) = extend_material {
            entity_commands.insert(WithMaterialExtension(material));
        }

        entity_commands.id()
    }
}
