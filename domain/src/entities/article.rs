use serde::{Serialize, Deserialize};

use crate::entities::Author;

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct Article {
    pub id: u64,
    pub title: String,
    pub author: Author,
    pub text : String
}
