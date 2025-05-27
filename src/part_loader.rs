use crate::{EmptyMaterialExtension, WithMaterialExtension};
use bevy::asset::AssetServer;
use bevy::pbr::MaterialExtension;
use bevy::prelude::{BuildChildrenTransformExt, Commands, Entity, GltfAssetLabel, Res, SceneRoot, Transform};
use std::marker::PhantomData;

pub struct PartLoader<'a, EXTENSION, MATERIAL = EmptyMaterialExtension> {
    // Spawns a child node on this entity
    on: Option<Entity>,
    // Sets an offset for the model
    offset: Option<Transform>,
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
            offset: None,
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

    pub fn material_trait<NewMaterial>(self) -> PartLoader<'a, EXTENSION, NewMaterial> {
        PartLoader {
            on: self.on,
            offset: self.offset,
            path: self.path,
            extend_material: None,
            asset_label: self.asset_label,
            phantom: Default::default(),
        }
    }

    pub fn offset(mut self, offset: Transform) -> Self {
        self.offset = Some(offset);
        self
    }

    pub fn extend_material<NewMaterial>(
        self,
        material: NewMaterial,
    ) -> PartLoader<'a, EXTENSION, NewMaterial> {
        let mut new = self.material_trait();
        new.extend_material = Some(material);
        new
    }

    pub fn asset_label(mut self, asset_label: GltfAssetLabel) -> Self {
        self.asset_label = Some(asset_label);
        self
    }

    pub fn build(self, commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
        let Self {
            on,
            offset,
            path,
            asset_label,
            extend_material,
            ..
        } = self;

        let mut entity_commands = commands.spawn_empty();

        if let Some(on) = on {
            entity_commands.set_parent_in_place(on);
        }

        let path = asset_label
            .unwrap_or(GltfAssetLabel::Scene(0))
            .from_asset(path.clone());

        let scene = asset_server.load(path);

        if let Some(offset) = offset {
            entity_commands.insert(offset);
        }

        entity_commands.insert(SceneRoot(scene));

        if let Some(material) = extend_material {
            entity_commands.insert(WithMaterialExtension(material));
        }

        entity_commands.id()
    }
}
