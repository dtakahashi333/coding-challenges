# python/tests/dsa/binary_tree/test_is_valid_bst.py

from unittest import TestCase

from dsa.binary_tree.is_valid_bst import Solution
from dsa.binary_tree.tree_node import build_binary_tree


class TestIsValidBST(TestCase):
    def setUp(self):
        super().setUp()
        self.solution = Solution()

    def test_is_valid_bst1(self):
        root = build_binary_tree([2, 1, 3])
        result = self.solution.isValidBST(root)
        self.assertTrue(result)

    def test_is_valid_bst2(self):
        root = build_binary_tree([5, 1, 4, None, None, 3, 6])
        result = self.solution.isValidBST(root)
        self.assertFalse(result)

    def test_is_valid_bst3(self):
        root = build_binary_tree([5, 4, 6, None, None, 3, 7])
        result = self.solution.isValidBST(root)
        self.assertFalse(result)

    def test_is_valid_bst4(self):
        root = build_binary_tree([2, 2, 2])
        result = self.solution.isValidBST(root)
        self.assertFalse(result)

    def test_is_valid_bst4(self):
        root = build_binary_tree([0, None, -1])
        result = self.solution.isValidBST(root)
        self.assertFalse(result)
