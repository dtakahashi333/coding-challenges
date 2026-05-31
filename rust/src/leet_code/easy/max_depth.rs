// rust/src/leet_code/easy/max_depth.rs

// 104. Maximum Depth of Binary Tree
// https://leetcode.com/problems/maximum-depth-of-binary-tree/description/

use crate::common::tree_node::TreeNode;

use std::cell::RefCell;
use std::cmp::max;
use std::collections::VecDeque;
use std::rc::Rc;

#[derive(Debug)]
pub struct Solution;

impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() {
            return 0;
        }

        let mut queue = VecDeque::new();
        queue.push_back(root.unwrap());

        let mut depth = 0;
        while !queue.is_empty() {
            let queue_size = queue.len();
            for _ in 0..queue_size {
                if let Some(n) = queue.pop_front() {
                    if let Some(left) = n.borrow().left.clone() {
                        queue.push_back(left);
                    }
                    if let Some(right) = n.borrow().right.clone() {
                        queue.push_back(right);
                    }
                }
            }
            depth += 1;
        }

        depth
    }

    pub fn max_depth2(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(node) = root {
            let n_ref = node.borrow();
            1 + max(
                Self::max_depth(n_ref.left.clone()),
                Self::max_depth(n_ref.right.clone()),
            )
        } else {
            0
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use crate::common::tree_node::vec_to_bst;

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
        let result = Solution::max_depth(root);
        assert_eq!(result, 3);
    }

    #[test]
    fn test2() {
        let root = vec_to_bst(vec![Some(1), None, Some(2)]);
        let result = Solution::max_depth(root);
        assert_eq!(result, 2);
    }

    #[test]
    fn test3() {
        let root = vec_to_bst(vec![
            Some(3),
            Some(9),
            Some(20),
            None,
            None,
            Some(15),
            Some(7),
        ]);
        let result = Solution::max_depth2(root);
        assert_eq!(result, 3);
    }

    #[test]
    fn test4() {
        let root = vec_to_bst(vec![Some(1), None, Some(2)]);
        let result = Solution::max_depth2(root);
        assert_eq!(result, 2);
    }
}
