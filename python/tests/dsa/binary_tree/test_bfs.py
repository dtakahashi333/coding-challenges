#!/bin/env python3

# python/tests/dsa/binary_tree/test_bfs.py

from unittest import TestCase
from dsa.binary_tree.tree_node import build_binary_tree
from dsa.binary_tree.bfs import Solution


class TestBFS(TestCase):

    def test_bfs1(self):
        root = build_binary_tree([])
        s = Solution()
        res = s.bfs(root)
        # Assertion
        self.assertListEqual(res, [])

    def test_bfs2(self):
        root = build_binary_tree([1])
        s = Solution()
        res = s.bfs(root)
        # Assertion
        self.assertListEqual(res, [1])

    def test_bfs3(self):
        root = build_binary_tree([1, 2, 3, 4, 5, 6, 7])
        s = Solution()
        res = s.bfs(root)
        # Assertion
        self.assertListEqual(res, [1, 2, 3, 4, 5, 6, 7])

    def test_bfs4(self):
        root = build_binary_tree([1, 2, None, 3, None, 4])
        s = Solution()
        res = s.bfs(root)
        # Assertion
        self.assertListEqual(res, [1, 2, 3, 4])

    def test_bfs5(self):
        root = build_binary_tree([1, None, 2, None, 3, None, 4])
        s = Solution()
        res = s.bfs(root)
        # Assertion
        self.assertListEqual(res, [1, 2, 3, 4])

    def test_bfs6(self):
        root = build_binary_tree([1, 2, 3, None, 5, None, 7])
        s = Solution()
        res = s.bfs(root)
        # Assertion
        self.assertListEqual(res, [1, 2, 3, 5, 7])

    def test_bfs7(self):
        root = build_binary_tree([10, 5, 20, 3, None, 15, 30, 12])
        s = Solution()
        res = s.bfs(root)
        # Assertion
        self.assertListEqual(res, [10, 5, 20, 3, 15, 30, 12])
