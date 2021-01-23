mod inverted_index;
mod wiki;

fn main() {
    let docs = match wiki::parse_documents(
        "/Users/mc/Documents/boolean_retrieval/enwiki-latest-abstract1.xml",
    ) {
        Ok(d) => d,
        Err(e) => panic!("err : {}", e),
    };
    for i in 0..5 {
        println!("{:?}", docs[i]);
    }
}
