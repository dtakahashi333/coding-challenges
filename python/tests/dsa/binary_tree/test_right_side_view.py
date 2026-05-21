#!/bin/env python3

# tests/dsa/binary_tree/test_right_side_view.py

from unittest import TestCase

from dsa.binary_tree.right_side_view import Solution
from dsa.binary_tree.tree_node import build_binary_tree


class TestRightSideView(TestCase):

    def test_right_side_view1(self):
        root = build_binary_tree([1, 2, 3, None, 5, None, 4])
        s = Solution()
        res = s.rightSideView(root)
        self.assertListEqual(res, [1, 3, 4])

    def test_right_side_view2(self):
        root = build_binary_tree([1, 2, 3, 4, None, None, None, 5])
        s = Solution()
        res = s.rightSideView(root)
        self.assertListEqual(res, [1, 3, 4, 5])

    def test_right_side_view3(self):
        root = build_binary_tree([1, None, 3])
        s = Solution()
        res = s.rightSideView(root)
        self.assertListEqual(res, [1, 3])

    def test_right_side_view4(self):
        root = build_binary_tree([])
        s = Solution()
        res = s.rightSideView(root)
        self.assertListEqual(res, [])
