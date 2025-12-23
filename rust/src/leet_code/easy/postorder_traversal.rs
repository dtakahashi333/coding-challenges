// rust/src/leet_code/easy/postorder_traversal.rs

// 145. Binary Tree Postorder Traversal
// https://leetcode.com/problems/binary-tree-postorder-traversal/description/

use crate::leet_code::common::tree_node::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
pub struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn postorder_traversal_recursive(&self, root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result = Vec::new();
        self.postorder_traversal_recursive_helper(&root, &mut result);
        result
    }

    fn postorder_traversal_recursive_helper(
        &self,
        node: &Option<Rc<RefCell<TreeNode>>>,
        result: &mut Vec<i32>,
    ) {
        if let Some(n) = node {
            let n_ref = n.borrow();
            self.postorder_traversal_recursive_helper(&n_ref.left, result);
            self.postorder_traversal_recursive_helper(&n_ref.right, result);
            result.push(n_ref.val);
        }
    }

    #[allow(dead_code)]
    pub fn postorder_traversal(&self, root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result = Vec::new();
        if root.is_none() {
            return result;
        }
        let mut stack = Vec::new();
        stack.push(root.as_ref().cloned().unwrap());
        while !stack.is_empty() {
            if let Some(n) = stack.pop() {
                let n_ref = n.borrow();
                result.push(n_ref.val);
                if let Some(left) = &n_ref.left {
                    stack.push(Rc::clone(left));
                }
                if let Some(right) = &n_ref.right {
                    stack.push(Rc::clone(right));
                }
            }
        }
        result.reverse();
        result
    }

    #[allow(dead_code)]
    pub fn postorder_traversal2(&self, root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result = Vec::new();
        if root.is_none() {
            return result;
        }
        let mut stack = vec![(root.unwrap(), false)];

        while let Some((node, visited)) = stack.pop() {
            let node_ref = node.borrow();

            if visited {
                result.push(node_ref.val);
            } else {
                stack.push((node.clone(), true));
                if let Some(r) = node_ref.right.clone() {
                    stack.push((r, false));
                }
                if let Some(l) = node_ref.left.clone() {
                    stack.push((l, false));
                }
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
        let result = s.postorder_traversal_recursive(root);
        assert_eq!(result, vec![3, 2, 1]);
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
        let result = s.postorder_traversal_recursive(root);
        assert_eq!(result, vec![4, 6, 7, 5, 2, 9, 8, 3, 1]);
    }

    #[test]
    fn test3() {
        let list = vec![];
        let root = vec_to_bst(list);
        let s = Solution;
        let result = s.postorder_traversal_recursive(root);
        assert_eq!(result, vec![]);
    }

    #[test]
    fn test4() {
        let list = vec![Some(1)];
        let root = vec_to_bst(list);
        let s = Solution;
        let result = s.postorder_traversal_recursive(root);
        assert_eq!(result, vec![1]);
    }

    #[test]
    fn test5() {
        let list = vec![Some(1), None, Some(2), Some(3)];
        let root = vec_to_bst(list);
        let s = Solution;
        let result = s.postorder_traversal(root);
        assert_eq!(result, vec![3, 2, 1]);
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
        let result = s.postorder_traversal(root);
        assert_eq!(result, vec![4, 6, 7, 5, 2, 9, 8, 3, 1]);
    }

    #[test]
    fn test7() {
        let list = vec![];
        let root = vec_to_bst(list);
        let s = Solution;
        let result = s.postorder_traversal(root);
        assert_eq!(result, vec![]);
    }

    #[test]
    fn test8() {
        let list = vec![Some(1)];
        let root = vec_to_bst(list);
        let s = Solution;
        let result = s.postorder_traversal(root);
        assert_eq!(result, vec![1]);
    }

    #[test]
    fn test9() {
        let list = vec![Some(1), None, Some(2), Some(3)];
        let root = vec_to_bst(list);
        let s = Solution;
        let result = s.postorder_traversal2(root);
        assert_eq!(result, vec![3, 2, 1]);
    }

    #[test]
    fn test10() {
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
        let result = s.postorder_traversal2(root);
        assert_eq!(result, vec![4, 6, 7, 5, 2, 9, 8, 3, 1]);
    }

    #[test]
    fn test11() {
        let list = vec![];
        let root = vec_to_bst(list);
        let s = Solution;
        let result = s.postorder_traversal2(root);
        assert_eq!(result, vec![]);
    }

    #[test]
    fn test12() {
        let list = vec![Some(1)];
        let root = vec_to_bst(list);
        let s = Solution;
        let result = s.postorder_traversal2(root);
        assert_eq!(result, vec![1]);
    }
}
