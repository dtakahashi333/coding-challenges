// rust/src/leet_code/easy/inorder_traversal.rs

// 94. Binary Tree Inorder Traversal
// https://leetcode.com/problems/binary-tree-inorder-traversal/description/

use crate::leet_code::common::tree_node::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
pub struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn inorder_traversal_recursive(&self, root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result = Vec::new();
        self.inorder_traversal_recursive_helper(&root, &mut result);
        result
    }

    fn inorder_traversal_recursive_helper(
        &self,
        tree_node: &Option<Rc<RefCell<TreeNode>>>,
        list: &mut Vec<i32>,
    ) {
        if let Some(node) = tree_node {
            self.inorder_traversal_recursive_helper(&node.borrow().left, list);
            list.push(node.borrow().val);
            self.inorder_traversal_recursive_helper(&node.borrow().right, list);
        }
    }

    #[allow(dead_code)]
    pub fn inorder_traversal(&self, root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result = Vec::new();

        if root.is_none() {
            return result;
        }

        // node: Option<Rc<RefCell<TreeNode>>>
        let mut node = root.as_ref().cloned();
        let mut stack = Vec::new();

        while node.is_some() || !stack.is_empty() {
            while let Some(n) = node {
                let left = n.borrow().left.clone();
                stack.push(n);
                node = left;
            }
            // node == None
            if !stack.is_empty() {
                if let Some(n) = stack.pop() {
                    result.push(n.borrow().val);
                    node = n.borrow().right.clone();
                }
            }
        }

        result
    }

    // ChatGPT
    #[allow(dead_code)]
    pub fn inorder_traversal2(&self, root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result = Vec::new();
        let mut stack = Vec::new();
        let mut node = root;

        while node.is_some() || !stack.is_empty() {
            while let Some(n) = node {
                node = n.borrow().left.clone(); // go left
                stack.push(n);
            }
            node = stack.pop();
            if let Some(n) = node {
                result.push(n.borrow().val); // visit node
                node = n.borrow().right.clone(); // go right
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::common::tree_node::build_tree2;

    use super::*;

    #[test]
    fn test1() {
        let list = vec![Some(1), None, Some(2), Some(3)];
        let root = build_tree2(list);
        let s = Solution;
        let result = s.inorder_traversal_recursive(root);
        assert_eq!(result, vec![1, 3, 2]);
    }

    #[test]
    fn test2() {
        let list = vec![
            Some(1),
            Some(2),
            Some(3),
            Some(4),
            Some(5),
            None,
            Some(8),
            None,
            None,
            Some(6),
            Some(7),
            Some(9),
        ];
        let root = build_tree2(list);
        let s = Solution;
        let result = s.inorder_traversal_recursive(root);
        assert_eq!(result, vec![4, 2, 6, 5, 7, 1, 3, 9, 8]);
    }

    #[test]
    fn test3() {
        let list = vec![];
        let root = build_tree2(list);
        let s = Solution;
        let result = s.inorder_traversal_recursive(root);
        assert_eq!(result, vec![]);
    }

    #[test]
    fn test4() {
        let list = vec![Some(1)];
        let root = build_tree2(list);
        let s = Solution;
        let result = s.inorder_traversal_recursive(root);
        assert_eq!(result, vec![1]);
    }

    #[test]
    fn test5() {
        let list = vec![Some(1), None, Some(2), Some(3)];
        let root = build_tree2(list);
        let s = Solution;
        let result = s.inorder_traversal(root);
        assert_eq!(result, vec![1, 3, 2]);
    }

    #[test]
    fn test6() {
        let list = vec![
            Some(1),
            Some(2),
            Some(3),
            Some(4),
            Some(5),
            None,
            Some(8),
            None,
            None,
            Some(6),
            Some(7),
            Some(9),
        ];
        let root = build_tree2(list);
        let s = Solution;
        let result = s.inorder_traversal(root);
        assert_eq!(result, vec![4, 2, 6, 5, 7, 1, 3, 9, 8]);
    }

    #[test]
    fn test7() {
        let list = vec![];
        let root = build_tree2(list);
        let s = Solution;
        let result = s.inorder_traversal(root);
        assert_eq!(result, vec![]);
    }

    #[test]
    fn test8() {
        let list = vec![Some(1)];
        let root = build_tree2(list);
        let s = Solution;
        let result = s.inorder_traversal(root);
        assert_eq!(result, vec![1]);
    }
}
