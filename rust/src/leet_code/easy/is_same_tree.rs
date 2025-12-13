// rust/src/leet_code/easy/is_same_tree.rs

// 100. Same Tree
// https://leetcode.com/problems/same-tree/

use crate::leet_code::common::tree_node::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
pub struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn is_same_tree(
        &self,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        if p.is_none() || q.is_none() {
            return p.is_none() && q.is_none();
        }

        let mut node1 = p.as_ref().cloned();
        let mut node2 = q.as_ref().cloned();

        let mut stack1 = Vec::new();
        let mut stack2 = Vec::new();

        while (node1.is_some() && node2.is_some()) || (!stack1.is_empty() && !stack2.is_empty()) {
            while node1.is_some() && node2.is_some() {
                // let n1 = node1.as_ref().unwrap().clone();
                // let n2 = node2.as_ref().unwrap().clone();
                let n1 = node1.take().unwrap();
                let n2 = node2.take().unwrap();
                node1 = n1.borrow().left.clone();
                node2 = n2.borrow().left.clone();
                if n1.borrow().val != n2.borrow().val {
                    return false;
                }
                stack1.push(n1);
                stack2.push(n2);
            }
            if node1.is_some() || node2.is_some() {
                return false;
            }
            if let (Some(n1), Some(n2)) = (stack1.pop(), stack2.pop()) {
                node1 = n1.borrow().right.clone();
                node2 = n2.borrow().right.clone();
            } else {
                return false;
            }
        }
        node1.is_none() && node2.is_none() && stack1.is_empty() && stack2.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::common::tree_node::vec_to_bst;

    use super::*;

    #[test]
    fn test1() {
        let p = vec_to_bst(vec![Some(1), Some(2), Some(2)]);
        let q = vec_to_bst(vec![Some(1), Some(2), Some(2)]);
        let s = Solution;
        let result = s.is_same_tree(p, q);
        assert_eq!(result, true);
    }

    #[test]
    fn test2() {
        let p = vec_to_bst(vec![Some(1), Some(2)]);
        let q = vec_to_bst(vec![Some(1), None, Some(2)]);
        let s = Solution;
        let result = s.is_same_tree(p, q);
        assert_eq!(result, false);
    }

    #[test]
    fn test3() {
        let p = vec_to_bst(vec![Some(1), Some(2), Some(1)]);
        let q = vec_to_bst(vec![Some(1), Some(1), Some(2)]);
        let s = Solution;
        let result = s.is_same_tree(p, q);
        assert_eq!(result, false);
    }

    #[test]
    fn test4() {
        let p = vec_to_bst(vec![]);
        let q = vec_to_bst(vec![Some(0)]);
        let s = Solution;
        let result = s.is_same_tree(p, q);
        assert_eq!(result, false);
    }

    #[test]
    fn test5() {
        let p = vec_to_bst(vec![Some(1)]);
        let q = vec_to_bst(vec![Some(1), None, Some(2)]);
        let s = Solution;
        let result = s.is_same_tree(p, q);
        assert_eq!(result, false);
    }
}
