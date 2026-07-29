#!/usr/bin/env python3

# python/src/dsa/arrays/daily_temperatures.py

# 739. Daily Temperatures
# https://leetcode.com/problems/daily-temperatures/description/

from typing import List


class Solution:
    def dailyTemperatures(self, temperatures: List[int]) -> List[int]:
        length = len(temperatures)
        result = [0] * length
        for i in range(length):
            for j in range(i + 1, length, 1):
                if temperatures[j] > temperatures[i]:
                    result[i] = j - i
                    break

        return result

    def dailyTemperatures2(self, temperatures: List[int]) -> List[int]:
        length = len(temperatures)
        result = [0] * length
        stack = [0]
        for j in range(1, length, 1):
            while stack and temperatures[j] > temperatures[stack[-1]]:
                i = stack.pop()
                result[i] = j - i
            stack.append(j)

        return result


def main():
    temperatures = [73, 74, 75, 71, 69, 72, 76, 73]
    s = Solution()
    s.dailyTemperatures2(temperatures)  # [1, 1, 4, 2, 1, 1, 0, 0]


if __name__ == "__main__":
    main()
