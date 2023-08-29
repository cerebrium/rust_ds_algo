use std::collections::HashMap;

use super::LNode::LNode;

#[derive(Debug, Default)]
struct Trie {
    head: HashMap<char, LNode>,
}

impl Trie {
    pub fn create(&mut self, words: Vec<String>) {
        for word in words {
            self.add_word(word)
        }
    }

    pub fn add_word(&mut self, word: String) {
        let mut curr_node = &self.head;

        for letter in word.chars() {
            if curr_node.contains_key(&letter) {
                if let Some(node) = curr_node.get(&letter) {
                    curr_node = &node.links;
                }
            } else {
                curr_node.insert(
                    letter,
                    LNode {
                        is_word: false,
                        val: letter,
                        links: HashMap::new(),
                    },
                );
            }
        }
    }
}
