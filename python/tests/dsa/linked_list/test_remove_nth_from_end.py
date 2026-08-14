# python/tests/dsa/linked_list/test_remove_nth_from_end.py

from unittest import TestCase
from dsa.linked_list.linked_list import linked_list_to_list, list_to_linked_list
from dsa.linked_list.remove_nth_from_end import Solution


class TestRemoveNthFromEnd(TestCase):
    def setUp(self):
        self.solution = Solution()
        return super().setUp()

    def test_remove_nth_from_end1(self):
        head = list_to_linked_list([1, 2, 3, 4, 5])
        n = 2
        self.assertListEqual(
            linked_list_to_list(self.solution.removeNthFromEnd(head, n)), [1, 2, 3, 5]
        )
        head = list_to_linked_list([1, 2, 3, 4, 5])
        self.assertListEqual(
            linked_list_to_list(self.solution.removeNthFromEnd2(head, n)), [1, 2, 3, 5]
        )

    def test_remove_nth_from_end2(self):
        head = list_to_linked_list([1])
        n = 1
        self.assertListEqual(
            linked_list_to_list(self.solution.removeNthFromEnd(head, n)), []
        )
        head = list_to_linked_list([1])
        self.assertListEqual(
            linked_list_to_list(self.solution.removeNthFromEnd2(head, n)), []
        )

    def test_remove_nth_from_end3(self):
        head = list_to_linked_list([1, 2])
        n = 1
        self.assertListEqual(
            linked_list_to_list(self.solution.removeNthFromEnd(head, n)), [1]
        )
        head = list_to_linked_list([1, 2])
        self.assertListEqual(
            linked_list_to_list(self.solution.removeNthFromEnd2(head, n)), [1]
        )
