# python/src/dsa/heaps/k_closest.py

# 973. K Closest Points to Origin
# https://leetcode.com/problems/k-closest-points-to-origin/description/

import heapq


class Solution:
    def kClosest(self, points: list[list[int]], k: int) -> list[list[int]]:
        heap = []
        for point in points:
            x = point[0]
            y = point[1]
            distance = x**2 + y**2
            if len(heap) >= k:
                if distance < heap[0][0]:
                    heapq.heapreplace_max(heap, (distance, point))
            else:
                heapq.heappush_max(heap, (distance, point))

        return [point for _, point in heap]
