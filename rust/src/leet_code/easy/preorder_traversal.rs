// rust/src/leet_code/easy/preorder_traversal.rs

// 144. Binary Tree Preorder Traversal
// https://leetcode.com/problems/binary-tree-preorder-traversal/

use crate::leet_code::common::tree_node::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
pub struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn preorder_traversal_recursive(&self, root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result = Vec::new();
        self.preorder_traversal_recursive_helper(&root, &mut result);
        result
    }

    fn preorder_traversal_recursive_helper(
        &self,
        node: &Option<Rc<RefCell<TreeNode>>>,
        result: &mut Vec<i32>,
    ) {
        if let Some(n) = node {
            let n_ref = n.borrow();
            result.push(n_ref.val);
            self.preorder_traversal_recursive_helper(&n_ref.left, result);
            self.preorder_traversal_recursive_helper(&n_ref.right, result);
        }
    }

    #[allow(dead_code)]
    pub fn preorder_traversal(&self, root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result = Vec::new();
        if root.is_none() {
            return result;
        }
        let mut node = root;
        let mut stack = Vec::new();
        while node.is_some() || !stack.is_empty() {
            while let Some(n) = node {
                { 
                    let n_ref = n.borrow();
                    result.push(n_ref.val);
                    node = n_ref.left.clone();
                }
                stack.push(n);
                // // Increase the reference counter
                // let n2 = Rc::clone(&n);
                // result.push(n2.borrow().val);
                // node = n2.borrow().left.clone();
                // stack.push(n);
            }
            if let Some(n) = stack.pop() {
                node = n.borrow().right.clone();
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::common::tree_node::vec_to_bst;

    use super::*;

    #[test]
    fn test1() {
        let list = vec![Some(1), None, Some(2), Some(3)];
        let root = vec_to_bst(list);
        let s = Solution;
        let result = s.preorder_traversal_recursive(root);
        assert_eq!(result, vec![1, 2, 3]);
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
        let root = vec_to_bst(list);
        let s = Solution;
        let result = s.preorder_traversal_recursive(root);
        assert_eq!(result, vec![1, 2, 4, 5, 6, 7, 3, 8, 9]);
    }

    #[test]
    fn test3() {
        let list = vec![];
        let root = vec_to_bst(list);
        let s = Solution;
        let result = s.preorder_traversal_recursive(root);
        assert_eq!(result, vec![]);
    }

    #[test]
    fn test4() {
        let list = vec![Some(1)];
        let root = vec_to_bst(list);
        let s = Solution;
        let result = s.preorder_traversal_recursive(root);
        assert_eq!(result, vec![1]);
    }

    #[test]
    fn test5() {
        let list = vec![Some(1), None, Some(2), Some(3)];
        let root = vec_to_bst(list);
        let s = Solution;
        let result = s.preorder_traversal(root);
        assert_eq!(result, vec![1, 2, 3]);
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
        let root = vec_to_bst(list);
        let s = Solution;
        let result = s.preorder_traversal(root);
        assert_eq!(result, vec![1, 2, 4, 5, 6, 7, 3, 8, 9]);
    }

    #[test]
    fn test7() {
        let list = vec![];
        let root = vec_to_bst(list);
        let s = Solution;
        let result = s.preorder_traversal(root);
        assert_eq!(result, vec![]);
    }

    #[test]
    fn test8() {
        let list = vec![Some(1)];
        let root = vec_to_bst(list);
        let s = Solution;
        let result = s.preorder_traversal(root);
        assert_eq!(result, vec![1]);
    }
}
