#!/bin/env python3

# python/tests/dsa/greedy_algorithms/test_find_content_children.py

from unittest import TestCase
from dsa.greedy_algorithms.find_content_children import Solution


class TestFindContentChildren(TestCase):
    def test_find_content_children1(self):
        g = [1, 2, 3]
        s = [1, 1]
        sol = Solution()
        result = sol.findContentChildren(g, s)
        self.assertEqual(result, 1)

    def test_find_content_children2(self):
        g = [1,2]
        s = [1,2,3]
        sol = Solution()
        result = sol.findContentChildren(g, s)
        self.assertEqual(result, 2)
