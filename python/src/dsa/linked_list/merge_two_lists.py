# python/src/dsa/linked_list/merge_two_lists.py

# 21. Merge Two Sorted Lists
# https://leetcode.com/problems/merge-two-sorted-lists/description/

from typing import Optional

from dsa.linked_list.linked_list import (
    ListNode,
    linked_list_to_list,
    list_to_linked_list,
)


class Solution:
    def mergeTwoLists(
        self, list1: Optional[ListNode], list2: Optional[ListNode]
    ) -> Optional[ListNode]:
        head = ListNode()
        current = head
        while list1 and list2:
            if list1.val < list2.val:
                current.next = ListNode(list1.val)
                list1 = list1.next
            else:
                current.next = ListNode(list2.val)
                list2 = list2.next

            current = current.next

        if list1:
            current.next = list1
        elif list2:
            current.next = list2

        return head.next

    def mergeTwoLists2(
        self, list1: Optional[ListNode], list2: Optional[ListNode]
    ) -> Optional[ListNode]:
        head = list1
        while list1 and list2:
            if list1.val < list2.val:
                list1 = list1.next
            else:
                # swap values
                list1_val = list1.val
                list1.val = list2.val
                list2.val = list1_val
                # insert list2 node to list1
                list2_next = list2.next
                list2.next = list1.next
                list1.next = list2
                list1 = list2
                list2 = list2_next

        if not head:
            return list2

        if list2:
            previous, current = head, head
            while current:
                previous = current
                current = current.next

            previous.next = list2

        return head

    def mergeTwoLists3(
        self, list1: Optional[ListNode], list2: Optional[ListNode]
    ) -> Optional[ListNode]:
        head = ListNode()
        current = head
        while list1 and list2:
            if list1.val < list2.val:
                current.next = list1
                list1 = list1.next
            else:
                current.next = list2
                list2 = list2.next

            current = current.next

        if list1:
            current.next = list1
        elif list2:
            current.next = list2

        return head.next


def main():
    list1 = list_to_linked_list([1, 2, 4])
    list2 = list_to_linked_list([1, 3, 4])
    print(linked_list_to_list(Solution().mergeTwoLists2(list1, list2)))


if __name__ == "__main__":
    main()
