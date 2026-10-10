# python/tests/dsa/heaps/test_median_finder.py


from unittest import TestCase
from dsa.heaps.median_finder import MedianFinder


class TestMedianFinder(TestCase):

    def test_median_finder1(self):
        median_finder = MedianFinder()
        median_finder.addNum(1)
        median_finder.addNum(2)
        self.assertEqual(median_finder.findMedian(), 1.5)
        median_finder.addNum(3)
        self.assertEqual(median_finder.findMedian(), 2.0)

    def test_median_finder2(self):
        # [[],[-1],[],[-2],[],[-3],[],[-4],[],[-5],[]]
        median_finder = MedianFinder()
        median_finder.addNum(-1)
        self.assertEqual(median_finder.findMedian(), -1)
        median_finder.addNum(-2)
        self.assertEqual(median_finder.findMedian(), -1.5)
        median_finder.addNum(-3)
        self.assertEqual(median_finder.findMedian(), -2.0)
        median_finder.addNum(-4)
        self.assertEqual(median_finder.findMedian(), -2.5)
        median_finder.addNum(-5)
        self.assertEqual(median_finder.findMedian(), -3.0)

    def test_median_finder3(self):
        median_finder = MedianFinder()
        median_finder.addNum(1)
        median_finder.addNum(5)
        median_finder.addNum(2)
        median_finder.addNum(0)
        self.assertEqual(median_finder.findMedian(), 1.5)
        median_finder.addNum(-1)
        self.assertEqual(median_finder.findMedian(), 1)
