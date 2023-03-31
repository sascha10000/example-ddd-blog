use serde::{Serialize, Deserialize};

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct Author {
    pub id: u64,
    pub name: String,
    pub surname: String
}
