use bevy::pbr::{ExtendedMaterial, MaterialExtension};
use bevy::prelude::{
    Assets, Children, Commands, Component, Entity, MeshMaterial3d, Query, ResMut, StandardMaterial,
    Trigger,
};
use bevy::scene::SceneInstanceReady;

#[derive(Component)]
pub struct WithMaterialExtension<MATERIAL>(pub MATERIAL);

pub fn replace_material<MATERIAL: MaterialExtension + Clone>(
    trigger: Trigger<SceneInstanceReady>,
    mut commands: Commands,
    material_extension_query: Query<&WithMaterialExtension<MATERIAL>>,
    query_children: Query<(Option<&MeshMaterial3d<StandardMaterial>>, Option<&Children>)>,
    mut materials: ResMut<Assets<ExtendedMaterial<StandardMaterial, MATERIAL>>>,
    mut std_materials: ResMut<Assets<StandardMaterial>>,
) {
    let target = trigger.target();

    let Ok(with_material_extension) = material_extension_query.get(target) else {
        return;
    };

    commands
        .entity(target)
        .remove::<WithMaterialExtension<MATERIAL>>();
    let WithMaterialExtension(material_extension) = with_material_extension;

    process_children(
        target,
        material_extension,
        &mut commands,
        &query_children,
        &mut materials,
        &mut std_materials,
    );
}

fn process_children<MATERIAL: MaterialExtension + Clone>(
    entity: Entity,
    material_extension: &MATERIAL,
    commands: &mut Commands,
    query_children: &Query<(Option<&MeshMaterial3d<StandardMaterial>>, Option<&Children>)>,
    materials: &mut ResMut<Assets<ExtendedMaterial<StandardMaterial, MATERIAL>>>,
    std_materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    if let Ok((mat, children)) = query_children.get(entity) {
        // Process material
        if let Some(mat) = mat {
            let std = std_materials.get(&mat.0).unwrap();

            let extended = ExtendedMaterial {
                base: std.clone(),
                extension: material_extension.clone(),
            };

            commands
                .entity(entity)
                .remove::<MeshMaterial3d<StandardMaterial>>()
                .insert(MeshMaterial3d(materials.add(extended)));
        }

        if let Some(children) = children {
            for child in children.iter() {
                process_children(
                    *child,
                    material_extension,
                    commands,
                    query_children,
                    materials,
                    std_materials,
                );
            }
        }
    }
}
