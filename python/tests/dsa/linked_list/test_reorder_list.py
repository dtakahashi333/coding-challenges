# python/tests/dsa/linked_list/test_reorder_list.py

from unittest import TestCase
from dsa.linked_list.linked_list import linked_list_to_list, list_to_linked_list
from dsa.linked_list.reorder_list import Solution


class TestReorderList(TestCase):
    def setUp(self):
        self.solution = Solution()
        return super().setUp()

    def test_reorder_list1(self):
        head = list_to_linked_list([1, 2, 3, 4])
        self.solution.reorderList(head)
        self.assertListEqual(linked_list_to_list(head), [1, 4, 2, 3])
        head = list_to_linked_list([1, 2, 3, 4])
        self.solution.reorderList2(head)
        self.assertListEqual(linked_list_to_list(head), [1, 4, 2, 3])

    def test_reorder_list2(self):
        head = list_to_linked_list([1, 2, 3, 4, 5])
        self.solution.reorderList(head)
        self.assertListEqual(linked_list_to_list(head), [1, 5, 2, 4, 3])
        head = list_to_linked_list([1, 2, 3, 4, 5])
        self.solution.reorderList2(head)
        self.assertListEqual(linked_list_to_list(head), [1, 5, 2, 4, 3])

    def test_reorder_list3(self):
        head = list_to_linked_list([1])
        self.solution.reorderList(head)
        self.assertListEqual(linked_list_to_list(head), [1])
        head = list_to_linked_list([1])
        self.solution.reorderList2(head)
        self.assertListEqual(linked_list_to_list(head), [1])

    def test_reorder_list4(self):
        head = list_to_linked_list([1, 2])
        self.solution.reorderList(head)
        self.assertListEqual(linked_list_to_list(head), [1, 2])
        head = list_to_linked_list([1, 2])
        self.solution.reorderList2(head)
        self.assertListEqual(linked_list_to_list(head), [1, 2])

    def test_reorder_list5(self):
        head = list_to_linked_list([1, 2, 3])
        self.solution.reorderList(head)
        self.assertListEqual(linked_list_to_list(head), [1, 3, 2])
        head = list_to_linked_list([1, 2, 3])
        self.solution.reorderList2(head)
        self.assertListEqual(linked_list_to_list(head), [1, 3, 2])
