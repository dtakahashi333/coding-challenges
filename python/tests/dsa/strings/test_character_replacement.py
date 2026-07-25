#!/usr/bin/env python3

# python/tests/dsa/strings/test_character_replacement.py

from unittest import TestCase
from dsa.strings.character_replacement import Solution


class TestCharacterReplacement(TestCase):
    def test_character_replacement1(self):
        s = "ABAB"
        k = 2
        sol = Solution()
        self.assertEqual(sol.characterReplacement(s, k), 4)
        self.assertEqual(sol.characterReplacement2(s, k), 4)

    def test_character_replacement2(self):
        s = "AABABBA"
        k = 1
        sol = Solution()
        self.assertEqual(sol.characterReplacement(s, k), 4)
        self.assertEqual(sol.characterReplacement2(s, k), 4)

    def test_character_replacement3(self):
        s = "ABAA"
        k = 0
        sol = Solution()
        self.assertEqual(sol.characterReplacement(s, k), 2)
        self.assertEqual(sol.characterReplacement2(s, k), 2)

    def test_character_replacement4(self):
        s = "ABBB"
        k = 2
        sol = Solution()
        self.assertEqual(sol.characterReplacement(s, k), 4)
        self.assertEqual(sol.characterReplacement2(s, k), 4)

    def test_character_replacement5(self):
        s = "BAAAB"
        k = 2
        sol = Solution()
        self.assertEqual(sol.characterReplacement(s, k), 5)
        self.assertEqual(sol.characterReplacement2(s, k), 5)

    def test_character_replacement6(self):
        s = "CADBBB"
        k = 4
        sol = Solution()
        self.assertEqual(sol.characterReplacement(s, k), 6)
        self.assertEqual(sol.characterReplacement2(s, k), 6)
