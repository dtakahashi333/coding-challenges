#!/bin/env python3

# tests/dsa/binary_tree/test_maximum_depth.py

from unittest import TestCase
from dsa.binary_tree.tree_node import build_binary_tree
from dsa.binary_tree.maximum_depth import Solution


class TestMaximumDepth(TestCase):

    def test_max_depth1(self):
        root = build_binary_tree([])
        s = Solution()
        res = s.maxDepth(root)
        # Assertion
        self.assertEqual(res, 0)

    def test_max_depth2(self):
        root = build_binary_tree([1])
        s = Solution()
        res = s.maxDepth(root)
        # Assertion
        self.assertEqual(res, 1)

    def test_max_depth3(self):
        root = build_binary_tree([3, 9, 20, None, None, 15, 7])
        s = Solution()
        res = s.maxDepth(root)
        # Assertion
        self.assertEqual(res, 3)

    def test_max_depth4(self):
        root = build_binary_tree([1, None, 2])
        s = Solution()
        res = s.maxDepth(root)
        # Assertion
        self.assertEqual(res, 2)
