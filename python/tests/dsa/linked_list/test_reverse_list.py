# python/tests/dsa/linked_list/test_reverse_list.py

from unittest import TestCase

from dsa.linked_list.linked_list import linked_list_to_list, list_to_linked_list
from dsa.linked_list.reverse_list import Solution


class TestReverseList(TestCase):
    def setUp(self):
        self.solution = Solution()
        return super().setUp()

    def test_reverse_list1(self):
        head = list_to_linked_list([1, 2, 3, 4, 5])
        self.assertListEqual(
            linked_list_to_list(self.solution.reverseList(head)), [5, 4, 3, 2, 1]
        )
        self.assertListEqual(
            linked_list_to_list(self.solution.reverseList2(head)), [5, 4, 3, 2, 1]
        )

    def test_reverse_list2(self):
        head = list_to_linked_list([1, 2])
        self.assertListEqual(
            linked_list_to_list(self.solution.reverseList(head)), [2, 1]
        )
        self.assertListEqual(
            linked_list_to_list(self.solution.reverseList2(head)), [2, 1]
        )
