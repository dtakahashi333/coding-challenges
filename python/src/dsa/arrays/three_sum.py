#!/usr/bin/env python3

# python/src/dsa/arrays/three_sum.py

# 15. 3Sum
# https://leetcode.com/problems/3sum/


class Solution:
    def threeSum(self, nums: list[int]) -> list[list[int]]:
        length = len(nums)
        result = set()
        for i in range(length):
            target = -nums[i]
            seen = set()
            for j in range(i + 1, length, 1):
                complement = target - nums[j]
                if complement in seen:
                    result.add(tuple(sorted([nums[i], nums[j], complement])))
                seen.add(nums[j])

        return [list(x) for x in result]

    def threeSum2(self, nums: list[int]) -> list[list[int]]:
        length = len(nums)
        nums.sort()
        result = set()
        for i in range(length):
            target = -nums[i]
            left = i + 1
            right = length - 1
            while left < right:
                if nums[left] + nums[right] == target:
                    result.add(tuple(sorted([nums[i], nums[left], nums[right]])))
                    if left < right:
                        left += 1
                        right -= 1
                elif nums[left] + nums[right] < target:
                    left += 1
                else:
                    right -= 1

        return [list(x) for x in result]

    def threeSum3(self, nums: list[int]) -> list[list[int]]:
        length = len(nums)
        nums.sort()
        result = []
        for i in range(length):
            if i > 0 and nums[i] == nums[i - 1]:
                continue
            target = -nums[i]
            left = i + 1
            right = length - 1
            while left < right:
                if nums[left] + nums[right] == target:
                    result.append([nums[i], nums[left], nums[right]])
                    if left < right:
                        current_num = nums[left]
                        while left < right and nums[left] == current_num:
                            left += 1
                        current_num = nums[right]
                        while left < right and nums[right] == current_num:
                            right -= 1
                elif nums[left] + nums[right] < target:
                    current_num = nums[left]
                    while left < right and nums[left] == current_num:
                        left += 1
                else:
                    current_num = nums[right]
                    while left < right and nums[right] == current_num:
                        right -= 1

        return result
