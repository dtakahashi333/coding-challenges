#!/bin/env python3

# tests/dsa/binary_tree/test_zigzag_level_order_traversal.py

from unittest import TestCase
from dsa.binary_tree.tree_node import build_binary_tree
from dsa.binary_tree.zigzag_level_order_traversal import Solution


class TestZigzagLevelOrderTraversal(TestCase):

    def test_zigzag_level_order_traversal1(self):
        root = build_binary_tree([3, 9, 20, None, None, 15, 7])
        s = Solution()
        res = s.zigzagLevelOrder(root)
        self.assertListEqual(res, [[3], [20, 9], [15, 7]])

    def test_zigzag_level_order_traversal2(self):
        root = build_binary_tree([1])
        s = Solution()
        res = s.zigzagLevelOrder(root)
        self.assertListEqual(res, [[1]])

    def test_zigzag_level_order_traversal3(self):
        root = build_binary_tree([])
        s = Solution()
        res = s.zigzagLevelOrder(root)
        self.assertListEqual(res, [])

    def test_zigzag_level_order_traversal4(self):
        root = build_binary_tree([1, 2, 3, 4, None, None, 5])
        s = Solution()
        res = s.zigzagLevelOrder(root)
        self.assertListEqual(res, [[1], [3, 2], [4, 5]])
