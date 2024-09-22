use crate::trie::{Trie, TrieResult};
use crate::util;
use bincode;
use bincode::{Decode, Encode};
use std::collections::HashMap;

#[derive(Encode, Decode)]
struct TemplateTrieData {
    matches: Vec<usize>,
}

#[derive(Encode, Decode)]
pub struct TemplateEngine {
    prefixes: Trie<TemplateTrieData>,
    suffixes: Trie<TemplateTrieData>,
    tags: Trie<TemplateTrieData>,
}

#[derive(Debug, Clone)]
pub struct TemplateMetadata {
    prefixes: Option<Vec<String>>,
    suffixes: Option<Vec<String>>,
    tags: Option<Vec<String>>,
}

impl TemplateMetadata {
    pub fn new(
        prefixes: Option<Vec<String>>,
        suffixes: Option<Vec<String>>,
        tags: Option<Vec<String>>,
    ) -> Self {
        TemplateMetadata {
            prefixes,
            suffixes,
            tags,
        }
    }
}

impl TemplateEngine {
    pub fn new() -> Self {
        Self {
            prefixes: Trie::new(),
            suffixes: Trie::new(),
            tags: Trie::new(),
        }
    }

    pub fn from_templates(templates: Vec<TemplateMetadata>) -> Self {
        // Key, Template Index
        let mut prefixes: HashMap<String, Vec<usize>> = HashMap::new();
        let mut suffixes: HashMap<String, Vec<usize>> = HashMap::new();
        let mut tags: HashMap<String, Vec<usize>> = HashMap::new();

        let add_to_map =
            |index: usize, keys: &Vec<String>, map: &mut HashMap<String, Vec<usize>>| {
                for key in keys {
                    let entry = map.entry(key.to_string()).or_insert_with(&Vec::new);
                    entry.push(index);
                }
            };

        for (index, template) in templates.iter().enumerate() {
            if let Some(p) = &template.prefixes {
                add_to_map(index, p, &mut prefixes);
            }
            if let Some(s) = &template.suffixes {
                add_to_map(index, s, &mut suffixes);
            }
            if let Some(t) = &template.tags {
                add_to_map(index, t, &mut tags);
            }
        }

        Self {
            prefixes: Trie::from_vec(
                prefixes
                    .into_iter()
                    .map(|(key, matches)| (key, TemplateTrieData { matches }))
                    .collect(),
            ),
            // Reverse suffixes
            suffixes: Trie::from_vec(
                suffixes
                    .into_iter()
                    .map(|(key, matches)| {
                        (util::unicode_reverse(&key), TemplateTrieData { matches })
                    })
                    .collect(),
            ),
            tags: Trie::from_vec(
                tags.into_iter()
                    .map(|(key, matches)| (key, TemplateTrieData { matches }))
                    .collect(),
            ),
        }
    }

    pub fn search(&self, term: &str, tags: Option<&Vec<&str>>) -> Option<usize> {
        let mut results: HashMap<usize, u32> = HashMap::new();
        let mut highest_score = 0;
        let mut highest_index = None;

        let mut add_scores = |found: TrieResult<&TemplateTrieData>| {
            for match_id in &found.value.matches {
                let entry = results.entry(*match_id).or_insert(0);
                *entry += found.depth as u32;

                if *entry > highest_score {
                    highest_score = *entry;
                    highest_index = Some(*match_id);
                }
            }
        };

        if let Some(matches) = self.prefixes.get(term) {
            add_scores(matches);
        }

        let suffix = util::unicode_reverse(term);
        if let Some(matches) = self.suffixes.get(&suffix) {
            add_scores(matches);
        }

        if let Some(tags) = tags {
            for tag in tags {
                if let Some(matches) = self.tags.get(tag) {
                    add_scores(matches);
                }
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
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_template_search_multi_match() {
        let engine = TemplateEngine::from_templates(vec![
            TemplateMetadata {
                prefixes: Some(vec!["IRequest".to_string(), "Request".to_string()]),
                suffixes: Some(vec![
                    "Handler.cs".to_string(),
                    ".cs".to_string(),
                    "random".to_string(),
                ]),
                tags: Some(vec!["csharp".to_string()]),
            },
            TemplateMetadata {
                prefixes: None,
                suffixes: Some(vec![".cs".to_string()]),
                tags: Some(vec!["csharp".to_string()]),
            },
        ]);

        let result = engine.search(&"MyFavoriteRequestHandler.cs", Some(&vec!["csharp"]));
        assert_eq!(result, Some(0))
    }

    #[test]
    fn engine_search_finds_matching_suffix_template() {
        let engine = TemplateEngine::from_templates(vec![
            TemplateMetadata {
                prefixes: Some(vec!["IRequest".to_string()]),
                suffixes: None,
                tags: Some(vec!["csharp".to_string()]),
            },
            TemplateMetadata {
                prefixes: None,
                suffixes: Some(vec!["RequestHandler.cs".to_string()]),
                tags: None,
            },
        ]);

        let result = engine.search("IRequestHandler.cs", None);
        assert_eq!(result, Some(1), "Failed to match the correct template");
    }
}
