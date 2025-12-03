#!/bin/env python3

from unittest import TestCase
from dsa.binary_tree.tree_node import build_binary_tree
from dsa.binary_tree.inorder_traversal import Solution as S2
from dsa.binary_tree.inorder_traversal_recursive import Solution as S1


class TestInorderTraversal(TestCase):

    def test_inorder_traversal_recursive1(self):
        root = build_binary_tree([1, None, 2, 3])
        s1 = S1()
        traversal = s1.inorderTraversal(root)
        # Assertion
        self.assertListEqual(traversal, [1, 3, 2])

    def test_inorder_traversal_recursive2(self):
        root = build_binary_tree([1, 2, 3, 4, 5, None, 8, None, None, 6, 7, 9])
        s1 = S1()
        traversal = s1.inorderTraversal(root)
        # Assertion
        self.assertListEqual(traversal, [4, 2, 6, 5, 7, 1, 3, 9, 8])

    def test_inorder_traversal_recursive3(self):
        root = build_binary_tree([])
        s1 = S1()
        traversal = s1.inorderTraversal(root)
        # Assertion
        self.assertListEqual(traversal, [])

    def test_inorder_traversal_recursive4(self):
        root = build_binary_tree([1])
        s1 = S1()
        traversal = s1.inorderTraversal(root)
        # Assertion
        self.assertListEqual(traversal, [1])

    def test_inorder_traversal1(self):
        root = build_binary_tree([1, None, 2, 3])
        s2 = S2()
        traversal = s2.inorderTraversal(root)
        # Assertion
        self.assertListEqual(traversal, [1, 3, 2])

    def test_inorder_traversal2(self):
        root = build_binary_tree([1, 2, 3, 4, 5, None, 8, None, None, 6, 7, 9])
        s2 = S2()
        traversal = s2.inorderTraversal(root)
        # Assertion
        self.assertListEqual(traversal, [4, 2, 6, 5, 7, 1, 3, 9, 8])

    def test_inorder_traversal3(self):
        root = build_binary_tree([])
        s2 = S2()
        traversal = s2.inorderTraversal(root)
        # Assertion
        self.assertListEqual(traversal, [])

    def test_inorder_traversal4(self):
        root = build_binary_tree([1])
        s2 = S2()
        traversal = s2.inorderTraversal(root)
        # Assertion
        self.assertListEqual(traversal, [1])
