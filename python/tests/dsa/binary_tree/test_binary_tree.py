#!/bin/env python3

from unittest import TestCase
from dsa.binary_tree.binary_tree import build_binary_tree


class TestBinaryTree(TestCase):

    def test_build_binary_tree(self):
        items = [1, None, 2, 3]
        tree = build_binary_tree(items)

        # Assertions
        self.assertEqual(tree.val, 1)
        self.assertIsNone(tree.left)
        self.assertIsNotNone(tree.right)
        self.assertEqual(tree.right.val, 2)
        self.assertIsNotNone(tree.right.left)
        self.assertEqual(tree.right.left.val, 3)
