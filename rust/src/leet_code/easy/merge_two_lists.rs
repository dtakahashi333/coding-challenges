// rust/src/leet_code/easy/merge_two_lists.rs

// 21. Merge Two Sorted Lists
// https://leetcode.com/problems/merge-two-sorted-lists/

use crate::leet_code::common::list_node::ListNode;

#[derive(Debug)]
pub struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn merge_two_lists(
        &self,
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        // let mut list1 = match (list1.as_ref(), list2.as_ref()) {// list1.as_ref() converts Option<Box<ListNode>> into Option<&ListNode>.
        //     (Some(node1), Some(node2)) => {
        //         if node1.val < node2.val {
        //             Some(Box::new((*node1).clone()))
        //         } else {
        //             Some(Box::new((*node2).clone()))
        //         }
        //     }
        //     (Some(node1), None) => Some(Box::new((*node1).clone())),
        //     (None, Some(node2)) => Some(Box::new((*node2).clone())),
        //     (None, None) => None,
        // };
        let mut list1 = list1;
        let mut list2 = list2;

        let mut dummy = Box::new(ListNode::new(0));
        let mut tail = &mut dummy;

        while list1.is_some() && list2.is_some() {
            // Safe to unwrap because we just checked is_some
            let v1 = list1.as_ref().unwrap().val;
            let v2 = list2.as_ref().unwrap().val;

            if v1 <= v2 {
                // Move list1 node into the merged list
                let mut node = list1.take().unwrap();
                list1 = node.next.take();
                tail.next = Some(node);
            } else {
                // Move list2 node into the merged list
                let mut node = list2.take().unwrap();
                list2 = node.next.take();
                tail.next = Some(node);
            }

            // Move tail to the last node
            tail = tail.next.as_mut().unwrap();
        }

        // Append the remaining part
        tail.next = if list1.is_some() { list1 } else { list2 };

        dummy.next
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::common::list_node::{list_to_vec, vec_to_list};

    use super::*;

    #[test]
    fn test1() {
        let list1 = vec_to_list(vec![1, 2, 4]);
        let list2 = vec_to_list(vec![1, 3, 4]);
        let s = Solution;
        let merged = s.merge_two_lists(list1, list2);
        assert_eq!(list_to_vec(merged), vec![1, 1, 2, 3, 4, 4]);
    }

    #[test]
    fn test2() {
        let list1 = vec_to_list(vec![]);
        let list2 = vec_to_list(vec![]);
        let s = Solution;
        let merged = s.merge_two_lists(list1, list2);
        assert_eq!(list_to_vec(merged), vec![]);
    }

    #[test]
    fn test3() {
        let list1 = vec_to_list(vec![]);
        let list2 = vec_to_list(vec![0]);
        let s = Solution;
        let merged = s.merge_two_lists(list1, list2);
        assert_eq!(list_to_vec(merged), vec![0]);
    }
}
