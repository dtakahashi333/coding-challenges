# python/src/dsa/binary_tree/is_valid_bst.py

# 98. Validate Binary Search Tree
# https://leetcode.com/problems/validate-binary-search-tree/description/

from typing import Optional

from dsa.binary_tree.tree_node import TreeNode


class Solution:
    def isValidBST(self, root: Optional[TreeNode]) -> bool:
        return self.validate(root, None, None)

    def validate(
        self,
        node: TreeNode | None,
        lower_bound: int | None,
        upper_bound: int | None,
    ) -> bool:
        if node is None:
            return True

        if lower_bound is not None and node.val <= lower_bound:
            return False

        if upper_bound is not None and node.val >= upper_bound:
            return False

        return self.validate(
            node=node.left,
            lower_bound=lower_bound,
            upper_bound=node.val,
        ) and self.validate(
            node=node.right,
            lower_bound=node.val,
            upper_bound=upper_bound,
        )
