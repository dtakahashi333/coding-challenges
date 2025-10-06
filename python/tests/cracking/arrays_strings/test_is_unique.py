#!/bin/env python3

from unittest import TestCase
from cracking.arrays_strings.is_unique import Solution


class TestInUnique(TestCase):

    def test_is_unique1(self):
        input = "abcdef"
        s = Solution()
        self.assertEqual(s.isUnique(input), True)

    def test_is_unique2(self):
        input = "hello"
        s = Solution()
        self.assertEqual(s.isUnique(input), False)

    def test_is_unique3(self):
        input = ""
        s = Solution()
        self.assertEqual(s.isUnique(input), True)

    def test_is_unique4(self):
        input = "AaBbCc"
        s = Solution()
        self.assertEqual(s.isUnique(input), True)

    def test_is_unique5(self):
        input = "112233"
        s = Solution()
        self.assertEqual(s.isUnique(input), False)

    def test_is_unique6(self):
        input = "python3.9"
        s = Solution()
        self.assertEqual(s.isUnique(input), True)

    def test_is_unique7(self):
        input = "!@#$%^&*()"
        s = Solution()
        self.assertEqual(s.isUnique(input), True)

    def test_is_unique8(self):
        input = "😊🌟🍕🚀"
        s = Solution()
        self.assertEqual(s.isUnique(input), True)

    def test_is_unique9(self):
        input = "abcABCabc"
        s = Solution()
        self.assertEqual(s.isUnique(input), False)

    def test_is_unique10(self):
        input = "the quick brown fox"
        s = Solution()
        self.assertEqual(s.isUnique(input), False)
