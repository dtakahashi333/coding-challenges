#!/bin/env python3

# python/src/dsa/binary_tree/postorder_traversal.py

# 145. Binary Tree Postorder Traversal
# https://leetcode.com/problems/binary-tree-postorder-traversal/description/

from typing import List, Optional

from dsa.binary_tree.tree_node import TreeNode


class Solution:
    def postorderTraversal(self, root: Optional[TreeNode]) -> List[int]:
        if root is None:
            return []

        st1 = [root]
        st2 = []

        while st1:
            cur = st1.pop()
            st2.append(cur.val)

            if cur.left:
                st1.append(cur.left)
            if cur.right:
                st1.append(cur.right)

        return list(reversed(st2))
