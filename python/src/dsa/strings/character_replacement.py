#!/usr/bin/env python3

# python/src/dsa/strings/character_replacement.py

# 424. Longest Repeating Character Replacement
# https://leetcode.com/problems/longest-repeating-character-replacement/description/


class Solution:
    def characterReplacement(self, s: str, k: int) -> int:
        length = len(s)
        max_count = 0
        left, right = 0, 0
        while right < length:
            current_char = s[right]
            kk = k
            while True:
                while right < length and s[right] == current_char:
                    right += 1
                if right > length - 1 or kk == 0:
                    break
                kk -= 1
                right += 1

            max_count = max(max_count, right - left + min(left, kk))

            while left < length and s[left] == current_char:
                left += 1
            right = left

        return max_count

    def characterReplacement2(self, s: str, k: int) -> int:
        length = len(s)
        max_count = 0
        left, right = 0, 0
        seen = [0] * 26
        while right < length:
            seen[ord(s[right]) - 65] += 1
            right += 1

            while (right - left) - max(seen) > k:
                seen[ord(s[left]) - 65] -= 1
                left += 1

            max_count = max(max_count, (right - left))

        return max_count


def main():
    # s = "CADBBB"
    # k = 4
    # sol = Solution()
    # sol.characterReplacement2(s, k)
    # s = "ABAA"
    # k = 0
    # sol = Solution()
    # sol.characterReplacement2(s, k)
    s = "AABABBA"
    k = 1
    sol = Solution()
    sol.characterReplacement2(s, k)


if __name__ == "__main__":
    main()
