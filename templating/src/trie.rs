use std::collections::HashMap;
use bincode::{Decode, Encode};
use unicode_segmentation::UnicodeSegmentation;

#[derive(Encode, Decode, Debug, Default, Clone)]
pub struct Trie<T> {
    root: TrieNode<T>,
    len: usize,
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
            current = current.children.entry(character.to_string()).or_insert_with(|| TrieNode::new());
        }

        // Store data at the end of the path
        current.data = Some(data);
        self.len += 1;
    }

    pub fn get(&self, key: &str) -> Option<&T> {
        let mut current = &self.root;
        let mut last = None;

        for character in UnicodeSegmentation::graphemes(key, true) {
            match current.children.get(character) {
                Some(node) => {
                    if let Some(ref data) = node.data {
                        last = Some(data)
                    }

                    current = node
                },
                None => break,
            }
        }

        last
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_get() {
        let mut trie = Trie::new();
        trie.insert("apple", 1);
        trie.insert("app", 2);

        assert_eq!(trie.get("apple"), Some(&1));
        assert_eq!(trie.get("app"), Some(&2));
    }

    #[test]
    fn test_insert_and_get_unicode() {
        let mut trie = Trie::new();
        trie.insert("こんにちは", 1);
        trie.insert("こん", 2);

        assert_eq!(trie.get("こんにちは"), Some(&1));
        assert_eq!(trie.get("こん"), Some(&2));
    }

    #[test]
    fn test_get_partial_match() {
        let mut trie = Trie::new();
        trie.insert("hello", 1);
        trie.insert("hell", 2);

        // Returns the last data found, which is "hell" with depth 4
        assert_eq!(trie.get("hello"), Some(&1));
        assert_eq!(trie.get("hell"), Some(&2));
        assert_eq!(trie.get("he"), None); // 'he' is not stored in the trie
    }

    #[test]
    fn test_get_with_prefix() {
        let mut trie = Trie::new();
        trie.insert("hello", 1);
        trie.insert("hell", 2);
        trie.insert("heaven", 3);

        assert_eq!(trie.get("heaven"), Some(&3));
        assert_eq!(trie.get("hell"), Some(&2));
        assert_eq!(trie.get("hello"), Some(&1));
    }

    #[test]
    fn test_get_missing_key() {
        let mut trie = Trie::new();
        trie.insert("apple", 1);

        assert_eq!(trie.get("banana"), None); // "banana" is not in the trie
    }

    #[test]
    fn test_get_with_long_prefix() {
        let mut trie = Trie::new();
        trie.insert("apple", 1);
        trie.insert("app", 2);
        trie.insert("apartment", 3);

        // No exact match for "appl", but returns last found data (app, 3)
        assert_eq!(trie.get("appl"), Some(&2));
        assert_eq!(trie.get("app"), Some(&2));
        assert_eq!(trie.get("apartment"), Some(&3));
    }

    #[test]
    fn test_len() {
        let mut trie = Trie::new();
        trie.insert("dog", 1);
        trie.insert("cat", 2);
        trie.insert("donkey", 3);

        assert_eq!(trie.len, 3); // len field should correctly reflect number of entries
    }

    #[test]
    fn test_longer_input() {
        let mut trie = Trie::new();
        trie.insert("hello", 1);
        trie.insert("he", 2);
        trie.insert("helloworld", 3);

        // Matching as far as it can go
        assert_eq!(trie.get("helloworldish"), Some(&3));
    }
}

