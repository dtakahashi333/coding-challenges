// rust/src/leet_code/easy/is_balanced.rs

// 110. Balanced Binary Tree
// https://leetcode.com/problems/balanced-binary-tree/description/

use crate::leet_code::common::tree_node::TreeNode;

use std::cell::RefCell;
use std::cmp;
use std::rc::Rc;

#[derive(Debug)]
pub struct Solution;

impl Solution {
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let mut depth = 0;
        Self::is_balanced_helper(&root, &mut depth)
    }

    fn is_balanced_helper(node: &Option<Rc<RefCell<TreeNode>>>, depth: &mut i32) -> bool {
        if node.is_none() {
            *depth = 0;
            return true;
        }
        let mut left_depth = 0;
        let mut right_depth = 0;
        let mut is_balanced = true;
        if let Some(n) = node {
            let n_ref = n.borrow();
            is_balanced = Self::is_balanced_helper(&n_ref.left, &mut left_depth)
                && Self::is_balanced_helper(&n_ref.right, &mut right_depth);
        }
        *depth = cmp::max(left_depth, right_depth) + 1;
        is_balanced && (left_depth - right_depth).abs() <= 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::leet_code::common::tree_node::vec_to_bst;

    #[test]
    fn test1() {
        let root = vec_to_bst(vec![
            Some(3),
            Some(9),
            Some(20),
            None,
            None,
            Some(15),
            Some(7),
        ]);
        let result = Solution::is_balanced(root);
        assert!(result);
    }

    #[test]
    fn test2() {
        let root = vec_to_bst(vec![
            Some(1),
            Some(2),
            Some(2),
            Some(3),
            Some(3),
            None,
            None,
            Some(4),
            Some(4),
        ]);
        let result = Solution::is_balanced(root);
        assert!(!result);
    }

    #[test]
    fn test3() {
        let root = vec_to_bst(vec![]);
        let result = Solution::is_balanced(root);
        assert!(result);
    }
}
