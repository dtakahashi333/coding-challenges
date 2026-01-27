// rust/src/leetcode/common/list_node.rs

use core::fmt;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

impl fmt::Display for ListNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // opening bracket
        write!(f, "list: [")?;

        let mut node = Some(self);
        let mut first = true;
        let mut count = 0;
        while let Some(n) = node {
            if !first {
                write!(f, ", ")?; // add comma between nodes
            }
            write!(f, "{}", n.val)?; // write current value
            node = n.next.as_deref();
            first = false;
            count += 1;
        }

        // closing bracket
        writeln!(f, "]")?;

        write!(f, "size: {}", count)?;

        Ok(())
    }
}

// Helper: convert Vec<i32> → Option<Box<ListNode>>
#[allow(dead_code)]
pub fn vec_to_list(nums: Vec<i32>) -> Option<Box<ListNode>> {
    let mut current = None;

    for &num in nums.iter().rev() {
        let mut node = Box::new(ListNode::new(num));
        node.next = current;
        current = Some(node);
    }

    current
}

// Helper: convert Option<Box<ListNode>> → Vec<i32>
#[allow(dead_code)]
pub fn list_to_vec(mut list: Option<Box<ListNode>>) -> Vec<i32> {
    let mut result = vec![];

    while let Some(node) = list {
        result.push(node.val);
        list = node.next;
    }

    result
}
