use std::fmt::Display;

#[derive(Clone)]
pub struct LNode {
    val: usize,
    next: Option<Box<LNode>>,
}

impl Display for LNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Linked Node: {} ", self.val)
    }
}

impl Default for LNode {
    fn default() -> LNode {
        LNode { val: 0, next: None }
    }
}

impl LNode {
    pub fn new(val: usize, next: Option<Box<LNode>>) -> Self {
        Self { val, next }
    }

    pub fn create_linked_list(nums: Vec<usize>) -> Result<LNode, anyhow::Error> {
        if nums.is_empty() {
            return Ok(LNode::default());
        }

        if nums.len() == 1 {
            if let Some(val) = nums.first() {
                return Ok(LNode::new(*val, None));
            } else {
                return Err(anyhow::anyhow!("could not parse vector"));
            }
        }

        let head: LNode;

        if let Some(val) = nums.first() {
            head = LNode::new(*val, None)
        } else {
            return Err(anyhow::anyhow!("could not parse vector"));
        }

        let mut curr: LNode = head.clone();

        for num in nums {
            curr = LNode::new(num, Some(Box::new(curr)))
        }

        Ok(head)
    }
}
