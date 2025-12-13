// rust/src/leet_code/easy/has_path_sum.rs

// 112. Path Sum
// https://leetcode.com/problems/path-sum/description/

use crate::leet_code::common::tree_node::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
pub struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn has_path_sum(&self, root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
        let mut stack: Vec<(Rc<RefCell<TreeNode>>, i32)> = Vec::new();

        // while node.is_some() || !stack.is_empty() {
        //     while let Some(n) = node {
        //         let n_ref = n.borrow();
        //         let cumulative_sum = if let Some(last) = stack.last() {
        //             last.1
        //         } else {
        //             0
        //         } + n_ref.val;
        //         stack.push((Rc::clone(&n), cumulative_sum));
        //         node = n_ref.left.clone();
        //     }
        //     if let Some((n, sum)) = stack.pop() {
        //         let n_ref = n.borrow();
        //         if n_ref.right.is_none() {
        //             if n_ref.left.is_none() && sum == target_sum {
        //                 return true;
        //             }
        //         } else {
        //             // n_ref.right.is_some()
        //             let right = n_ref.right.as_ref().cloned().unwrap();
        //             let cumulative_sum = sum + right.borrow().val;
        //             node = right.borrow().left.as_ref().cloned();
        //             stack.push((right, cumulative_sum));
        //         }
        //     }
        // }
        // false

        if let Some(n) = root {
            stack.push((Rc::clone(&n), n.borrow().val));

            while !stack.is_empty() {
                if let Some((n, sum)) = stack.pop() {
                    let n_ref = n.borrow();
                    if n_ref.left.is_none() && n_ref.right.is_none() {
                        if sum == target_sum {
                            return true;
                        }
                    }
                    if let Some(left) = &n_ref.left {
                        let cumulative_sum = left.borrow().val + sum;
                        stack.push((Rc::clone(left), cumulative_sum));
                    }
                    if let Some(right) = &n_ref.right {
                        let cumulative_sum = right.borrow().val + sum;
                        stack.push((Rc::clone(right), cumulative_sum));
                    }
                }
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::leet_code::common::tree_node::vec_to_bst;

    #[test]
    fn test1() {
        let root = vec_to_bst(vec![
            Some(5),
            Some(4),
            Some(8),
            Some(11),
            None,
            Some(13),
            Some(4),
            Some(7),
            Some(2),
            None,
            None,
            None,
            Some(1),
        ]);
        let target_sum = 22;
        let s = Solution;
        let result = s.has_path_sum(root, target_sum);
        assert_eq!(result, true);
    }

    #[test]
    fn test2() {
        let root = vec_to_bst(vec![Some(1), Some(2), Some(3)]);
        let target_sum = 5;
        let s = Solution;
        let result = s.has_path_sum(root, target_sum);
        assert_eq!(result, false);
    }

    #[test]
    fn test3() {
        let root = vec_to_bst(vec![]);
        let target_sum = 0;
        let s = Solution;
        let result = s.has_path_sum(root, target_sum);
        assert_eq!(result, false);
    }

    #[test]
    fn test4() {
        let root = vec_to_bst(vec![Some(1), Some(2)]);
        let target_sum = 1;
        let s = Solution;
        let result = s.has_path_sum(root, target_sum);
        assert_eq!(result, false);
    }

    #[test]
    fn test5() {
        let root = vec_to_bst(vec![
            Some(1),
            Some(-2),
            Some(-3),
            Some(1),
            Some(3),
            Some(-2),
            None,
            Some(-1),
        ]);
        let target_sum = 3;
        let s = Solution;
        let result = s.has_path_sum(root, target_sum);
        assert_eq!(result, false);
    }

    #[test]
    fn test6() {
        let root = vec_to_bst(vec![
            Some(1),
            Some(-2),
            Some(-3),
            Some(1),
            Some(3),
            Some(-2),
            None,
            Some(-1),
        ]);
        let target_sum = -4;
        let s = Solution;
        let result = s.has_path_sum(root, target_sum);
        assert_eq!(result, true);
    }
}
