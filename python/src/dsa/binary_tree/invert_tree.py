# python/src/dsa/binary_tree/invert_tree.py

# 226. Invert Binary Tree
# https://leetcode.com/problems/invert-binary-tree/description/

from typing import Optional
from collections import deque

from dsa.binary_tree.tree_node import TreeNode


class Solution:
    def invertTree(self, root: Optional[TreeNode]) -> Optional[TreeNode]:
        if not root:
            return None
        q = deque()
        q.append(root)
        while q:
            node = q.popleft()
            # Swap the left and right children.
            if node:
                tmp = node.left
                node.left = node.right
                node.right = tmp
                if node.left:
                    q.append(node.left)
                if node.right:
                    q.append(node.right)

        return root
