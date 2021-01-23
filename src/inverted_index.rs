use crate::wiki::WikiDoc;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufWriter;

#[derive(Serialize, Deserialize)]
pub struct InvertedIndex {
    idx: HashMap<String, Vec<i32>>,
}

impl InvertedIndex {
    pub fn save(&self, fp: &str) -> std::io::Result<()> {
        let file = File::open(fp)?;
        let mut bw = BufWriter::new(file);
        serde_json::to_writer(&mut bw, &self.idx)?;
        Ok(())
    }

    pub fn add_wiki_doc(doc: WikiDoc) {}
    pub fn search(query: &str) {}
    fn parse_query() {}
    fn intersect_not() {}
    fn intersect() {}
}
