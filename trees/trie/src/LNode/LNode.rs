pub type Link = Option<Box<LNode>>;

#[derive(Debug)]
pub struct LNode {
    pub val: char,
    pub links: Vec<Link>,
    pub is_word: bool,
}
impl LNode {
    pub fn new(char: char, is_word: bool) -> Link {
        return Some(Box::new(LNode {
            val: char,
            links: vec![],
            is_word,
        }));
    }
}
