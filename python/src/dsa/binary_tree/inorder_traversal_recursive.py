#!/bin/env python3

# 94. Binary Tree Inorder Traversal
# https://leetcode.com/problems/binary-tree-inorder-traversal/submissions/1757854771/

from typing import List, Optional
from binary_tree import TreeNode


class Solution:

    def inorderTraversal(self, root: Optional[TreeNode]) -> List[int]:
        result = []
        self.inorderTraversalHelper(root, result)
        return result

    def inorderTraversalHelper(self, node: TreeNode, result: List[int]):
        if node is None:
            return
        self.inorderTraversalHelper(node.left, result)
        result.append(node.val)
        self.inorderTraversalHelper(node.right, result)
