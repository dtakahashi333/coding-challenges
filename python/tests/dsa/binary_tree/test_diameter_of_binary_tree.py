#!/bin/env python3

# python/tests/dsa/binary_tree/test_diameter_of_binary_tree.py

from unittest import TestCase
from dsa.binary_tree.diameter_of_binary_tree import Solution
from dsa.binary_tree.tree_node import build_binary_tree


class TestDiameterOfBinaryTree(TestCase):
    def test_diameter_of_binary_tree1(self):
        root = build_binary_tree([1, 2, 3, 4, 5])
        s = Solution()
        res = s.diameterOfBinaryTree(root)
        self.assertEqual(res, 3)

    def test_diameter_of_binary_tree2(self):
        root = build_binary_tree([1, 2])
        s = Solution()
        res = s.diameterOfBinaryTree(root)
        self.assertEqual(res, 1)

    def test_diameter_of_binary_tree3(self):
        root = build_binary_tree(
            [
                4,
                -7,
                -3,
                None,
                None,
                -9,
                -3,
                9,
                -7,
                -4,
                None,
                6,
                None,
                -6,
                -6,
                None,
                None,
                0,
                6,
                5,
                None,
                9,
                None,
                None,
                -1,
                -4,
                None,
                None,
                None,
                -2,
            ]
        )
        s = Solution()
        res = s.diameterOfBinaryTree(root)
        self.assertEqual(res, 8)
