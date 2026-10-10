# python/tests/dsa/heaps/test_k_closest.py

from unittest import TestCase
from dsa.heaps.k_closest import Solution


class TestKClosest(TestCase):
    def setUp(self):
        self.solution = Solution()

    def test_k_closest1(self):
        points = [[1, 3], [-2, 2]]
        k = 1
        result = self.solution.kClosest(points, k)
        self.assertListEqual(sorted(result), sorted([[-2, 2]]))

    def test_k_closest2(self):
        points = [[3, 3], [5, -1], [-2, 4]]
        k = 2
        result = self.solution.kClosest(points, k)
        self.assertListEqual(sorted(result), sorted([[3, 3], [-2, 4]]))
