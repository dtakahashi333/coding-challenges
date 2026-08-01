#!/usr/bin/env python3

# python/src/dsa/binary_search/rotated_sorted_arrays.py

# 33. Search in Rotated Sorted Array
# https://leetcode.com/problems/search-in-rotated-sorted-array/description/

from typing import List


class Solution:
    def search(self, nums: List[int], target: int) -> int:
        length = len(nums)
        first, last = 0, length - 1
        while first <= last:
            middle = first + (last - first) // 2
            num = nums[middle]
            if num == target:
                return middle
            elif num < target:
                if num < nums[first] <= target:
                    last = middle - 1
                else:
                    first = middle + 1
            else:
                if target <= nums[last] < num:
                    first = middle + 1
                else:
                    last = middle - 1

        return -1


def main():
    nums = [3,1]
    target = 1
    s = Solution()
    print(s.search(nums, target))


if __name__ == "__main__":
    main()
