from typing import List, Optional


# Definition for a binary tree node.
class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right


def build_binary_tree(items: List[int]) -> Optional[TreeNode]:
    """
    Returns a binary tree from given a list of integers.

    Args:
        items (List[int]): A list of integers.

    Returns:
        Optional[TreeNode]: The root node of the binary tree, or None if the list is empty.
    """
    if items is None or len(items) == 0:
        return None

    root = TreeNode(items[0])
    queue = [root]
    i = 1

    while i < len(items) and len(queue) > 0:
        node = queue.pop(0)
        if items[i] is not None:
            left = TreeNode(items[i])
            node.left = left
            queue.append(left)
        i += 1
        if i < len(items) and items[i] is not None:
            right = TreeNode(items[i])
            node.right = right
            queue.append(right)
        i += 1

    return root
