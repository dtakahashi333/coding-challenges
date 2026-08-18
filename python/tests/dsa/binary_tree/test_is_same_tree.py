# python/tests/dsa/binary_tree/test_is_same_tree.py

from unittest import TestCase

from dsa.binary_tree.is_same_tree import Solution
from dsa.binary_tree.tree_node import build_binary_tree


class TestIsSameTree(TestCase):
    def setUp(self):
        self.solution = Solution()
        return super().setUp()

    def test_is_same_tree1(self):
        p = build_binary_tree([1, 2, 3])
        q = build_binary_tree([1, 2, 3])
        self.assertTrue(self.solution.isSameTree(p, q))

    def test_is_same_tree2(self):
        p = build_binary_tree([1, 2])
        q = build_binary_tree([1, None, 2])
        self.assertFalse(self.solution.isSameTree(p, q))

    def test_is_same_tree3(self):
        p = build_binary_tree([1, 2, 1])
        q = build_binary_tree([1, 1, 2])
        self.assertFalse(self.solution.isSameTree(p, q))

    def test_is_same_tree4(self):
        p = build_binary_tree([])
        q = build_binary_tree([1])
        self.assertFalse(self.solution.isSameTree(p, q))
