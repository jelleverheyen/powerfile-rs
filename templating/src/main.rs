use crate::index::{TemplateIndex, TemplateOptions};
use std::path::PathBuf;
use std::str::FromStr;

mod index;
mod search;
mod trie;
mod util;

fn main() -> std::io::Result<()> {
    let index = TemplateIndex::build(TemplateOptions {
        block_size: 128,
        cached_templates_dir: PathBuf::from_str("./cache").unwrap(),
        index_path: PathBuf::from_str("./cache/index").unwrap(),
        template_source_dir: PathBuf::from_str("./templates").unwrap(),
    });

    index.write();
    let engine = index.to_engine();
    let result = engine.search(&"IRequestHandler", None);
    println!("search result: {:?}", result);

    Ok(())
}
