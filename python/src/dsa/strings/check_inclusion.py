#!/usr/bin/env python3

# python/src/dsa/strings/check_inclusion.py

# 567. Permutation in String
# https://leetcode.com/problems/permutation-in-string/description/


class Solution:
    def checkInclusion(self, s1: str, s2: str) -> bool:
        l1 = len(s1)
        l2 = len(s2)

        seen1 = [0] * 26
        for c1 in s1:
            seen1[ord(c1) - 97] += 1

        seen2 = [0] * 26
        for i in range(l2):
            seen2[ord(s2[i]) - 97] += 1
            if seen1 == seen2:
                return True

            if sum(seen2) == l1:
                seen2[ord(s2[i - (l1 - 1)]) - 97] -= 1

        return False

    def checkInclusion2(self, s1: str, s2: str) -> bool:
        l1 = len(s1)
        l2 = len(s2)

        if l1 > l2:
            return False

        seen1 = [0] * 26
        for c1 in s1:
            seen1[ord(c1) - ord("a")] += 1

        seen2 = [0] * 26
        left, right = 0, 0
        while right < l2:
            seen2[ord(s2[right]) - ord("a")] += 1
            right += 1

            if right - left > l1:
                seen2[ord(s2[left]) - ord("a")] -= 1
                left += 1

            if seen1 == seen2:
                return True

        return False


def main():
    s1 = "adc"
    s2 = "dcda"
    s = Solution()
    s.checkInclusion(s1, s2)


if __name__ == "__main__":
    main()
