use std::error::Error;
use std::fmt::Display;
use bevy::prelude::BevyError;
use crate::part_loader::PartLoader;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct PartPath<EXTENSION> {
    pub path: String,
    pub data: Option<EXTENSION>,
}

impl<EXTENSION> PartPath<EXTENSION> {
    pub fn load(&self) -> PartLoader<EXTENSION> {
        PartLoader::new(&self.path)
    }

    pub fn data(&self) -> Result<&EXTENSION, BevyError> {
        self.data.as_ref().ok_or(BevyError::from(MissingData))
    }
}

#[derive(Debug)]
pub struct MissingData;

impl Display for MissingData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "unable to read data for part path")
    }
}

impl Error for MissingData {}