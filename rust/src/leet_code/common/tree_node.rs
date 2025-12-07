// rust/src/leet_code/common/tree_node.rs

use std::cell::RefCell;
use std::collections::VecDeque;
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

pub fn vec_to_bst(values: Vec<Option<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
    build_tree2(values)
}

pub fn bst_to_vec(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Option<i32>> {
    let mut vec = Vec::new();

    if root.is_none() {
        return vec;
    }

    let mut queue = VecDeque::new();
    queue.push_back(root);

    while !queue.is_empty() {
        if let Some(node) = queue.pop_front() {
            match node {
                Some(n) => {
                    let n_ref = n.borrow();
                    vec.push(Some(n.borrow().val));
                    match &n_ref.left {
                        Some(left) => queue.push_back(Some(Rc::clone(left))),
                        None => queue.push_back(None),
                    }
                    match &n_ref.right {
                        Some(right) => queue.push_back(Some(Rc::clone(right))),
                        None => queue.push_back(None),
                    }
                }
                None => vec.push(None),
            }
        }
    }

    trim_trailing_nones(&mut vec);
    vec
}

fn trim_trailing_nones<T>(v: &mut Vec<Option<T>>) {
    if let Some(pos) = v.iter().rposition(|x| x.is_some()) {
        v.truncate(pos + 1);
    } else {
        v.clear(); // all None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let values = vec![Some(1), None, Some(2), Some(3)];
        let root = vec_to_bst(values);

        // Root exists
        assert!(root.is_some());
        let root = root.unwrap();
        assert_eq!(root.borrow().val, 1);

        // Left child is None
        assert!(root.borrow().left.is_none());

        // Right child exists
        assert!(root.borrow().right.is_some());
        let right = Rc::clone(root.borrow().right.as_ref().unwrap()); // borrow
        assert_eq!(right.borrow().val, 2);

        // Right's left child exists
        assert!(right.borrow().left.is_some());
        let right_left = Rc::clone(right.borrow().left.as_ref().unwrap());
        assert_eq!(right_left.borrow().val, 3);

        // Right's right child is None
        assert!(right.borrow().right.is_none());
    }

    #[test]
    fn test2() {
        let values = vec![Some(1), None, Some(2), Some(3)];
        let root = vec_to_bst(values.clone());
        let result = bst_to_vec(root);
        println!("dump");
        for val in &result {
            match val {
                Some(v) => println!("{}", v),
                None => println!("None"),
            }
        }
        assert_eq!(result, values);
    }
}
