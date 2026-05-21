#!/bin/env python3

# python/src/dsa/binary_tree/zigzag_level_order_traversal.py

# 103. Binary Tree Zigzag Level Order Traversal
# https://leetcode.com/problems/binary-tree-zigzag-level-order-traversal/description/

from collections import deque
from typing import List, Optional

from dsa.binary_tree.tree_node import TreeNode


class Solution:
    def zigzagLevelOrder(self, root: Optional[TreeNode]) -> List[List[int]]:
        if root is None:
            return []

        res = []
        q = deque()
        q.append(root)

        left_to_right = True

        while q:
            level = deque()
            size = len(q)
            for _ in range(size):
                node = q.popleft()

                if left_to_right:
                    level.append(node.val)
                else:
                    level.appendleft(node.val)

                if node.left:
                    q.append(node.left)
                if node.right:
                    q.append(node.right)

            res.append(list(level))
            left_to_right = not left_to_right

        return res
