// rust/src/leet_code/easy/delete_duplicates.rs

// 83. Remove Duplicates from Sorted List
// https://leetcode.com/problems/remove-duplicates-from-sorted-list/description/

use crate::leet_code::common::list_node::*;

#[derive(Debug)]
pub struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn delete_duplicates(&self, head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none() {
            return head;
        }

        let mut list = head;
        let mut dummy = Box::new(ListNode::new(0));
        let mut tail = &mut dummy;

        let mut prev_val = None;
        while list.is_some() {
            let val = list.as_ref().unwrap().val;
            let mut node = list.take().unwrap();
            list = node.next.take();
            if prev_val.is_none() || val != prev_val.unwrap() {
                tail.next = Some(node);
                // Move tail to the last node
                tail = tail.next.as_mut().unwrap();

                prev_val = Some(val);
            }
        }
        dummy.next
    }

    // Clean & Idiomatic Rust Version
    pub fn delete_duplicates2(&self, head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head = head;
        let mut dummy = Box::new(ListNode::new(0));
        let mut tail = &mut dummy;

        let mut prev_val: Option<i32> = None;

        while let Some(mut node) = head {
            head = node.next.take();
            let val = node.val;

            // Add node only if not equal to previous value
            if prev_val.map_or(true, |pv| pv != val) {
                prev_val = Some(val);
                tail.next = Some(node);
                tail = tail.next.as_mut().unwrap();
            }
        }

        dummy.next
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::common::list_node::{list_to_vec, vec_to_list};

    use super::*;

    #[test]
    fn test1() {
        let head = vec_to_list(vec![1, 1, 2]);
        let s = Solution;
        let result = s.delete_duplicates(head);
        assert_eq!(list_to_vec(result), vec![1, 2]);
    }

    #[test]
    fn test2() {
        let head = vec_to_list(vec![1, 1, 2, 3, 3]);
        let s = Solution;
        let result = s.delete_duplicates(head);
        assert_eq!(list_to_vec(result), vec![1, 2, 3]);
    }
}
