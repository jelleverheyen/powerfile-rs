use crate::template_index::{read_template_index, write_template_index};

mod indexer;
mod search;
mod storage;
mod template_index;
mod trie;
mod util;

fn main() -> std::io::Result<()> {
    let index = write_template_index(
        "template.index",
        vec![
            "my_templates/IRequestHandler.cs",
            "my_templates/csharp_class.cs",
        ],
    )?;

    let test = read_template_index("template.index", &vec![0, 1])?;
    for i in test {
        println!("{:?}", i)
    }

    Ok(())
}
