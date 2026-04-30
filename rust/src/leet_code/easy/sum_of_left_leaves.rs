// rust/src/leet_code/easy/sum_of_left_leaves.rs

// 404. Sum of Left Leaves
// https://leetcode.com/problems/sum-of-left-leaves/description/

use std::cell::RefCell;
use std::rc::Rc;

use crate::leet_code::common::tree_node::TreeNode;

pub struct Solution;

impl Solution {
    pub fn sum_of_left_leaves(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut sum = 0;
        Self::sum_of_left_leaves_helper(root, &mut sum);
        sum
    }

    fn sum_of_left_leaves_helper(node: Option<Rc<RefCell<TreeNode>>>, sum: &mut i32) {
        if let Some(n) = &node {
            let n_ref = n.borrow();
            if let Some(left) = &n_ref.left {
                if left.borrow().left.is_none() && left.borrow().right.is_none() {
                    *sum += left.borrow().val;
                } else {
                    Self::sum_of_left_leaves_helper(n_ref.left.clone(), sum);
                }
            }
            if n_ref.right.is_some() {
                Self::sum_of_left_leaves_helper(n_ref.right.clone(), sum);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::common::tree_node::vec_to_bst;

    use super::*;

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
        let result = Solution::sum_of_left_leaves(root);
        assert_eq!(result, 24);
    }

    #[test]
    fn test2() {
        let root = vec_to_bst(vec![Some(1)]);
        let result = Solution::sum_of_left_leaves(root);
        assert_eq!(result, 0);
    }

    #[test]
    fn test3() {
        let root = vec_to_bst(vec![Some(1), Some(2), Some(3), Some(4), Some(5)]);
        let result = Solution::sum_of_left_leaves(root);
        assert_eq!(result, 4);
    }
}
