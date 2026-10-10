# python/src/dsa/heaps/find_kth_largest.py

# 215. Kth Largest Element in an Array
# https://leetcode.com/problems/kth-largest-element-in-an-array/description/

import heapq


class Solution:
    def findKthLargest(self, nums: list[int], k: int) -> int:
        heap = []
        for num in nums:
            if len(heap) >= k:
                if num > heap[0]:
                    heapq.heappop(heap)
                    heapq.heappush(heap, num)
            else:
                heapq.heappush(heap, num)

        return heap[0]
