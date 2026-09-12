# python/src/dsa/binary_tree/kth_smallest.py

# 230. Kth Smallest Element in a BST
# https://leetcode.com/problems/kth-smallest-element-in-a-bst/description/

from typing import Optional

from dsa.binary_tree.tree_node import TreeNode


class Solution:
    def kthSmallest(self, root: Optional[TreeNode], k: int) -> int:
        if root is None:
            raise ValueError("Tree must not be empty")

        stack: list[TreeNode] = []
        current: TreeNode | None = root

        while current or stack:
            while current:
                stack.append(current)
                current = current.left

            current = stack.pop()
            k -= 1

            if k == 0:
                return current.val

            current = current.right
