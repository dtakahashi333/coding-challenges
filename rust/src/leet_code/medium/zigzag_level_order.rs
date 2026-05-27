// rust/src/dsa/binary_tree/zigzag_level_order.rs

// 103. Binary Tree Zigzag Level Order Traversal
// https://leetcode.com/problems/binary-tree-zigzag-level-order-traversal/description/

use std::{cell::RefCell, collections::VecDeque, rc::Rc};

use crate::leet_code::common::tree_node::TreeNode;

pub struct Solution;

impl Solution {
    pub fn zigzag_level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        if root.is_none() {
            return vec![];
        }
        let mut res = Vec::<Vec<i32>>::new();
        let mut deque = VecDeque::new();
        deque.push_back(root.unwrap());

        let mut left_to_right = true;

        while !deque.is_empty() {
            let size = deque.len();
            let mut cur_level = VecDeque::new();

            for _ in 0..size {
                if let Some(node) = deque.pop_front() {
                    let n_ref = node.borrow();

                    if left_to_right {
                        cur_level.push_back(n_ref.val);
                    } else {
                        cur_level.push_front(n_ref.val);
                    }

                    if let Some(left) = n_ref.left.clone() {
                        deque.push_back(left);
                    }
                    if let Some(right) = n_ref.right.clone() {
                        deque.push_back(right);
                    }
                }
            }

            res.push(Vec::from(cur_level));

            left_to_right = !left_to_right;
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::common::tree_node::vec_to_bst;

    use super::*;

    #[test]
    fn test1() {
        let list = vec![Some(3), Some(9), Some(20), None, None, Some(15), Some(7)];
        let root = vec_to_bst(list);
        let result = Solution::zigzag_level_order(root);
        assert_eq!(result, vec![vec![3], vec![20, 9], vec![15, 7]]);
    }

    #[test]
    fn test2() {
        let list = vec![];
        let root = vec_to_bst(list);
        let result = Solution::zigzag_level_order(root);
        assert_eq!(result, vec![] as Vec<Vec<i32>>);
    }

    #[test]
    fn test3() {
        let list = vec![Some(1)];
        let root = vec_to_bst(list);
        let result = Solution::zigzag_level_order(root);
        assert_eq!(result, vec![vec![1]]);
    }
}
