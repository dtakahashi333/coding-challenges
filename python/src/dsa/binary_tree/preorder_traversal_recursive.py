#!/bin/env python3

# 144. Binary Tree Preorder Traversal
# https://leetcode.com/problems/binary-tree-preorder-traversal/description/

from typing import List, Optional
from dsa.binary_tree.tree_node import TreeNode


class Solution:

    def preorderTraversal(self, root: Optional[TreeNode]) -> List[int]:
        result = []
        if root is None:
            return result
        self.preorderTraversalHelper(root, result)
        return result

    def preorderTraversalHelper(self, node: Optional[TreeNode], result: List[int]):
        if node is None:
            return

        result.append(node.val)
        if node.left:
            self.preorderTraversalHelper(node.left, result)
        if node.right:
            self.preorderTraversalHelper(node.right, result)
