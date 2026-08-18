# python/src/dsa/binary_tree/is_same_tree.py

# 100. Same Tree
# https://leetcode.com/problems/same-tree/description/

from typing import Optional

from dsa.binary_tree.tree_node import TreeNode


class Solution:
    def isSameTree(self, p: Optional[TreeNode], q: Optional[TreeNode]) -> bool:
        if p is None and q is None:
            return True

        if (p is None and q is not None) or (p is not None and q is None):
            return False

        return (
            p.val == q.val
            and self.isSameTree(p.left, q.left)
            and self.isSameTree(p.right, q.right)
        )
