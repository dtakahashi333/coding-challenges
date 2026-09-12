# python/tests/dsa/binary_tree/test_kth_smallest.py

from unittest import TestCase

from dsa.binary_tree.kth_smallest import Solution
from dsa.binary_tree.tree_node import build_binary_tree


class TestKthSmallest(TestCase):
    def setUp(self):
        super().setUp()
        self.solution = Solution()

    def test_kth_smallest1(self):
        root = build_binary_tree([3, 1, 4, None, 2])
        k = 1
        self.assertEqual(self.solution.kthSmallest(root=root, k=k), 1)

    def test_kth_smallest2(self):
        root = build_binary_tree([5, 3, 6, 2, 4, None, None, 1])
        k = 3
        self.assertEqual(self.solution.kthSmallest(root=root, k=k), 3)
