#!/usr/bin/env python3

# python/tests/dsa/strings/test_group_anagrams.py

from unittest import TestCase
from dsa.strings.group_anagrams import Solution


class TestGroupAnagrams(TestCase):
    def test_group_anagrams1(self):
        strs = ["eat", "tea", "tan", "ate", "nat", "bat"]
        s = Solution()
        result = s.groupAnagrams(strs)
        for group in result:
            group.sort()
        self.assertListEqual(
            sorted(result, key=lambda x: x[0]),
            sorted(
                [["bat"], ["nat", "tan"], ["ate", "eat", "tea"]], key=lambda x: x[0]
            ),
        )
        result = s.groupAnagrams2(strs)
        for group in result:
            group.sort()
        self.assertListEqual(
            sorted(result, key=lambda x: x[0]),
            sorted(
                [["bat"], ["nat", "tan"], ["ate", "eat", "tea"]], key=lambda x: x[0]
            ),
        )

    def test_group_anagrams2(self):
        strs = [""]
        s = Solution()
        result = s.groupAnagrams(strs)
        for group in result:
            group.sort()
        self.assertListEqual(
            sorted(result, key=lambda x: x[0]),
            sorted([[""]], key=lambda x: x[0]),
        )
        result = s.groupAnagrams2(strs)
        for group in result:
            group.sort()
        self.assertListEqual(
            sorted(result, key=lambda x: x[0]),
            sorted([[""]], key=lambda x: x[0]),
        )

    def test_group_anagrams3(self):
        strs = ["a"]
        s = Solution()
        result = s.groupAnagrams(strs)
        for group in result:
            group.sort()
        self.assertListEqual(
            sorted(result, key=lambda x: x[0]),
            sorted([["a"]], key=lambda x: x[0]),
        )
        result = s.groupAnagrams2(strs)
        for group in result:
            group.sort()
        self.assertListEqual(
            sorted(result, key=lambda x: x[0]),
            sorted([["a"]], key=lambda x: x[0]),
        )
