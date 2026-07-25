#!/usr/bin/env python3

# python/src/dsa/strings/length_of_longest_substring.py

# 3. Longest Substring Without Repeating Characters
# https://leetcode.com/problems/longest-substring-without-repeating-characters/description/


class Solution:
    def lengthOfLongestSubstring(self, s: str) -> int:
        length = len(s)
        max_count = 0
        for i, c in enumerate(s):
            seen = set()
            seen.add(c)
            count = 1
            for j in range(i + 1, length, 1):
                if s[j] in seen:
                    break
                seen.add(s[j])
                count += 1

            max_count = max(max_count, count)

        return max_count

    def lengthOfLongestSubstring2(self, s: str) -> int:
        length = len(s)
        left, right = 0, 0
        max_count = 0
        seen = set()
        while True:
            while right < length:
                if s[right] in seen:
                    break
                seen.add(s[right])
                right += 1
            max_count = max(max_count, right - left)
            if right > length - 1:
                break
            while s[left] != s[right]:
                seen.remove(s[left])
                left += 1
            seen.remove(s[left])
            left += 1

        return max_count
