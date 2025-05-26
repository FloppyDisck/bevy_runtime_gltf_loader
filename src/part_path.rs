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
}
