#!/bin/env python3

# 94. Binary Tree Inorder Traversal
# https://leetcode.com/problems/binary-tree-inorder-traversal/submissions/1757854771/

from typing import List, Optional
from binary_tree import TreeNode


class Solution:

    def inorderTraversal(self, root: Optional[TreeNode]) -> List[int]:
        result = []
        if root is None:
            return result

        current = root
        stack = []

        while current is not None or len(stack) > 0:
            if current is not None:
                if current.left is not None:
                    stack.append(current)
                    current = current.left
                else:
                    result.append(current.val)
                    current = current.right
            elif len(stack) > 0:
                current = stack.pop()
                result.append(current.val)
                current = current.right

        return result
