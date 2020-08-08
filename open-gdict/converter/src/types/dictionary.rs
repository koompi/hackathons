use serde_derive::{Serialize, Deserialize};
use super::word::Word;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Dictionary {
    pub dictionary: Vec<Word>
}