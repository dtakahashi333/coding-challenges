// rust/src/leet_code/easy/is_symmetric.rs

// 101. Symmetric Tree
// https://leetcode.com/problems/symmetric-tree/

use crate::leet_code::common::tree_node::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
pub struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn is_symmetric(&self, root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if root.is_none() {
            return true;
        }

        let mut left = root.as_ref().cloned();
        let mut right = root.as_ref().cloned();
        let mut left_stack = Vec::new();
        let mut right_stack = Vec::new();

        while (left.is_some() && right.is_some())
            || (!left_stack.is_empty() && !right_stack.is_empty())
        {
            while left.is_some() && right.is_some() {
                // let l = left.as_ref().unwrap().clone();
                // let r = right.as_ref().unwrap().clone();
                let l = left.take().unwrap();
                let r = right.take().unwrap();
                if l.borrow().val != r.borrow().val {
                    return false;
                }
                left = l.borrow().left.clone();
                right = r.borrow().right.clone();
                left_stack.push(l);
                right_stack.push(r);
            }
            if left.is_some() || right.is_some() {
                return false;
            }
            if let (Some(l), Some(r)) = (left_stack.pop(), right_stack.pop()) {
                left = l.borrow().right.clone();
                right = r.borrow().left.clone();
            } else {
                return false;
            }
        }
        left.is_none() && right.is_none() && left_stack.is_empty() && right_stack.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::leet_code::common::tree_node::build_tree2;

    #[test]
    fn test1() {
        let root = build_tree2(vec![1, 2, 2, 3, 4, 4, 3].iter().map(|x| Some(*x)).collect());
        let s = Solution;
        let result = s.is_symmetric(root);
        assert_eq!(result, true);
    }

    #[test]
    fn test2() {
        let root = build_tree2(vec![
            Some(1),
            Some(2),
            Some(2),
            None,
            Some(3),
            None,
            Some(3),
        ]);
        let s = Solution;
        let result = s.is_symmetric(root);
        assert_eq!(result, false);
    }
}
