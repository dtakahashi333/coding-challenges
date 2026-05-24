#!/bin/env python3

# python/src/dsa/binary_tree/diameter_of_binary_tree.py

from typing import Optional

from dsa.binary_tree.tree_node import TreeNode


class Solution:
    def diameterOfBinaryTree(self, root: Optional[TreeNode]) -> int:
        if root is None:
            return 0

        left_height = self.height(root.left)
        right_height = self.height(root.right)

        left_diameter = self.diameterOfBinaryTree(root.left)
        right_diameter = self.diameterOfBinaryTree(root.right)

        return max(left_height + right_height, left_diameter, right_diameter)

    def height(self, node: Optional[TreeNode]) -> int:
        if node is None:
            return 0
        return 1 + max(self.height(node.left), self.height(node.right))
