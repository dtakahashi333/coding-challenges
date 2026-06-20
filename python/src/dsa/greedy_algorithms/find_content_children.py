#!/bin/env python3

# python/src/dsa/greedy_algorithms/find_content_children.py

# 455. Assign Cookies
# https://leetcode.com/problems/assign-cookies/description/

from typing import List


class Solution:
    def findContentChildren(self, g: List[int], s: List[int]) -> int:
        g.sort(reverse=True)
        s.sort(reverse=True)

        i, j = 0, 0
        while i < len(g) and j < len(s):
            if s[j] >= g[i]:
                i, j = i + 1, j + 1
            else:
                i = i + 1

        return j
