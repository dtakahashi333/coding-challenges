# python/tests/dsa/binary_tree/test_lowest_common_ancestor.py

from unittest import TestCase

from dsa.binary_tree.lowest_common_ancestor import Solution
from dsa.binary_tree.tree_node import TreeNode, build_binary_tree


class TestLowestCommonAncestor(TestCase):
    def setUp(self):
        super().setUp()
        self.solution = Solution()

    def test_lowest_common_ancestor1(self):
        root = build_binary_tree([3, 5, 1, 6, 2, 0, 8, None, None, 7, 4])
        p = TreeNode(5)
        q = TreeNode(1)
        self.assertEqual(self.solution.lowestCommonAncestor(root, p, q).val, 3)

    def test_lowest_common_ancestor2(self):
        root = build_binary_tree([3, 5, 1, 6, 2, 0, 8, None, None, 7, 4])
        p = TreeNode(5)
        q = TreeNode(4)
        self.assertEqual(self.solution.lowestCommonAncestor(root, p, q).val, 5)

    def test_lowest_common_ancestor3(self):
        root = build_binary_tree([1, 2])
        p = TreeNode(1)
        q = TreeNode(2)
        self.assertEqual(self.solution.lowestCommonAncestor(root, p, q).val, 1)
