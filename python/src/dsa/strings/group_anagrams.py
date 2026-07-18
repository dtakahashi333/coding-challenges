#!/usr/bin/env python3

# python/src/dsa/strings/group_anagrams.py

# 49. Group Anagrams
# https://leetcode.com/problems/group-anagrams/description/

from typing import List


class Solution:
    def groupAnagrams(self, strs: List[str]) -> List[List[str]]:
        freq = [[0] * 26 for _ in range(len(strs))]
        for i, word in enumerate(strs):
            for ch in word:
                freq[i][ord(ch) - 97] += 1
        groups = []
        for i in range(len(freq)):
            if len([x for x in groups if i in x]) != 0:
                continue
            new_group = set([i])
            for j in range(i + 1, len(freq), 1):
                if len([x for x in groups if j in x]) != 0:
                    continue
                is_anagram = True
                for k in range(26):
                    if freq[i][k] != freq[j][k]:
                        is_anagram = False
                        break
                if is_anagram:
                    new_group.add(j)
            groups.append(new_group)
        anagram_groups = []
        for group in groups:
            anagram_group = []
            for i in group:
                anagram_group.append(strs[i])
            anagram_groups.append(anagram_group)

        return anagram_groups

    def groupAnagrams2(self, strs: List[str]) -> List[List[str]]:
        groups = {}
        for word in strs:
            freq = [0] * 26
            for c in word:
                freq[ord(c) - 97] += 1

            freq = tuple(freq)  # convert List to Tuple
            groups.setdefault(freq, []).append(word)

        return list(groups.values())
