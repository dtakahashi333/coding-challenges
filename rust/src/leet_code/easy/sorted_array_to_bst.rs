// rust/src/leet_code/easy/sorted_array_to_bst.rs

// 108. Convert Sorted Array to Binary Search Tree
// https://leetcode.com/problems/convert-sorted-array-to-binary-search-tree/description/

use crate::leet_code::common::tree_node::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
pub struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn sorted_array_to_bst(&self, nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        if nums.is_empty() {
            return None;
        }
        self.sorted_array_to_bst_helper(&nums, 0, (nums.len() - 1) as isize)
    }

    fn sorted_array_to_bst_helper(
        &self,
        nums: &Vec<i32>,
        start: isize,
        end: isize,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if start > end {
            return None;
        }
        let mid = start + (end - start) / 2;
        let node = Rc::new(RefCell::new(TreeNode::new(nums[mid as usize])));
        node.borrow_mut().left = self.sorted_array_to_bst_helper(nums, start, mid - 1);
        node.borrow_mut().right = self.sorted_array_to_bst_helper(nums, mid + 1, end);
        Some(node)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::leet_code::common::tree_node::bst_to_vec;

    #[test]
    fn test1() {
        let nums = vec![-10, -3, 0, 5, 9];
        let s = Solution;
        let root = s.sorted_array_to_bst(nums);
        let result = bst_to_vec(root);
        assert_eq!(result, vec![Some(0), Some(-10), Some(5), None, Some(-3), None, Some(9)]);
    }

    #[test]
    fn test2() {
        let nums = vec![1,3];
        let s = Solution;
        let root = s.sorted_array_to_bst(nums);
        let result = bst_to_vec(root);
        assert_eq!(result, vec![Some(1), None, Some(3)]);
    }
}
