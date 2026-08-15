# python/tests/dsa/linked_list/test_lru_cache.py

from unittest import TestCase
from dsa.linked_list.lru_cache import LRUCache


class TestLRUCache(TestCase):
    def setUp(self):
        return super().setUp()

    def test_lru_cache1(self):
        """ "
        ["LRUCache", "put", "put", "get", "put", "get", "put", "get", "get", "get"]
        [[2], [1, 1], [2, 2], [1], [3, 3], [2], [4, 4], [1], [3], [4]]
        """
        lRUCache = LRUCache(2)
        lRUCache.put(1, 1)  # cache is {1=1}
        lRUCache.put(2, 2)  # cache is {1=1, 2=2}
        self.assertEqual(lRUCache.get(1), 1)  # return 1
        lRUCache.put(3, 3)  # LRU key was 2, evicts key 2, cache is {1=1, 3=3}
        self.assertEqual(lRUCache.get(2), -1)  # returns -1 (not found)
        lRUCache.put(4, 4)  # LRU key was 1, evicts key 1, cache is {4=4, 3=3}
        self.assertEqual(lRUCache.get(1), -1)  # return -1 (not found)
        self.assertEqual(lRUCache.get(3), 3)  # return 3
        self.assertEqual(lRUCache.get(4), 4)  # return 4
