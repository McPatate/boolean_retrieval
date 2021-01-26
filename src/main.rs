mod inverted_index;
mod wiki;

use inverted_index::InvertedIndex;

fn main() {
    let docs = match wiki::parse_documents(
        "/Users/mc/Documents/boolean_retrieval/enwiki-latest-abstract1.xml",
    ) {
        Ok(d) => d,
        Err(e) => panic!("err : {}", e),
    };
    let mut ii = InvertedIndex::new();
    for i in 0..docs.len() {
        ii.add_wiki_doc(&docs[i], i);
    }
    match ii.save("/Users/mc/Documents/boolean_retrieval/inv_idx.json") {
        Ok(_) => println!("saved file succesfully"),
        Err(e) => panic!("err : {}", e),
    };
}
