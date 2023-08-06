use std::fmt::Display;

#[derive(Clone, Debug, Default)]
pub struct LNode {
    val: usize,
    next: Option<Box<LNode>>,
}

impl Display for LNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Linked Node: {} ", self.val)
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

        let mut head: LNode;
        let mut curr: &mut Option<Box<LNode>>;

        if let Some(num) = nums.first() {
            head = LNode::new(*num, None);
        } else {
            return Err(anyhow::anyhow!("issue with first"));
        }

        if let Some(num) = nums.get(1) {
            head.next = Some(Box::new(LNode::new(*num, None)));
            curr = &mut head.next;
        } else {
            return Err(anyhow::anyhow!("issue with first"));
        }

        for i in 2..nums.len() {
            if let Some(num) = nums.get(i) {
                match curr {
                    Some(boxed_val) => {
                        boxed_val.next = Some(Box::new(LNode::new(*num, None)));
                        curr = &mut boxed_val.next;
                    }
                    None => todo!(),
                }
            } else {
                return Err(anyhow::anyhow!("issue with first"));
            }
        }

        Ok(head)
    }

    pub fn print_list(head: LNode) {
        println!("head.next: {:?}", head);
    }
}
