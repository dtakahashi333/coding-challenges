#!/bin/env python3

# python/src/dsa/binary_tree/right_side_view.py

# 199. Binary Tree Right Side View
# https://leetcode.com/problems/binary-tree-right-side-view/description/

from collections import deque
from typing import List, Optional

from dsa.binary_tree.tree_node import TreeNode


class Solution:
    def rightSideView(self, root: Optional[TreeNode]) -> List[int]:
        if root is None:
            return []

        res = []
        q = deque()
        q.append(root)

        while q:
            level = []
            size = len(q)
            for _ in range(size):
                node = q.popleft()
                level.append(node.val)
                if node.left:
                    q.append(node.left)
                if node.right:
                    q.append(node.right)

            res.append(level[-1])

        return res
