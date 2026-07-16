#!/usr/bin/env python3

# python/src/dsa/strings/is_anagram.py

# 242. Valid Anagram
# https://leetcode.com/problems/valid-anagram/description/


class Solution:
    def isAnagram(self, s: str, t: str) -> bool:
        count = [0] * 26
        for c in s:
            count[ord(c) - 97] += 1
        for c in t:
            pos = ord(c) - 97
            count[pos] -= 1
            if count[pos] < 0:
                return False

        count = [x for x in count if x > 0]

        return len(count) == 0
