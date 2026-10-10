# python/tests/dsa/heaps/test_kth_largest.py

from unittest import TestCase
from dsa.heaps.kth_largest import KthLargest


class TestKthLargest(TestCase):

    def test_kth_largest1(self):
        k = 3
        nums = [4, 5, 8, 2]
        kth_largest = KthLargest(k, nums)
        self.assertEqual(kth_largest.add(3), 4)
        self.assertEqual(kth_largest.add(5), 5)
        self.assertEqual(kth_largest.add(10), 5)
        self.assertEqual(kth_largest.add(9), 8)
        self.assertEqual(kth_largest.add(4), 8)

    def test_kth_largest2(self):
        k = 4
        nums = [7, 7, 7, 7, 8, 3]
        kth_largest = KthLargest(k, nums)
        self.assertEqual(kth_largest.add(2), 7)
        self.assertEqual(kth_largest.add(10), 7)
        self.assertEqual(kth_largest.add(9), 7)
        self.assertEqual(kth_largest.add(9), 8)
