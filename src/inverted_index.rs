use crate::wiki::WikiDoc;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader, BufWriter};

#[derive(Serialize, Deserialize)]
pub struct InvertedIndex {
    idx: HashMap<String, Vec<usize>>,
}

impl InvertedIndex {
    pub fn open(&mut self, fp: &str) -> std::io::Result<()> {
        let file = File::open(fp)?;
        let br = BufReader::new(file);
        self.idx = serde_json::from_reader(br)?;
        Ok(())
    }

    pub fn save(&self, fp: &str) -> std::io::Result<()> {
        let file = File::create(fp)?;
        let mut bw = BufWriter::new(file);
        serde_json::to_writer(&mut bw, &self.idx)?;
        Ok(())
    }

    pub fn add_wiki_doc(&mut self, doc: &WikiDoc, doc_id: usize) {
        let tokens = InvertedIndex::tokenizer(doc.r#abstract.clone());
        let lowered_tokens = InvertedIndex::lowercase_filter(tokens);
        for lt in lowered_tokens {
            if self.idx.contains_key(&lt) {
                let postings = match self.idx.get_mut(&lt) {
                    Some(v) => v,
                    None => panic!("unintialized postings list"),
                };
                if !postings.contains(&doc_id) {
                    postings.push(doc_id);
                }
            } else {
                self.idx.insert(lt, vec![doc_id]);
            }
        }
    }

    pub fn new() -> InvertedIndex {
        InvertedIndex {
            idx: HashMap::new(),
        }
    }

    pub fn search(query: &str) {}
    fn parse_query() {}
    fn intersect_not() {}
    fn intersect() {}

    fn lowercase_filter(tokens: Vec<String>) -> Vec<String> {
        let mut res: Vec<String> = Vec::with_capacity(tokens.len());
        for token in tokens {
            res.push(token.to_lowercase());
        }
        res
    }

    fn tokenizer(phrase: String) -> Vec<String> {
        phrase
            .split_terminator(|c: char| !c.is_alphanumeric())
            .map(|s| s.to_string())
            .collect()
    }
}
