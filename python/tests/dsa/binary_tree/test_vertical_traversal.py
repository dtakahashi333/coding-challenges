#!/bin/env python3

# python/tests/dsa/binary_tree/test_vertical_traversal.py

from unittest import TestCase
from dsa.binary_tree.tree_node import build_binary_tree
from dsa.binary_tree.vertical_traversal import Solution


class TestVerticalTraversal(TestCase):
    def test_vertical_traversal1(self):
        root = build_binary_tree([3, 9, 20, None, None, 15, 7])
        s = Solution()
        res = s.verticalTraversal(root)
        self.assertListEqual(res, [[9], [3, 15], [20], [7]])

    def test_vertical_traversal2(self):
        root = build_binary_tree([1, 2, 3, 4, 5, 6, 7])
        s = Solution()
        res = s.verticalTraversal(root)
        self.assertListEqual(res, [[4], [2], [1, 5, 6], [3], [7]])

    def test_vertical_traversal3(self):
        root = build_binary_tree([1, 2, 3, 4, 6, 5, 7])
        s = Solution()
        res = s.verticalTraversal(root)
        self.assertListEqual(res, [[4], [2], [1, 5, 6], [3], [7]])
