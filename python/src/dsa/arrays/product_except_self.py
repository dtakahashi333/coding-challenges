#!/usr/bin/env python3

# python/src/dsa/arrays/product_except_self.py

# 238. Product of Array Except Self
# https://leetcode.com/problems/product-of-array-except-self/description/

from typing import List


class Solution:
    def productExceptSelf(self, nums: List[int]) -> List[int]:
        length = len(nums)
        left_product = [0] * length
        for i, num in enumerate(nums):
            if i == 0:
                left_product[i] = num
            else:
                left_product[i] = left_product[i - 1] * num

        right_product = [0] * length
        for i, num in enumerate(reversed(nums)):
            if i == 0:
                right_product[length - 1 - i] = num
            else:
                right_product[length - 1 - i] = right_product[length - i] * num

        product = [0] * length
        for i in range(length):
            if i == 0:
                product[i] = right_product[i + 1]
            elif i == length - 1:
                product[i] = left_product[i - 1]
            else:
                product[i] = left_product[i - 1] * right_product[i + 1]

        return product

    def productExceptSelf2(self, nums: List[int]) -> List[int]:
        length = len(nums)
        product = [1] * length
        for i in range(1, length, 1):
            product[i] = product[i - 1] * nums[i - 1]
        right_product = 1
        for i in range(length - 2, -1, -1):
            product[i] *= right_product * nums[i + 1]
            right_product *= nums[i + 1]

        return product
