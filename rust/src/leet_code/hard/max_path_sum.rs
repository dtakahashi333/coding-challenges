// rust/src/leet_code/hard/max_path_sum.rs

// 124. Binary Tree Maximum Path Sum
// https://leetcode.com/problems/binary-tree-maximum-path-sum/description/

use std::cell::RefCell;
use std::cmp::max;
use std::rc::Rc;

use crate::common::tree_node::TreeNode;
pub struct Solution;

impl Solution {
    pub fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(node) = root {
            let n_ref = node.borrow();
            let left_down = max(0, Self::downward(n_ref.left.clone()));
            let right_down = max(0, Self::downward(n_ref.right.clone()));

            max(
                n_ref.val + left_down + right_down,
                max(
                    Self::max_path_sum(n_ref.left.clone()),
                    Self::max_path_sum(n_ref.right.clone()),
                ),
            )
        } else {
            i32::MIN
        }
    }

    fn downward(node: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(n) = node {
            let n_ref = n.borrow();
            n_ref.val
                + max(
                    0,
                    max(
                        Self::downward(n_ref.left.clone()),
                        Self::downward(n_ref.right.clone()),
                    ),
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
        let root = vec_to_bst(vec![Some(1), Some(2), Some(3)]);
        let result = Solution::max_path_sum(root);
        assert_eq!(result, 6);
    }

    #[test]
    fn test2() {
        let root = vec_to_bst(vec![
            Some(-10),
            Some(9),
            Some(20),
            None,
            None,
            Some(15),
            Some(7),
        ]);
        let result = Solution::max_path_sum(root);
        assert_eq!(result, 42);
    }

    #[test]
    fn test3() {
        let root = vec_to_bst(vec![Some(-3)]);
        let result = Solution::max_path_sum(root);
        assert_eq!(result, -3);
    }

    #[test]
    fn test4() {
        let root = vec_to_bst(vec![Some(2), Some(-1)]);
        let result = Solution::max_path_sum(root);
        assert_eq!(result, 2);
    }
}
