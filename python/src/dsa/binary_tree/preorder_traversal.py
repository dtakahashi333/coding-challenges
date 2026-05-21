#!/bin/env python3

# 144. Binary Tree Preorder Traversal
# https://leetcode.com/problems/binary-tree-preorder-traversal/description/

from typing import List, Optional
from dsa.binary_tree.tree_node import TreeNode
from collections import deque


class Solution:
    def preorderTraversal(self, root: Optional[TreeNode]) -> List[int]:
        if root is None:
            return []

        res = []
        st = deque()
        st.append(root)

        while st:
            curr = st.pop()
            res.append(curr.val)

            # push right first (so left is processed first)
            if curr.right:
                st.append(curr.right)
            if curr.left:
                st.append(curr.left)

        return res
