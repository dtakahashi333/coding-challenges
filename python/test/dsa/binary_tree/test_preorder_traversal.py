#!/bin/env python3

from unittest import TestCase
from binary_tree import build_binary_tree

# from inorder_traversal import Solution as S2
from preorder_traversal_recursive import Solution as S1


class TestInorderTraversal(TestCase):

    def test_preorder_traversal1(self):
        root = build_binary_tree([1, None, 2, 3])
        s = S1()
        traversal = s.preorderTraversal(root)
        # Assertion
        self.assertListEqual(traversal, [1, 2, 3])

    def test_preorder_traversal2(self):
        root = build_binary_tree([1, 2, 3, 4, 5, None, 8, None, None, 6, 7, 9])
        s = S1()
        traversal = s.preorderTraversal(root)
        # Assertion
        self.assertListEqual(traversal, [1, 2, 4, 5, 6, 7, 3, 8, 9])

    def test_preorder_traversal3(self):
        root = build_binary_tree([])
        s = S1()
        traversal = s.preorderTraversal(root)
        # Assertion
        self.assertListEqual(traversal, [])

    def test_preorder_traversal4(self):
        root = build_binary_tree([1])
        s = S1()
        traversal = s.preorderTraversal(root)
        # Assertion
        self.assertListEqual(traversal, [1])
