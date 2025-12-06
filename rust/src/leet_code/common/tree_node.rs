// rust/src/leet_code/common/tree_node.rs

use std::cell::RefCell;
use std::rc::Rc;

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

pub fn build_tree(values: Vec<Option<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
    use std::collections::VecDeque;

    if values.is_empty() || values[0].is_none() {
        return None;
    }

    let root = Rc::new(RefCell::new(TreeNode::new(values[0].unwrap())));
    let mut queue = VecDeque::new();
    queue.push_back(Rc::clone(&root));

    let mut i = 1;
    while i < values.len() {
        if let Some(node) = queue.pop_front() {
            // Left child
            if let Some(Some(val)) = values.get(i) {
                let left = Rc::new(RefCell::new(TreeNode::new(*val)));
                node.borrow_mut().left = Some(Rc::clone(&left));
                queue.push_back(left);
            }
            i += 1;

            // Right child
            if let Some(Some(val)) = values.get(i) {
                let right = Rc::new(RefCell::new(TreeNode::new(*val)));
                node.borrow_mut().right = Some(Rc::clone(&right));
                queue.push_back(right);
            }
            i += 1;
        }
    }

    Some(root)
}

pub fn build_tree2(values: Vec<Option<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
    use std::collections::VecDeque;

    if values.is_empty() || values[0].is_none() {
        return None;
    }

    let root = Rc::new(RefCell::new(TreeNode::new(values[0].unwrap())));
    let mut queue = VecDeque::new();
    queue.push_back(Rc::clone(&root));

    let mut i = 1_usize;
    while i < values.len() && !queue.is_empty() {
        let node = queue.pop_front().unwrap();
        if let Some(val) = values[i] {
            let left = Rc::new(RefCell::new(TreeNode::new(val)));
            node.borrow_mut().left = Some(Rc::clone(&left));
            // at this point, left is still the owner of the TreeNode, so it is safely moved to the queue.
            queue.push_back(left);
        }
        i += 1;
        if i < values.len() {
            if let Some(val) = values[i] {
                let right = Rc::new(RefCell::new(TreeNode::new(val)));
                node.borrow_mut().right = Some(Rc::clone(&right));
                queue.push_back(right);
            }
        }
        i += 1;
    }

    Some(root)
}
