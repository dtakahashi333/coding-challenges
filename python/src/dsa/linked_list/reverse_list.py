#!/usr/bin/env python3

# python/src/dsa/linked_list/reverse_list.py

# 206. Reverse Linked List
# https://leetcode.com/problems/reverse-linked-list/description/

from typing import Optional

from dsa.linked_list.linked_list import ListNode


class Solution:
    def reverseList(self, head: Optional[ListNode]) -> Optional[ListNode]:
        node, reverse_list = head, None
        while node != None:
            reverse_list = ListNode(node.val, reverse_list)
            node = node.next

        return reverse_list
    
    def reverseList2(self, head: Optional[ListNode]) -> Optional[ListNode]:
        if head is None:
            return None
        current_node = head.next
        head.next = None
        while current_node != None:
            next_node = current_node.next
            current_node.next = head
            head = current_node
            current_node = next_node

        return head


"""
h
|
p    c    n
|    |    |
1 -> 2 -> 3 -> 4 -> None 
"""