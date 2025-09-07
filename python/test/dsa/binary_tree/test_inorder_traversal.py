#!/bin/env python3

from unittest import TestCase
from binary_tree import build_binary_tree
from inorder_traversal import Solution as S2
from inorder_traversal_recursive import Solution as S1


class TestInorderTraversal(TestCase):

    def test_inorder_traversal1(self):
        root = build_binary_tree([1, None, 2, 3])
        s = S1()
        traversal = s.inorderTraversal(root)
        # Assertion
        self.assertListEqual(traversal, [1, 3, 2])

    def test_inorder_traversal2(self):
        root = build_binary_tree([1, 2, 3, 4, 5, None, 8, None, None, 6, 7, 9])
        s = S1()
        traversal = s.inorderTraversal(root)
        # Assertion
        self.assertListEqual(traversal, [4, 2, 6, 5, 7, 1, 3, 9, 8])

    def test_inorder_traversal3(self):
        root = build_binary_tree([1, None, 2, 3])
        s = S2()
        traversal = s.inorderTraversal(root)
        # Assertion
        self.assertListEqual(traversal, [1, 3, 2])

    def test_inorder_traversal4(self):
        root = build_binary_tree([1, 2, 3, 4, 5, None, 8, None, None, 6, 7, 9])
        s = S2()
        traversal = s.inorderTraversal(root)
        # Assertion
        self.assertListEqual(traversal, [4, 2, 6, 5, 7, 1, 3, 9, 8])
