// rust/src/leet_code/easy/postorder_traversal.rs

// 145. Binary Tree Postorder Traversal
// https://leetcode.com/problems/binary-tree-postorder-traversal/description/

use crate::common::tree_node::TreeNode;

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

#[derive(Debug)]
pub struct Solution;

impl Solution {
    pub fn postorder_traversal_recursive(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result = Vec::new();
        Self::postorder_traversal_recursive_helper(&root, &mut result);
        result
    }

    fn postorder_traversal_recursive_helper(
        node: &Option<Rc<RefCell<TreeNode>>>,
        result: &mut Vec<i32>,
    ) {
        if let Some(n) = node {
            let n_ref = n.borrow();
            Self::postorder_traversal_recursive_helper(&n_ref.left, result);
            Self::postorder_traversal_recursive_helper(&n_ref.right, result);
            result.push(n_ref.val);
        }
    }

    pub fn postorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        if root.is_none() {
            return vec![];
        }

        let mut res = Vec::<i32>::new();
        let mut deque = VecDeque::new();
        let mut cur = root;

        while cur.is_some() || !deque.is_empty() {
            while let Some(node) = cur {
                res.push(node.borrow().val);
                cur = node.borrow().right.clone();
                deque.push_back(node);
            }

            if let Some(node) = deque.pop_back() {
                cur = node.borrow().left.clone();
            }
        }

        res.reverse();

        res
    }

    pub fn postorder_traversal2(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
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
    use crate::common::tree_node::vec_to_bst;

    use super::*;

    #[test]
    fn test1() {
        let list = vec![Some(1), None, Some(2), Some(3)];
        let root = vec_to_bst(list);
        let result = Solution::postorder_traversal_recursive(root);
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
        let result = Solution::postorder_traversal_recursive(root);
        assert_eq!(result, vec![4, 6, 7, 5, 2, 9, 8, 3, 1]);
    }

    #[test]
    fn test3() {
        let list = vec![];
        let root = vec_to_bst(list);
        let result = Solution::postorder_traversal_recursive(root);
        assert_eq!(result, vec![]);
    }

    #[test]
    fn test4() {
        let list = vec![Some(1)];
        let root = vec_to_bst(list);
        let result = Solution::postorder_traversal_recursive(root);
        assert_eq!(result, vec![1]);
    }

    #[test]
    fn test5() {
        let list = vec![Some(1), None, Some(2), Some(3)];
        let root = vec_to_bst(list);
        let result = Solution::postorder_traversal(root);
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
        let result = Solution::postorder_traversal(root);
        assert_eq!(result, vec![4, 6, 7, 5, 2, 9, 8, 3, 1]);
    }

    #[test]
    fn test7() {
        let list = vec![];
        let root = vec_to_bst(list);
        let result = Solution::postorder_traversal(root);
        assert_eq!(result, vec![]);
    }

    #[test]
    fn test8() {
        let list = vec![Some(1)];
        let root = vec_to_bst(list);
        let result = Solution::postorder_traversal(root);
        assert_eq!(result, vec![1]);
    }

    #[test]
    fn test9() {
        let list = vec![Some(1), None, Some(2), Some(3)];
        let root = vec_to_bst(list);
        let result = Solution::postorder_traversal2(root);
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
        let result = Solution::postorder_traversal2(root);
        assert_eq!(result, vec![4, 6, 7, 5, 2, 9, 8, 3, 1]);
    }

    #[test]
    fn test11() {
        let list = vec![];
        let root = vec_to_bst(list);
        let result = Solution::postorder_traversal2(root);
        assert_eq!(result, vec![]);
    }

    #[test]
    fn test12() {
        let list = vec![Some(1)];
        let root = vec_to_bst(list);
        let result = Solution::postorder_traversal2(root);
        assert_eq!(result, vec![1]);
    }
}
