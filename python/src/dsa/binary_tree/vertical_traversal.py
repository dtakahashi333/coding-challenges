#!/bin/env python3

# python/src/dsa/binary_tree/vertical_traversal.py

from collections import deque
from typing import List, Optional

from dsa.binary_tree.tree_node import TreeNode


class Solution:
    def verticalTraversal(self, root: Optional[TreeNode]) -> List[List[int]]:
        if root is None:
            return []

        map1 = {}
        q = deque([(root, 0)])

        while q:
            size = len(q)

            map2 = {}
            for _ in range(size):
                node, idx = q.popleft()
                if idx not in map2:
                    map2.setdefault(idx, [])
                map2[idx].append(node.val)

                if node.left:
                    q.append((node.left, idx - 1))
                if node.right:
                    q.append((node.right, idx + 1))

            for idx in map2:
                if idx not in map1:
                    map1.setdefault(idx, [])
                map1[idx].extend(sorted(map2[idx]))

        res = []
        for idx in sorted(map1):
            res.append(map1[idx])

        return res
