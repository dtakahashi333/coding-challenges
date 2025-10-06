#!/bin/env python3

# coding-challenges/python/src/cracking/arrays_strings/is_unique.py

from typing import Dict


class Solution:

    def isUnique(self, s: str) -> bool:
        hashmap: Dict[str, bool] = {}
        for i in range(len(s)):
            c = s[i]
            if c in hashmap:
                return False
            hashmap[c] = True
        return True

    def isUniqueWithoutDataStructures(self, s: str) -> bool:
        for i in range(len(s)):
            for j in range(i + 1, len(s), 1):
                if s[i] == s[j]:
                    return False
        return True
