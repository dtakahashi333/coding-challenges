#!/bin/env python3

# python/src/dsa/binary_tree/postorder_traversal_recursive.py

# 145. Binary Tree Postorder Traversal
# https://leetcode.com/problems/binary-tree-postorder-traversal/description/

from typing import List, Optional

from dsa.binary_tree.tree_node import TreeNode


class Solution:
    def postorderTraversal(self, root: Optional[TreeNode]) -> List[int]:
        res = []
        self.helper(root, res)
        return res

    def helper(self, node: Optional[TreeNode], res: List[int]):
        if node is None:
            return

        self.helper(node.left, res)
        self.helper(node.right, res)
        res.append(node.val)
