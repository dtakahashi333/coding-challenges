# python/src/dsa/linked_list/reorder_list.py

# 143. Reorder List
# https://leetcode.com/problems/reorder-list/description/

from typing import Optional

from dsa.linked_list.linked_list import ListNode


class Solution:
    def reorderList(self, head: Optional[ListNode]) -> None:
        """
        Do not return anything, modify head in-place instead.
        """
        lst = []
        current = head
        while current:
            # Look for the tail
            lst.append(current)
            current = current.next
            lst[-1].next = None

        current = lst[0]
        for i in range(1, len(lst), 1):
            if i % 2 == 0:
                current.next = lst[i // 2]
            else:
                current.next = lst[-((i + 1) // 2)]
            current = current.next

    def reorderList2(self, head: Optional[ListNode]) -> None:
        """
        Do not return anything, modify head in-place instead.
        """
        # Find the middle
        fast, slow = head, head
        previous = slow
        while fast and fast.next:
            fast = fast.next.next
            previous = slow
            slow = slow.next

        if slow == fast or not slow.next:
            return

        previous.next = None

        # Now slow points to the middle node.
        # Reverse the second half.
        previous, current, next = None, slow, None
        while current:
            next = current.next
            current.next = previous
            previous = current
            current = next

        # Merge first and seconds halves.
        sentinel, first, second = ListNode(), head, previous
        while first and second:
            sentinel.next = first
            first = first.next
            sentinel = sentinel.next
            sentinel.next = second
            second = second.next
            sentinel = sentinel.next

        if first:
            sentinel.next = first
        elif second:
            sentinel.next = second
