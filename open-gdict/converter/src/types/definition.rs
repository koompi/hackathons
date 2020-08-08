use serde_derive::{Serialize, Deserialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Definition {
    definition: String,
    example: String,
    synonyms: Vec<String>
}