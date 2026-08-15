# python/src/dsa/linked_list/remove_nth_from_end.py

# 19. Remove Nth Node From End of List
# https://leetcode.com/problems/remove-nth-node-from-end-of-list/description/

from typing import Optional

from dsa.linked_list.linked_list import ListNode


class Solution:
    def removeNthFromEnd(self, head: Optional[ListNode], n: int) -> Optional[ListNode]:
        current = head
        count = 0
        while current:
            current = current.next
            count += 1

        previous, current = None, head
        for _ in range(count - n):
            previous, current = current, current.next

        if previous:
            previous.next = current.next
            current = None
            return head
        else:
            return None

    def removeNthFromEnd2(self, head: Optional[ListNode], n: int) -> Optional[ListNode]:
        tail = head
        for _ in range(n):
            if tail == None:
                raise ValueError()
            tail = tail.next

        previous, current = None, head
        while tail:
            previous, current, tail = current, current.next, tail.next

        if previous:
            previous.next = current.next
            current = None
            return head
        else:
            return None
