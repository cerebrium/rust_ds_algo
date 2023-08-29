use std::collections::HashMap;

#[derive(Debug)]
pub struct LNode {
    pub is_word: bool,
    pub val: char,
    pub links: HashMap<char, LNode>,
}

impl LNode {
    pub fn new(letter: char, is_word: bool) -> Self {
        LNode {
            is_word,
            val: letter,
            links: HashMap::new(),
        }
    }
}
