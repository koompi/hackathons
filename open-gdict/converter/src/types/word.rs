use serde_derive::{Serialize, Deserialize};

use super::{
    phonetic::Phonetic,
    meaning::Meaning
};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Word {
    word: String,
    phonetics: Vec<Phonetic>,
    meanings: Vec<Meaning>,
}