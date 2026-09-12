# python/src/dsa/binary_tree/lowest_common_ancestor.py

# 236. Lowest Common Ancestor of a Binary Tree
# https://leetcode.com/problems/lowest-common-ancestor-of-a-binary-tree/description/

from dsa.binary_tree.tree_node import TreeNode


class Solution:
    def lowestCommonAncestor(
        self, root: TreeNode, p: TreeNode, q: TreeNode
    ) -> TreeNode | None:
        if root is None:
            return None

        if root.val == p.val or root.val == q.val:
            return root

        left_result = self.lowestCommonAncestor(root.left, p, q)
        right_result = self.lowestCommonAncestor(root.right, p, q)

        if left_result is not None and right_result is not None:
            return root
        if left_result is not None:
            return left_result
        if right_result is not None:
            return right_result
        return None
