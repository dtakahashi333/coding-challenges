#!/bin/env python3

# python/src/dsa/binary_tree/bfs.py

# BFS

from typing import List, Optional
from collections import deque

from dsa.binary_tree.tree_node import TreeNode


class Solution:
    def bfs(self, root: Optional[TreeNode]) -> List[int]:
        if root is None:
            return []
        res = []
        q = deque()
        q.append(root)
        while q:
            node = q.popleft()
            res.append(node.val)
            if node.left is not None:
                q.append(node.left)
            if node.right is not None:
                q.append(node.right)

        return res
