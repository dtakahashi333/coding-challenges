#!/usr/bin/env pythoh3

# python/src/dsa/arrays/top_k_frequent.py

# 347. Top K Frequent Elements
# https://leetcode.com/problems/top-k-frequent-elements/description/

from typing import List


class Solution:
    def topKFrequent(self, nums: List[int], k: int) -> List[int]:
        freq = {}
        for n in nums:
            freq[n] = freq.get(n, 0) + 1

        pairs = list(freq.items())
        pairs.sort(key=lambda x: x[1], reverse=True)

        return [n for n, _ in pairs[:k]]

    def topKFrequent2(self, nums: List[int], k: int) -> List[int]:
        freq = {}
        for n in nums:
            freq[n] = freq.get(n, 0) + 1

        bucket = [[] for _ in range(len(nums) + 1)]
        for key, val in freq.items():
            bucket[val].append(key)

        result = []
        for numbers in reversed(bucket):
            for num in numbers:
                result.append(num)
                if len(result) == k:
                    return result

        return result
