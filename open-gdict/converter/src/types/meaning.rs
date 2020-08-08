use serde_derive::{Serialize, Deserialize};
use super::definition::Definition;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Meaning {
    part_of_speech: String,
    definitions: Vec<Definition>
}