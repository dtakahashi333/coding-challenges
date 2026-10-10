# python/src/dsa/heaps/kth_largest.py

# 703. Kth Largest Element in a Stream
# https://leetcode.com/problems/kth-largest-element-in-a-stream/description/

import heapq


class KthLargest:

    def __init__(self, k: int, nums: list[int]):
        self.k = k
        self.heap = []
        for i in range(len(nums)):
            self.add(nums[i])

    def add(self, val: int) -> int:
        if len(self.heap) >= self.k:
            if self.heap[0] < val:
                heapq.heappop(self.heap)
            else:
                return self.heap[0]
            
        heapq.heappush(self.heap, val)
        
        return self.heap[0]
