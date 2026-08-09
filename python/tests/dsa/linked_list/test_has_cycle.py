# tests/dsa/linked_list/has_cycle.py

from unittest import TestCase
from dsa.linked_list.has_cycle import Solution
from dsa.linked_list.linked_list import list_to_cyclic_linked_list


class TestHasCycle(TestCase):
    def setUp(self):
        self.solution = Solution()
        return super().setUp()

    def test_has_cycle1(self):
        head = list_to_cyclic_linked_list([3, 2, 0, -4], 1)
        self.assertEqual(self.solution.hasCycle(head), True)
        self.assertEqual(self.solution.hasCycle2(head), True)

    def test_has_cycle2d(self):
        head = list_to_cyclic_linked_list([1, 2], 0)
        self.assertEqual(self.solution.hasCycle(head), True)
        self.assertEqual(self.solution.hasCycle2(head), True)

    def test_has_cycle2d(self):
        head = list_to_cyclic_linked_list([1], -1)
        self.assertEqual(self.solution.hasCycle(head), False)
        self.assertEqual(self.solution.hasCycle2(head), False)
