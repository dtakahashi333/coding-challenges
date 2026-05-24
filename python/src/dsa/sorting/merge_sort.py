#!/bin/env python3

# python/src/dsa/sorting/merge_sort.py

from typing import List


class Solution:
    def mergeSort(self, nums: List[int]) -> List[int]:
        size = len(nums)
        self.helper(nums, 0, size - 1)
        return nums

    def helper(self, nums: List[int], start: int, end: int):
        if end <= start:
            return

        mid = start + (end - start) // 2

        self.helper(nums, start, mid)
        self.helper(nums, mid + 1, end)

        self.merge(nums, start, mid, end)

    def merge(self, nums: List[int], start: int, mid: int, end: int):
        cur1 = start
        end1 = mid
        cur2 = mid + 1
        end2 = end

        tmp = []
        while cur1 <= end1 and cur2 <= end2:
            if nums[cur1] <= nums[cur2]:
                tmp.append(nums[cur1])
                cur1 += 1
            else:
                tmp.append(nums[cur2])
                cur2 += 1

        for i in range(cur1, end1 + 1):
            tmp.append(nums[i])
        for i in range(cur2, end2 + 1):
            tmp.append(nums[i])

        cur = start
        for num in tmp:
            nums[cur] = num
            cur += 1
