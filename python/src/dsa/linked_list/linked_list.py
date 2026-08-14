# python/src/dsa/linked_list/linked_list.py


from typing import List, Optional


# Definition for singly-linked list.
class ListNode:
    def __init__(self, val=0, next=None):
        self.val = val
        self.next = next


def list_to_linked_list(lst: List[int]) -> Optional[ListNode]:
    length = len(lst)
    head = None
    for i in range(length - 1, -1, -1):
        if i == length - 1:
            head = ListNode(lst[i])
        else:
            head = ListNode(lst[i], head)

    return head


def linked_list_to_list(head: ListNode) -> List[int]:
    node = head
    lst = []
    while node:
        lst.append(node.val)
        node = node.next

    return lst


def list_to_cyclic_linked_list(lst: List[int], pos: int) -> Optional[ListNode]:
    head = list_to_linked_list(lst)
    if not head or pos == -1:
        return head
    node, connection = head, None
    index = 0
    while node.next:
        if index == pos:
            connection = node
        node = node.next
        index += 1
    node.next = connection
    return head
