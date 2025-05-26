use crate::parts_map::PartPath;
use bevy::prelude::{Asset, Deref, Reflect, Resource};
use serde::Deserialize;
use std::collections::HashMap;

// Json loaded map of all available parts
#[derive(Resource, Deref, Asset, Reflect, Deserialize)]
pub struct PartsMap<EXTENSION: Asset>(pub(crate) HashMap<String, PartPath<EXTENSION>>);

impl<EXTENSION: Asset> Default for PartsMap<EXTENSION> {
    fn default() -> Self {
        Self(HashMap::new())
    }
}

impl<EXTENSION: Asset> PartsMap<EXTENSION> {
    pub fn contains(&self, name: &str) -> bool {
        self.0.contains_key(name)
    }

    pub fn may_load_part(&self, name: &str) -> Option<&PartPath<EXTENSION>> {
        self.0.get(name)
    }
    pub fn load_part(&self, name: &str) -> &PartPath<EXTENSION> {
        self.may_load_part(name).unwrap()
    }
}
