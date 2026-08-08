# python/tests/dsa/linked_list/test_linked_list.py

from unittest import TestCase
from dsa.linked_list.linked_list import list_to_linked_list, linked_list_to_list


class TestLinkedList(TestCase):
    def test_linked_list1(self):
        lst = [1, 2, 3, 4, 5]
        head = list_to_linked_list(lst)
        self.assertListEqual(linked_list_to_list(head), lst)

    def test_linked_list2(self):
        lst = [1, 2]
        head = list_to_linked_list(lst)
        self.assertListEqual(linked_list_to_list(head), lst)
