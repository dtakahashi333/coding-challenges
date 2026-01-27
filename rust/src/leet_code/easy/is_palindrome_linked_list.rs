// rust/src/leet_code/easy/is_palindrome_linked_list.rs

// 234. Palindrome Linked List
// https://leetcode.com/problems/palindrome-linked-list/description/

use crate::leet_code::common::list_node::ListNode;

#[derive(Debug)]
pub struct Solution;

impl Solution {
    pub fn is_palindrome_linked_list(mut head: Option<Box<ListNode>>) -> bool {
        if head.is_none() {
            return true;
        }

        // 1. Find middle using mutable references
        let steps = Self::steps_to_middle(&head);

        // 2. Re-walk to get a mutable link
        let mut slow_mut = &mut head;
        // steps + 1 to avoid second_half includes the middle of the list with odd number of elements.
        for _ in 0..steps + 1 {
            slow_mut = &mut slow_mut.as_mut().unwrap().next;
        }

        // 3. Reverse second half (ownership-based)
        let second_half = slow_mut.take();
        let mut right = Self::reverse_in_place(second_half);
        let mut left = &head;

        // 4. Compare halves
        while let Some(r) = right {
            if let Some(l) = left {
                if l.val != r.val {
                    return false;
                }
                left = &l.next;
            } else {
                return true;
            }
            right = r.next;
        }

        true

        // let mut slow = head.as_deref();
        // let mut fast = head.as_deref();

        // while let Some(f) = fast {
        //     fast = f.next.as_deref();
        //     if let Some(f2) = fast {
        //         fast = f2.next.as_deref();
        //         slow = slow.and_then(|s| s.next.as_deref());
        //     } else {
        //         break;
        //     }
        // }

        // // Now slow points to the middle point of the list.
    }

    fn steps_to_middle(head: &Option<Box<ListNode>>) -> usize {
        // 1. Find middle using mutable references
        let mut slow = head;
        let mut fast = head;
        let mut steps = 0;

        while let Some(f) = fast {
            if let Some(f1) = f.next.as_ref() {
                if f1.next.is_some() {
                    fast = &f1.next; // safe: reference to existing node, no clone
                    slow = &slow.as_ref().unwrap().next;
                    steps += 1;
                } else {
                    break;
                }
            } else {
                break;
            }
        }

        steps
    }

    fn reverse_in_place(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        // prev = null
        // cur = head

        // while cur:
        //     next = cur.next
        //     cur.next = prev
        //     prev = cur
        //     cur = next
        let mut prev = None;
        let mut cur = head;
        while let Some(mut node) = cur {
            let next = node.next.take(); // take ownership of cur.next
            node.next = prev; // reverse pointer
            prev = Some(node); // move cur into prev
            cur = next; // advance
        }
        prev
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::common::list_node::vec_to_list;

    use super::*;

    #[test]
    fn test1() {
        let head = vec_to_list(vec![1, 2, 2, 1]);
        let result = Solution::is_palindrome_linked_list(head);
        assert_eq!(result, true);
    }

    #[test]
    fn test2() {
        let head = vec_to_list(vec![1, 2]);
        let result = Solution::is_palindrome_linked_list(head);
        assert_eq!(result, false);
    }

    #[test]
    fn test3() {
        let head = vec_to_list(vec![1]);
        let result = Solution::is_palindrome_linked_list(head);
        assert_eq!(result, true);
    }

    #[test]
    fn test4() {
        let head = vec_to_list(vec![
            1, 2, 2, 1, 5, 7, 5, 3, 8, 9, 0, 6, 5, 9, 6, 9, 2, 1, 3, 0, 8, 8, 4, 8, 9, 0, 3, 5, 6,
            8, 9, 0, 1, 3, 2, 1, 0, 9, 8, 6, 5, 3, 0, 9, 8, 4, 8, 8, 0, 3, 1, 2, 9, 6, 9, 5, 6, 0,
            9, 8, 3, 5, 7, 5, 1, 2, 2, 1,
        ]);
        let result = Solution::is_palindrome_linked_list(head);
        assert_eq!(result, false);
    }
}
