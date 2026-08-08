# python/tests/dsa/linked_list/merge_two_lists.py

from unittest import TestCase
from dsa.linked_list.linked_list import linked_list_to_list, list_to_linked_list
from dsa.linked_list.merge_two_lists import Solution


class TestMergeTwoLists(TestCase):
    def setUp(self):
        self.solution = Solution()
        return super().setUp()

    def test_merge_two_lists1(self):
        list1 = list_to_linked_list([1, 2, 4])
        list2 = list_to_linked_list([1, 3, 4])
        self.assertListEqual(
            linked_list_to_list(self.solution.mergeTwoLists(list1, list2)),
            [1, 1, 2, 3, 4, 4],
        )
        list1 = list_to_linked_list([1, 2, 4])
        list2 = list_to_linked_list([1, 3, 4])
        self.assertListEqual(
            linked_list_to_list(self.solution.mergeTwoLists2(list1, list2)),
            [1, 1, 2, 3, 4, 4],
        )
        list1 = list_to_linked_list([1, 2, 4])
        list2 = list_to_linked_list([1, 3, 4])
        self.assertListEqual(
            linked_list_to_list(self.solution.mergeTwoLists3(list1, list2)),
            [1, 1, 2, 3, 4, 4],
        )

    def test_merge_two_lists2(self):
        list1 = list_to_linked_list([])
        list2 = list_to_linked_list([])
        self.assertListEqual(
            linked_list_to_list(self.solution.mergeTwoLists(list1, list2)),
            [],
        )
        list1 = list_to_linked_list([])
        list2 = list_to_linked_list([])
        self.assertListEqual(
            linked_list_to_list(self.solution.mergeTwoLists2(list1, list2)),
            [],
        )
        list1 = list_to_linked_list([])
        list2 = list_to_linked_list([])
        self.assertListEqual(
            linked_list_to_list(self.solution.mergeTwoLists3(list1, list2)),
            [],
        )

    def test_merge_two_lists3(self):
        list1 = list_to_linked_list([])
        list2 = list_to_linked_list([0])
        self.assertListEqual(
            linked_list_to_list(self.solution.mergeTwoLists(list1, list2)),
            [0],
        )
        list1 = list_to_linked_list([])
        list2 = list_to_linked_list([0])
        self.assertListEqual(
            linked_list_to_list(self.solution.mergeTwoLists2(list1, list2)),
            [0],
        )
        list1 = list_to_linked_list([])
        list2 = list_to_linked_list([0])
        self.assertListEqual(
            linked_list_to_list(self.solution.mergeTwoLists3(list1, list2)),
            [0],
        )

    def test_merge_two_lists4(self):
        list1 = list_to_linked_list([1])
        list2 = list_to_linked_list([2])
        self.assertListEqual(
            linked_list_to_list(self.solution.mergeTwoLists(list1, list2)),
            [1, 2],
        )
        list1 = list_to_linked_list([1])
        list2 = list_to_linked_list([2])
        self.assertListEqual(
            linked_list_to_list(self.solution.mergeTwoLists2(list1, list2)),
            [1, 2],
        )
        list1 = list_to_linked_list([1])
        list2 = list_to_linked_list([2])
        self.assertListEqual(
            linked_list_to_list(self.solution.mergeTwoLists3(list1, list2)),
            [1, 2],
        )
