use std::collections::HashMap;
use crate::trie::Trie;
use bincode;
use bincode::{Decode, Encode};
use unicode_segmentation::UnicodeSegmentation;


#[derive(Encode, Decode)]
struct TemplateTrieData {
    template_index: usize,
    score: u32,
}

#[derive(Encode, Decode)]
struct TemplateEngine {
    prefixes: Trie<TemplateTrieData>,
    suffixes: Trie<TemplateTrieData>,
    tags: Trie<TemplateTrieData>,
}

impl TemplateEngine {
    pub fn new() -> Self {
        Self { prefixes: Trie::new(), suffixes: Trie::new(), tags: Trie::new() }
    }
}

impl TemplateEngine {
    pub fn search(self: &Self, term: &str, tags: Vec<&str>) -> Option<usize> {
        let mut results: HashMap<usize, u32> = HashMap::new();
        let mut highest_score = 0;
        let mut highest_index = None;

        let mut add_score = |template_index: usize, score: u32| {
            let entry = results.entry(template_index).or_insert(0);
            *entry += score;

            if *entry > highest_score {
                highest_score = *entry;
                highest_index = Some(template_index);
            }
        };

        if let Some(data) = self.prefixes.get(term) {
            add_score(data.template_index, data.score);
        }

        let suffix = UnicodeSegmentation::graphemes(term, true).collect::<String>();
        if let Some(data) = self.suffixes.get(&suffix) {
            add_score(data.template_index, data.score);
        }

        for tag in tags {
            if let Some(data) = self.tags.get(tag) {
                add_score(data.template_index, data.score);
            }
        }

        highest_index
    }

    // pub fn from_cache<R>(reader: R) -> Self
    // where
    //     R: Read,
    // {
    //     let result: TemplateEngine =
    //         bincode::decode_from_std_read(reader, bincode::config::standard()).unwrap();
    //
    //     result
    // }
    //
    // pub fn to_cache<W>(writer: W) -> Result<usize, Err>
    // where
    //     W: Write,
    // {
    //     let _ = bincode::encode_into_std_write(Self, writer, bincode::config::standard());
    // }
}
