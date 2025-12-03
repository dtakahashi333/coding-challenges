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

        current = root
        stack = []

        while current is not None or len(stack) > 0:
            while current is not None:
                result.append(current.val)
                stack.append(current)
                current = current.left

            # current is None.
            current = stack.pop()

            current = current.right

        return result
