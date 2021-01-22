use quick_xml::de::{from_str, DeError};
use serde::Deserialize;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

#[derive(Debug, Deserialize, PartialEq)]
pub struct WikiDoc {
    title: String,
    url: String,
    r#abstract: String,
}

impl Clone for WikiDoc {
    fn clone(&self) -> WikiDoc {
        WikiDoc {
            title: self.title.clone(),
            url: self.url.clone(),
            r#abstract: self.r#abstract.clone(),
        }
    }
}

fn load_corpus(fp: &str) -> std::io::Result<String> {
    let file = File::open(fp)?;
    let mut br = BufReader::new(file);
    let mut xml = String::new();
    br.read_to_string(&mut xml)?;
    Ok(xml)
}

pub fn parse_documents(fp: &str) -> Result<Vec<WikiDoc>, DeError> {
    let xml = match load_corpus(fp) {
        Ok(s) => s,
        Err(e) => panic!("err : {}", e)
    };
    let docs = from_str::<Vec<WikiDoc>>(&xml)?;
    Ok(docs)
}
