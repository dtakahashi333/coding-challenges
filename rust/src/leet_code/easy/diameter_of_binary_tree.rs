// rust/src/leet_code/easy/diameter_of_binary_tree.rs

// 543. Diameter of Binary Tree
// https://leetcode.com/problems/diameter-of-binary-tree/description/

use std::{cell::RefCell, cmp::max, rc::Rc};

use crate::common::tree_node::TreeNode;

pub struct Solution;

impl Solution {
    pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(node) = root {
            let n_ref = node.borrow();
            let left_height = Self::height(n_ref.left.clone());
            let right_height = Self::height(n_ref.right.clone());

            max(
                left_height + right_height,
                max(
                    Self::diameter_of_binary_tree(n_ref.left.clone()),
                    Self::diameter_of_binary_tree(n_ref.right.clone()),
                ),
            )
        } else {
            0
        }
    }

    fn height(node: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(n) = node {
            let n_ref = n.borrow();
            1 + max(
                Self::height(n_ref.left.clone()),
                Self::height(n_ref.right.clone()),
            )
        } else {
            0
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::common::tree_node::vec_to_bst;

    use super::*;

    #[test]
    fn test1() {
        let root = vec_to_bst(vec![Some(1), Some(2), Some(3), Some(4), Some(5)]);
        let result = Solution::diameter_of_binary_tree(root);
        assert_eq!(result, 3);
    }

    #[test]
    fn test2() {
        let root = vec_to_bst(vec![Some(1), Some(2)]);
        let result = Solution::diameter_of_binary_tree(root);
        assert_eq!(result, 1);
    }

    #[test]
    fn test3() {
        let root = vec_to_bst(vec![
            Some(4),
            Some(-7),
            Some(-3),
            None,
            None,
            Some(-9),
            Some(-3),
            Some(9),
            Some(-7),
            Some(-4),
            None,
            Some(6),
            None,
            Some(-6),
            Some(-6),
            None,
            None,
            Some(0),
            Some(6),
            Some(5),
            None,
            Some(9),
            None,
            None,
            Some(-1),
            Some(-4),
            None,
            None,
            None,
            Some(2),
        ]);
        let result = Solution::diameter_of_binary_tree(root);
        assert_eq!(result, 8);
    }
}
