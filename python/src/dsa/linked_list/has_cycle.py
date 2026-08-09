# python/src/dsa/linked_list/has_cycle.py

# 141. Linked List Cycle
# https://leetcode.com/problems/linked-list-cycle/description/

from typing import Optional

from dsa.linked_list.linked_list import ListNode


class Solution:
    def hasCycle(self, head: Optional[ListNode]) -> bool:
        visited = set()
        node = head
        while node:
            if node in visited:
                return True
            visited.add(node)
            node = node.next

        return False

    def hasCycle2(self, head: Optional[ListNode]) -> bool:
        fast, slow = head, head
        while fast and fast.next:
            fast, slow = fast.next.next, slow.next
            if slow == fast:
                return True

        return False
