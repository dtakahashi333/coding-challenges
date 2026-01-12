// rust/src/leet_code/easy/count_nodes.rs

// 222. Count Complete Tree Nodes
// https://leetcode.com/problems/count-complete-tree-nodes/description/

use std::{cell::RefCell, rc::Rc};

use crate::leet_code::common::tree_node::TreeNode;

#[derive(Debug)]
pub struct Solution;

impl Solution {
    pub fn count_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
            Some(node) => Self::count_nodes_helper(&Some(node)),
            None => 0,
        }
    }

    fn count_nodes_helper(node: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match node {
            Some(n) => {
                let n_ref = n.borrow();
                1 + Self::count_nodes_helper(&n_ref.left) + Self::count_nodes_helper(&n_ref.right)
            }
            None => 0,
        }
    }

    pub fn count_nodes2(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        use std::collections::VecDeque;

        if root.is_some() {
            let mut queue = VecDeque::new();
            queue.push_back(root);

            let mut node_count = 0;

            while let Some(node) = queue.pop_front() {
                let left_depth = Self::get_left_depth(node.clone());
                let right_depth = Self::get_right_depth(node.clone());
                if left_depth == right_depth {
                    node_count += (1 << left_depth) - 1;
                } else if let Some(n) = node {
                    queue.push_back(n.borrow().left.clone());
                    queue.push_back(n.borrow().right.clone());
                    node_count += 1;
                }
            }
            node_count
        } else {
            0
        }
    }

    pub fn count_nodes3(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        // Base case
        if root.is_none() {
            return 0;
        }

        // Compute left and right depths
        let left_depth = Self::get_left_depth(root.clone());
        let right_depth = Self::get_right_depth(root.clone());

        if left_depth == right_depth {
            // Perfect binary tree
            return (1 << left_depth) - 1; // 2^depth - 1
        }

        // Otherwise, recursively count left and right
        let root_ref = root.unwrap();
        let left_count = Self::count_nodes(root_ref.borrow().left.clone());
        let right_count = Self::count_nodes(root_ref.borrow().right.clone());

        1 + left_count + right_count
    }

    fn get_left_depth(mut node: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut depth = 0;
        while let Some(n) = node {
            depth += 1;
            node = n.borrow().left.clone();
        }
        depth
    }

    fn get_right_depth(mut node: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut depth = 0;
        while let Some(n) = node {
            depth += 1;
            node = n.borrow().right.clone();
        }
        depth
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::common::tree_node::vec_to_bst;

    use super::*;

    #[test]
    fn test1() {
        let root = vec_to_bst(vec![Some(1), Some(2), Some(3), Some(4), Some(5), Some(6)]);
        let result = Solution::count_nodes(root);
        assert_eq!(result, 6);
    }

    #[test]
    fn test2() {
        let root = vec_to_bst(vec![]);
        let result = Solution::count_nodes(root);
        assert_eq!(result, 0);
    }

    #[test]
    fn test3() {
        let root = vec_to_bst(vec![Some(1)]);
        let result = Solution::count_nodes(root);
        assert_eq!(result, 1);
    }

    #[test]
    fn test4() {
        let root = vec_to_bst(vec![Some(1), Some(2), Some(3), Some(4), Some(5), Some(6)]);
        let result = Solution::count_nodes2(root);
        assert_eq!(result, 6);
    }

    #[test]
    fn test5() {
        let root = vec_to_bst(vec![]);
        let result = Solution::count_nodes2(root);
        assert_eq!(result, 0);
    }

    #[test]
    fn test6() {
        let root = vec_to_bst(vec![Some(1)]);
        let result = Solution::count_nodes2(root);
        assert_eq!(result, 1);
    }

    #[test]
    fn test7() {
        let root = vec_to_bst(vec![Some(1), Some(2), Some(3), Some(4), Some(5), Some(6)]);
        let result = Solution::count_nodes3(root);
        assert_eq!(result, 6);
    }

    #[test]
    fn test8() {
        let root = vec_to_bst(vec![]);
        let result = Solution::count_nodes3(root);
        assert_eq!(result, 0);
    }

    #[test]
    fn test9() {
        let root = vec_to_bst(vec![Some(1)]);
        let result = Solution::count_nodes3(root);
        assert_eq!(result, 1);
    }
}
