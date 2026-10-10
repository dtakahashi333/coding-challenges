# python/tests/dsa/heaps/test_find_kth_largest.py

from unittest import TestCase
from dsa.heaps.find_kth_largest import Solution


class TestFindKthLargest(TestCase):
    def setUp(self):
        self.solution = Solution()

    def test_find_kth_largest1(self):
        nums = [3, 2, 1, 5, 6, 4]
        k = 2
        result = self.solution.findKthLargest(nums, k)
        self.assertEqual(result, 5)

    def test_find_kth_largest2(self):
        nums = [3, 2, 3, 1, 2, 4, 5, 5, 6]
        k = 4
        result = self.solution.findKthLargest(nums, k)
        self.assertEqual(result, 4)
