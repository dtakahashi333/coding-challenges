#!/bin/env python3

# python/src/dsa/binary_tree/postorder_traversal.py

# 145. Binary Tree Postorder Traversal
# https://leetcode.com/problems/binary-tree-postorder-traversal/description/

from collections import deque
from typing import List, Optional

from dsa.binary_tree.tree_node import TreeNode


class Solution:
    def postorderTraversal(self, root: Optional[TreeNode]) -> List[int]:
        if root is None:
            return []

        res = deque()
        dq = deque()
        cur = root

        while cur or dq:
            while cur:
                res.appendleft(cur.val)
                dq.append(cur)
                cur = cur.right

            cur = dq.pop()

            cur = cur.left

        return list(res)
