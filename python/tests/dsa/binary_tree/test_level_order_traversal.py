#!/usr/bin/env python3

# tests/dsa/binary_tree/test_level_order_traversal.py


from unittest import TestCase

from dsa.binary_tree.level_order_traversal import Solution
from dsa.binary_tree.tree_node import build_binary_tree


class TestLevelOrderTraversal(TestCase):
    def setUp(self):
        super().setUp()
        self.solution = Solution()

    def test_level_order_traversal1(self):
        root = build_binary_tree([3, 9, 20, None, None, 15, 7])
        res = self.solution.levelOrder(root)
        # Assertion
        self.assertListEqual(res, [[3], [9, 20], [15, 7]])

    def test_level_order_traversal2(self):
        root = build_binary_tree([1])
        res = self.solution.levelOrder(root)
        # Assertion
        self.assertListEqual(res, [[1]])

    def test_level_order_traversal3(self):
        root = build_binary_tree([])
        res = self.solution.levelOrder(root)
        # Assertion
        self.assertListEqual(res, [])
