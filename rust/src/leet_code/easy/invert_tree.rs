// rust/src/leet_code/easy/invert_tree.rs

// 226. Invert Binary Tree
// https://leetcode.com/problems/invert-binary-tree/description/

use std::cell::RefCell;
use std::rc::Rc;

use crate::leet_code::common::tree_node::TreeNode;

#[derive(Debug)]
pub struct Solution;

impl Solution {
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::invert_tree_helper(&root)
    }

    fn invert_tree_helper(node: &Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        match node {
            Some(n) => {
                let n_ref = n.borrow();
                let mut inv = TreeNode::new(n_ref.val);
                // switch children
                inv.left = Self::invert_tree_helper(&n_ref.right);
                inv.right = Self::invert_tree_helper(&n_ref.left);
                Some(Rc::new(RefCell::new(inv)))
            }
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::common::tree_node::{bst_to_vec, vec_to_bst};

    use super::*;

    #[test]
    fn test1() {
        let root = vec_to_bst(vec![
            Some(4),
            Some(2),
            Some(7),
            Some(1),
            Some(3),
            Some(6),
            Some(9),
        ]);
        let result = Solution::invert_tree(root);
        assert_eq!(
            bst_to_vec(result),
            vec![
                Some(4),
                Some(7),
                Some(2),
                Some(9),
                Some(6),
                Some(3),
                Some(1)
            ]
        );
    }

    #[test]
    fn test3() {
        let root = vec_to_bst(vec![Some(2), Some(1), Some(3)]);
        let result = Solution::invert_tree(root);
        assert_eq!(bst_to_vec(result), vec![Some(2), Some(3), Some(1)]);
    }
}
