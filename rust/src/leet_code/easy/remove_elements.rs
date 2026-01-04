// rust/src/leet_code/easy/remove_elements.rs

// 203. Remove Linked List Elements
// https://leetcode.com/problems/remove-linked-list-elements/description/

use crate::leet_code::common::list_node::ListNode;

#[derive(Debug)]
pub struct Solution;

impl Solution {
    pub fn remove_elements(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
        let mut dummy = ListNode::new(0);
        let mut tail = &mut dummy;
        let mut cur = head;
        while cur.is_some() {
            let cur_val = cur.as_ref().unwrap().val;
            let mut node = cur.take().unwrap();
            cur = node.next.take();
            if cur_val != val {
                tail.next = Some(node);
                // Move tail to the last node
                tail = tail.next.as_mut().unwrap();
            }
        }
        dummy.next
    }

    pub fn remove_elements2(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
        let mut head = head;
        let mut cur = &mut head;

        while let Some(mut node) = cur.take() {
            if node.val == val {
                // Skip this node
                *cur = node.next.take();
            } else {
                // Keep the node
                *cur = Some(node);
                cur = &mut cur.as_mut().unwrap().next;
            }
        }

        head
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::common::list_node::{list_to_vec, vec_to_list};

    use super::*;

    #[test]
    fn test1() {
        let head = vec_to_list(vec![1, 2, 6, 3, 4, 5, 6]);
        let val = 6;
        let result = Solution::remove_elements(head, val);
        assert_eq!(list_to_vec(result), vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test2() {
        let head = vec_to_list(vec![]);
        let val = 1;
        let result = Solution::remove_elements(head, val);
        assert_eq!(list_to_vec(result), vec![]);
    }

    #[test]
    fn test3() {
        let head = vec_to_list(vec![7, 7, 7, 7]);
        let val = 7;
        let result = Solution::remove_elements(head, val);
        assert_eq!(list_to_vec(result), vec![]);
    }

    #[test]
    fn test4() {
        let head = vec_to_list(vec![1, 2, 6, 3, 4, 5, 6]);
        let val = 6;
        let result = Solution::remove_elements2(head, val);
        assert_eq!(list_to_vec(result), vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test5() {
        let head = vec_to_list(vec![]);
        let val = 1;
        let result = Solution::remove_elements2(head, val);
        assert_eq!(list_to_vec(result), vec![]);
    }

    #[test]
    fn test6() {
        let head = vec_to_list(vec![7, 7, 7, 7]);
        let val = 7;
        let result = Solution::remove_elements2(head, val);
        assert_eq!(list_to_vec(result), vec![]);
    }
}
