use crate::PartPath;
use bevy::prelude::{Asset, BevyError, Deref, Reflect, Resource};
use serde::Deserialize;
use std::collections::HashMap;
use std::error::Error;
use std::fmt::Display;

// Json loaded map of all available parts
#[derive(Resource, Deref, Asset, Reflect, Deserialize)]
pub struct PartsMap<EXTENSION: Asset>(pub(crate) HashMap<String, PartPath<EXTENSION>>);

impl<EXTENSION: Asset> Default for PartsMap<EXTENSION> {
    fn default() -> Self {
        Self(HashMap::new())
    }
}

impl<EXTENSION: Asset> PartsMap<EXTENSION> {
    pub fn load_part(&self, name: &str) -> Result<&PartPath<EXTENSION>, BevyError> {
        self.get(name).ok_or(BevyError::from(PartNotFoundError::new(name)))
    }
}

#[derive(Debug)]
pub struct PartNotFoundError {
    pub part: String,
}

impl PartNotFoundError {
    pub fn new(part: &str) -> Self {
        Self {
            part: part.to_string(),
        }
    }
}

impl Display for PartNotFoundError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "unable to read {} part", &self.part)
    }
}

impl Error for PartNotFoundError {}
