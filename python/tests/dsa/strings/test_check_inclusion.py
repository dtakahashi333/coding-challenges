#!/usr/bin/env python3

# python/tests/dsa/strings/test_check_inclusion.py

from unittest import TestCase
from dsa.strings.check_inclusion import Solution


class TestCheckInclusion(TestCase):
    def test_check_inclusion1(self) -> bool:
        s1 = "ab"
        s2 = "eidbaooo"
        s = Solution()
        self.assertTrue(s.checkInclusion(s1, s2))
        self.assertTrue(s.checkInclusion2(s1, s2))

    def test_check_inclusion2(self) -> bool:
        s1 = "ab"
        s2 = "eidboaoo"
        s = Solution()
        self.assertFalse(s.checkInclusion(s1, s2))
        self.assertFalse(s.checkInclusion2(s1, s2))

    def test_check_inclusion3(self) -> bool:
        s1 = "adc"
        s2 = "dcda"
        s = Solution()
        self.assertTrue(s.checkInclusion(s1, s2))
        self.assertTrue(s.checkInclusion2(s1, s2))
