#!/usr/bin/env python3

# python/src/dsa/strings/is_anagram.py

# 242. Valid Anagram
# https://leetcode.com/problems/valid-anagram/description/


class Solution:
    def isAnagram(self, s: str, t: str) -> bool:
        freq = [0] * 26
        for c in s:
            freq[ord(c) - 97] += 1
        for c in t:
            pos = ord(c) - 97
            freq[pos] -= 1
            if freq[pos] < 0:
                return False

        freq = [x for x in freq if x > 0]

        return len(freq) == 0
