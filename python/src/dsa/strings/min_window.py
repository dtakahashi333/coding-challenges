#!/usr/bin/env python3

# python/src/dsa/strings/min_window.py

# 76. Minimum Window Substring
# https://leetcode.com/problems/minimum-window-substring/


class Solution:
    def minWindow(self, s: str, t: str) -> str:
        if len(s) < len(t):
            return ""

        need = [0] * 52
        for c in t:
            if "a" <= c <= "z":
                need[ord(c) - ord("a")] += 1
            elif "A" <= c <= "Z":
                need[ord(c) - ord("A") + 26] += 1

        min_count = len(s) + 1
        min_window = ""
        window = [0] * 52
        left, right = 0, 0
        while right < len(s):
            if "a" <= s[right] <= "z":
                window[ord(s[right]) - ord("a")] += 1
            elif "A" <= s[right] <= "Z":
                window[ord(s[right]) - ord("A") + 26] += 1

            if self.isSubset(need, window):
                # shrink the window
                while self.isSubset(need, window):
                    if "a" <= s[left] <= "z":
                        window[ord(s[left]) - ord("a")] -= 1
                    elif "A" <= s[left] <= "Z":
                        window[ord(s[left]) - ord("A") + 26] -= 1
                    left += 1

                if min_count > right - (left - 1) + 1:
                    min_count = right - (left - 1) + 1
                    min_window = s[left - 1 : right + 1]

            right += 1

        return min_window

    def isSubset(self, s1, s2):
        for i in range(52):
            if s1[i] != 0 and s1[i] > s2[i]:
                return False
        return True

    def minWindow2(self, s: str, t: str) -> str:
        if len(s) < len(t):
            return ""

        need = {}
        for c in t:
            need[c] = need.get(c, 0) + 1
        required = len(need)

        min_count = len(s) + 1
        min_window = ""
        window = {}
        left, right = 0, 0
        formed = 0
        while right < len(s):
            c = s[right]
            window[c] = window.get(c, 0) + 1
            if c in need and window[c] == need[c]:
                formed += 1

            while required == formed:
                if min_count > right - left + 1:
                    min_count = right - left + 1
                    min_window = s[left : right + 1]
                # shrink the window
                c = s[left]
                window[c] -= 1
                left += 1
                if c in need and window[c] < need[c]:
                    formed -= 1

            right += 1

        return min_window


def main():
    s = "a"
    t = "a"
    sol = Solution()
    print(sol.minWindow2(s, t))


if __name__ == "__main__":
    main()
