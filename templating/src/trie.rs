use bincode::{Decode, Encode};
use std::collections::HashMap;
use unicode_segmentation::UnicodeSegmentation;

#[derive(Encode, Decode, Debug, Default, Clone)]
pub struct Trie<T> {
    root: TrieNode<T>,
    pub len: usize,
}

#[derive(Encode, Decode, Debug, Default, Clone)]
pub struct TrieNode<T> {
    children: HashMap<String, TrieNode<T>>,
    data: Option<T>,
}

impl<T> TrieNode<T> {
    pub fn new() -> Self {
        TrieNode {
            children: Default::default(),
            data: None,
        }
    }
}

impl<T> Trie<T> {
    pub fn new() -> Self {
        Trie {
            root: TrieNode::new(),
            len: 0,
        }
    }

    pub fn insert(&mut self, key: &str, data: T) {
        let mut current = &mut self.root;

        for character in UnicodeSegmentation::graphemes(key, true).collect::<Vec<&str>>() {
            current = current
                .children
                .entry(character.to_string())
                .or_insert_with(|| TrieNode::new());

            self.len += 1;
        }

        // Store data at the end of the path
        current.data = Some(data);
    }

    pub fn from_vec(data: Vec<(String, T)>) -> Self {
        let mut trie = Trie::new();

        for (key, value) in data {
            trie.insert(&key, value);
        }

        trie
    }

    pub fn get(&self, key: &str) -> Option<TrieResult<&T>> {
        let mut current = &self.root;
        let mut last = None;

        for (depth, character) in UnicodeSegmentation::graphemes(key, true).enumerate() {
            match current.children.get(character) {
                Some(node) => {
                    if let Some(ref data) = node.data {
                        last = Some((data, depth + 1))
                    }

                    current = node
                }
                None => break,
            }
        }

        if let Some((data, depth)) = last {
            return Some(TrieResult { value: data, depth });
        }

        None
    }
}

pub struct TrieResult<T> {
    pub depth: usize,
    pub value: T,
}

impl<T> TrieResult<T> {
    pub fn new(depth: usize, value: T) -> Self {
        TrieResult { depth, value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insert_tracks_depth() {
        let mut trie = Trie::new();

        trie.insert("test", 0);
        assert_eq!(trie.len, 4);

        trie.insert("test2", 0);
        assert_eq!(trie.len, 9);

        trie.insert("12345", 0);
        assert_eq!(trie.len, 14)
    }

    #[test]
    fn get_longest_match_wins() {
        let mut prefixes = Trie::new();
        prefixes.insert("I", 0);
        prefixes.insert("IRequest", 1);

        if let Some(result) = prefixes.get("IRequestHandler") {
            assert_eq!(result.depth, 8);
        }
    }

    #[test]
    fn get_single_match_wins() {
        let mut prefixes = Trie::new();
        prefixes.insert("I", 0);
        prefixes.insert("IRequestHandler", 1);

        if let Some(result) = prefixes.get("IRequest") {
            assert_eq!(result.depth, 1);
            assert_eq!(*result.value, 0);
        }
    }

    #[test]
    fn insert_and_get_unicode() {
        let mut trie = Trie::new();
        trie.insert("こんにちは", 1);
        trie.insert("こん", 2);

        if let Some(result) = trie.get("こんにちは") {
            assert_eq!(result.depth, 5);
            assert_eq!(*result.value, 1);
        }

        if let Some(result) = trie.get("こんに") {
            assert_eq!(result.depth, 2);
            assert_eq!(*result.value, 2);
        }
    }

    #[test]
    fn get_missing_key_returns_none() {
        let mut trie = Trie::new();
        trie.insert("apple", 1);
        assert!(trie.get("banana").is_none());
    }
}
