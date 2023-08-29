use std::{cell::RefCell, rc::Rc};

pub type Link = Rc<RefCell<Option<GNode>>>;

pub struct GNode {
    value: usize,
}
