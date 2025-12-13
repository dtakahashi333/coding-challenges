// rust/src/leet_code/easy/min_depth.rs

// 111. Minimum Depth of Binary Tree
// https://leetcode.com/problems/minimum-depth-of-binary-tree/description/

use crate::leet_code::common::tree_node::TreeNode;

use std::cell::RefCell;
use std::cmp;
use std::rc::Rc;

#[derive(Debug)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn min_depth(&self, root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let node = root;
        // if let Some(n) = node {
        //     let n_ref = n.borrow();
        //     if n_ref.left.is_none() && n_ref.right.is_none() {
        //         return 1;
        //     } else if n_ref.left.is_none() {
        //         return self.min_depth_helper(Some(Rc::clone(&n_ref.right))) + 1;
        //     } else if n_ref.right.is_none() {
        //         return self.min_depth_helper(Some(Rc::clone(&n_ref.left))) + 1;
        //     } else {
        //         return cmp::min(
        //             self.min_depth_helper(Some(Rc::clone(&n_ref.left))),
        //             self.min_depth_helper(Some(Rc::clone(&n_ref.right))),
        //         ) + 1;
        //     }
        // } else {
        //     return 0;
        // }
        match node {
            None => 0,
            Some(n) => {
                let n_ref = n.borrow();
                match (&n_ref.left, &n_ref.right) {
                    (None, None) => 1,
                    (Some(l), None) => self.min_depth(Some(Rc::clone(l))) + 1,
                    (None, Some(r)) => self.min_depth(Some(Rc::clone(r))) + 1,
                    (Some(l), Some(r)) => {
                        cmp::min(
                            self.min_depth(Some(Rc::clone(l))),
                            self.min_depth(Some(Rc::clone(r))),
                        ) + 1
                    }
                }
            }
        }
    }

    #[allow(dead_code)]
    pub fn min_depth2(&self, root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        use std::collections::VecDeque;

        if let Some(node) = root {
            let mut queue = VecDeque::new();
            queue.push_back((node, 1));
            while let Some((n, depth)) = queue.pop_front() {
                let n_ref = n.borrow();
                if n_ref.left.is_none() && n_ref.right.is_none() {
                    return depth;
                }
                if let Some(left) = &n_ref.left {
                    queue.push_back((Rc::clone(left), depth + 1));
                }
                if let Some(right) = &n_ref.right {
                    queue.push_back((Rc::clone(right), depth + 1));
                }
            }
        }
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::leet_code::common::tree_node::vec_to_bst;

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
        let s = Solution;
        let result = s.min_depth(root);
        assert_eq!(result, 2);
    }

    #[test]
    fn test2() {
        let root = vec_to_bst(vec![
            Some(2),
            None,
            Some(3),
            None,
            Some(4),
            None,
            Some(5),
            None,
            Some(6),
        ]);
        let s = Solution;
        let result = s.min_depth(root);
        assert_eq!(result, 5);
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
        let s = Solution;
        let result = s.min_depth2(root);
        assert_eq!(result, 2);
    }

    #[test]
    fn test4() {
        let root = vec_to_bst(vec![
            Some(2),
            None,
            Some(3),
            None,
            Some(4),
            None,
            Some(5),
            None,
            Some(6),
        ]);
        let s = Solution;
        let result = s.min_depth2(root);
        assert_eq!(result, 5);
    }
}
