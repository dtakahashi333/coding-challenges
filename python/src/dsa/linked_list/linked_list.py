# python/src/dsa/linked_list/linked_list.py


from typing import List, Optional


# Definition for singly-linked list.
class ListNode:
    def __init__(self, val=0, next=None):
        self.val = val
        self.next = next


def list_to_linked_list(lst: List[int]) -> ListNode:
    length = len(lst)
    node = None
    for i in range(length - 1, -1, -1):
        if i == length - 1:
            node = ListNode(lst[i])
        else:
            node = ListNode(lst[i], node)

    return node


def linked_list_to_list(head: ListNode) -> List[int]:
    node = head
    lst = []
    while node != None:
        lst.append(node.val)
        node = node.next

    return lst
