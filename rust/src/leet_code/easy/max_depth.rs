// rust/src/leet_code/easy/max_depth.rs

// 104. Maximum Depth of Binary Tree
// https://leetcode.com/problems/maximum-depth-of-binary-tree/

use crate::leet_code::common::tree_node::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
pub struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn max_depth(&self, root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        use std::collections::VecDeque;

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
}

#[cfg(test)]
mod test {
    use super::*;

    use crate::leet_code::common::tree_node::build_tree2;

    #[test]
    fn test1() {
        let root = build_tree2(vec![
            Some(3),
            Some(9),
            Some(20),
            None,
            None,
            Some(15),
            Some(7),
        ]);
        let s = Solution;
        let result = s.max_depth(root);
        assert_eq!(result, 3);
    }

    #[test]
    fn test2() {
        let root = build_tree2(vec![Some(1), None, Some(2)]);
        let s = Solution;
        let result = s.max_depth(root);
        assert_eq!(result, 2);
    }
}
