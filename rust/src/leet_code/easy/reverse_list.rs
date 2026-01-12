// rust/src/leet_code/easy/reverse_list.rs

// 206. Reverse Linked List
// https://leetcode.com/problems/reverse-linked-list/description/

use crate::leet_code::common::list_node::ListNode;

#[derive(Debug)]
pub struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none() {
            return None;
        }

        let mut head = head;
        let mut cur = head.take().unwrap();
        head = cur.next.take();
        let mut reversed = cur;
        while head.is_some() {
            cur = head.take().unwrap();
            head = cur.next.take();
            cur.next = Some(reversed);
            reversed = cur;
        }
        Some(reversed)
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::common::list_node::{list_to_vec, vec_to_list};

    use super::*;

    #[test]
    fn test1() {
        let head = vec_to_list(vec![1, 2, 3, 4, 5]);
        let result = Solution::reverse_list(head);
        assert_eq!(list_to_vec(result), vec![5, 4, 3, 2, 1]);
    }

    #[test]
    fn test2() {
        let head = vec_to_list(vec![1, 2]);
        let result = Solution::reverse_list(head);
        assert_eq!(list_to_vec(result), vec![2, 1]);
    }

    #[test]
    fn test3() {
        let head = vec_to_list(vec![]);
        let result = Solution::reverse_list(head);
        assert_eq!(list_to_vec(result), vec![]);
    }
}
