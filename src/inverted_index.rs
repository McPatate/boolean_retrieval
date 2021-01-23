use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct InvertedIndex {
    idx: HashMap<String, Vec<i32>>,
}

impl InvertedIndex {
    pub fn save() {}
    pub fn build() {}
    pub fn search(query: &str) {}
    fn parse_query() {}
    fn intersect_not() {}
    fn intersect() {}
}
