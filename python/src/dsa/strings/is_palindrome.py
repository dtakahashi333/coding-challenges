#!/usr/bin/env python3

# python/src/dsa/strings/is_palindrome.py

# 125. Valid Palindrome
# https://leetcode.com/problems/valid-palindrome/description/


class Solution:
    def isPalindrome(self, s: str) -> bool:
        head = 0
        tail = len(s) - 1
        """
        Although there are nested loops, the pointers only move in one direction: head moves from left to right and tail moves from right to left. Neither pointer ever moves backward, so each character is examined at most once. Therefore the total time complexity is O(n), not O(n²).
        """
        while head < tail:
            while head < tail and not s[head].isalnum():
                head += 1
            while head < tail and not s[tail].isalnum():
                tail -= 1
            if head >= tail:
                break
            if s[head].lower() != s[tail].lower():
                return False
            head += 1
            tail -= 1

        return True
