use serde_derive::{Serialize, Deserialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Phonetic {
    pub text: String,
    pub audio: String,
}