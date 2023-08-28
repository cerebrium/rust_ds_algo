use std::borrow::BorrowMut;

use super::LNode::{LNode, Link};

#[derive(Debug, Default)]
struct Trie {
    head: Vec<Link>,
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
            for node in curr_node {
                match node {
                    Some(deref_n) => {
                        if deref_n.val == letter {
                            curr_node = &deref_n.links;
                            break;
                        }
                    }
                    _ => (),
                };
            }

            // No match found in current options
            let last_letter = word.chars().last();
            let mut is_word = false;
            if let Some(ll) = last_letter {
                if ll == letter {
                    is_word = true;
                }
            }

            curr_node.borrow_mut().push(LNode::new(letter, is_word))
        }
    }
}
