#!/bin/env python3

# python/tests/dsa/binary_tree/test_postorder_traversal.py


from unittest import TestCase
from dsa.binary_tree.tree_node import build_binary_tree
from dsa.binary_tree.postorder_traversal_recursive import Solution as S1
from dsa.binary_tree.postorder_traversal import Solution as S2
from dsa.binary_tree.postorder_traversal_with_two_stacks import Solution as S3


class TestPostorderTraversal(TestCase):

    def test_postorder_traversal_recursive1(self):
        root = build_binary_tree([1, None, 2, 3])
        s1 = S1()
        traversal = s1.postorderTraversal(root)
        # Assertion
        self.assertListEqual(traversal, [3, 2, 1])

    def test_postorder_traversal_recursive2(self):
        root = build_binary_tree([1, 2, 3, 4, 5, None, 8, None, None, 6, 7, 9])
        s1 = S1()
        traversal = s1.postorderTraversal(root)
        # Assertion
        self.assertListEqual(traversal, [4, 6, 7, 5, 2, 9, 8, 3, 1])

    def test_postorder_traversal_recursive3(self):
        root = build_binary_tree([])
        s1 = S1()
        traversal = s1.postorderTraversal(root)
        # Assertion
        self.assertListEqual(traversal, [])

    def test_postorder_traversal_recursive4(self):
        root = build_binary_tree([1])
        s1 = S1()
        traversal = s1.postorderTraversal(root)
        # Assertion
        self.assertListEqual(traversal, [1])

    def test_postorder_traversal1(self):
        root = build_binary_tree([1, None, 2, 3])
        s2 = S2()
        traversal = s2.postorderTraversal(root)
        # Assertion
        self.assertListEqual(traversal, [3, 2, 1])

    def test_postorder_traversal2(self):
        root = build_binary_tree([1, 2, 3, 4, 5, None, 8, None, None, 6, 7, 9])
        s2 = S2()
        traversal = s2.postorderTraversal(root)
        # Assertion
        self.assertListEqual(traversal, [4, 6, 7, 5, 2, 9, 8, 3, 1])

    def test_postorder_traversal3(self):
        root = build_binary_tree([])
        s2 = S2()
        traversal = s2.postorderTraversal(root)
        # Assertion
        self.assertListEqual(traversal, [])

    def test_postorder_traversal4(self):
        root = build_binary_tree([1])
        s2 = S2()
        traversal = s2.postorderTraversal(root)
        # Assertion
        self.assertListEqual(traversal, [1])

    def test_postorder_traversal_with_two_stacks1(self):
        root = build_binary_tree([1, None, 2, 3])
        s2 = S3()
        traversal = s2.postorderTraversal(root)
        # Assertion
        self.assertListEqual(traversal, [3, 2, 1])

    def test_postorder_traversal_with_two_stacks2(self):
        root = build_binary_tree([1, 2, 3, 4, 5, None, 8, None, None, 6, 7, 9])
        s2 = S3()
        traversal = s2.postorderTraversal(root)
        # Assertion
        self.assertListEqual(traversal, [4, 6, 7, 5, 2, 9, 8, 3, 1])

    def test_postorder_traversal_with_two_stacks3(self):
        root = build_binary_tree([])
        s2 = S3()
        traversal = s2.postorderTraversal(root)
        # Assertion
        self.assertListEqual(traversal, [])

    def test_postorder_traversal_with_two_stacks4(self):
        root = build_binary_tree([1])
        s2 = S3()
        traversal = s2.postorderTraversal(root)
        # Assertion
        self.assertListEqual(traversal, [1])
