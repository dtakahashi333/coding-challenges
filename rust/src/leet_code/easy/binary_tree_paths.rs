// rust/src/leet_code/easy/binary_tree_paths.rs

// 257. Binary Tree Paths
// https://leetcode.com/problems/binary-tree-paths/description/

use std::cell::RefCell;
use std::rc::Rc;

use crate::leet_code::common::tree_node::TreeNode;

#[derive(Debug)]
pub struct Solution;

impl Solution {
    pub fn binary_tree_paths(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<String> {
        let mut res = Vec::new();
        if root.is_some() {
            Self::binary_tree_paths_helper(&root, String::new(), &mut res);
        }
        return res;
    }

    fn binary_tree_paths_helper(
        node: &Option<Rc<RefCell<TreeNode>>>,
        mut path: String,
        res: &mut Vec<String>,
    ) {
        if let Some(n) = node {
            let n_ref = n.borrow();

            // let path = if path.is_empty() {
            //     format!("{}", n_ref.val)
            // } else {
            //     format!("{}->{}", path, n_ref.val)
            // };
            if !path.is_empty() {
                path.push_str("->");
            }
            path.push_str(&n_ref.val.to_string());

            // leaf node.
            if n_ref.left.is_none() && n_ref.right.is_none() {
                res.push(path);
            } else {
                Self::binary_tree_paths_helper(&n_ref.left, path.clone(), res);
                Self::binary_tree_paths_helper(&n_ref.right, path, res);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::common::tree_node::vec_to_bst;

    use super::*;

    #[test]
    fn test1() {
        let root = vec_to_bst(vec![Some(1), Some(2), Some(3), None, Some(5)]);
        let result = Solution::binary_tree_paths(root);
        assert_eq!(result, vec!["1->2->5", "1->3"]);
    }

    #[test]
    fn test2() {
        let root = vec_to_bst(vec![Some(1)]);
        let result = Solution::binary_tree_paths(root);
        assert_eq!(result, vec!["1"]);
    }
}
