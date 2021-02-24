mod inverted_index;
mod skiplist;
mod wiki;

use inverted_index::InvertedIndex;
use std::io::{stdin, stdout, Write};

fn main() {
    let docs = match wiki::parse_documents(
        "/Users/mc/Documents/boolean_retrieval/enwiki-latest-abstract1.xml",
    ) {
        Ok(d) => d,
        Err(e) => panic!("err : {}", e),
    };
    let mut ii = InvertedIndex::new();
    match ii.open("inv_idx.json") {
        Ok(_) => println!("succesfully loaded inv idx"),
        Err(e) => panic!("err : {}", e),
    };
    //    for i in 0..docs.len() {
    //        ii.add_wiki_doc(&docs[i], i);
    //    }
    //    match ii.save("/Users/mc/Documents/boolean_retrieval/inv_idx.json") {
    //        Ok(_) => println!("saved file succesfully"),
    //        Err(e) => panic!("err : {}", e),
    //    };
    loop {
        let mut s = String::new();
        print!("> ");
        let _ = stdout().flush();
        stdin().read_line(&mut s).expect("Invalid string");
        if let Some('\n') = s.chars().next_back() {
            s.pop();
        }
        match ii.search(&s) {
            Some(results) => {
                for r in &results {
                    println!("\n---- {} ----\n{}\n", docs[*r].title, docs[*r].r#abstract);
                }
                println!("{} hits\n", results.len());
            }
            None => println!("Nothing found"),
        };
    }
}
