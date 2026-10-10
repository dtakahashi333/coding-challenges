# python/src/dsa/heaps/median_finder.py

# 295. Find Median from Data Stream
# https://leetcode.com/problems/find-median-from-data-stream/description/

import heapq


class MedianFinder:

    def __init__(self):
        # first_half is a max_heap, and second_half is a min_heap
        # first_half maintains smaller half of numbers
        # second_half maintains larger half of numbers
        self.first_half, self.second_half = [], []

    def addNum(self, num: int) -> None:
        if len(self.first_half) == len(self.second_half):
            # add num to the first half
            if len(self.second_half) != 0 and self.second_half[0] < num:
                heapq.heappush(self.second_half, num)
                heapq.heappush_max(self.first_half, heapq.heappop(self.second_half))
            else:
                heapq.heappush_max(self.first_half, num)
        else:
            # add num to the second half
            if self.first_half[0] > num:
                heapq.heappush_max(self.first_half, num)
                heapq.heappush(self.second_half, heapq.heappop_max(self.first_half))
            else:
                heapq.heappush(self.second_half, num)

        print(self.first_half, self.second_half)

    def findMedian(self) -> float:
        if len(self.first_half) == len(self.second_half):
            return (self.first_half[0] + self.second_half[0]) / 2
        else:
            return self.first_half[0]
