# python/src/dsa/linked_list/lru_cache.py

# 146. LRU Cache
# https://leetcode.com/problems/lru-cache/description/

from typing import Optional


# Definition for doubly-linked list.
class ListNode:
    def __init__(
        self,
        key: int = 0,
        val: int = 0,
        prev: Optional[ListNode] = None,
        next: Optional[ListNode] = None,
    ):
        self.key = key
        self.val = val
        self.next = next
        self.prev = prev


class LRUCache:
    def __init__(self, capacity: int):
        self.capacity = capacity
        self.hash_map = {}
        self.lru_sentinel = ListNode()
        self.mru_sentinel = ListNode(prev=self.lru_sentinel)
        self.lru_sentinel.next = self.mru_sentinel

    def get(self, key: int) -> int:
        if key in self.hash_map:
            node = self.hash_map[key]
            value = node.val
            if node != self.mru_sentinel.prev:
                self.remove(node)
                self.insert_at_mru(node)
            return value
        else:
            return -1

    def put(self, key: int, value: int) -> None:
        if key in self.hash_map:
            node = self.hash_map[key]
            node.val = value
            if node != self.mru_sentinel.prev:
                self.remove(node)
                self.insert_at_mru(node)
        else:
            node = ListNode(key=key, val=value)
            self.insert_at_mru(node)
        self.hash_map[key] = node
        if len(self.hash_map) > self.capacity:
            self.remove_at_lru(self.lru_sentinel.next)

    def remove(self, node: ListNode):
        node.prev.next = node.next
        node.next.prev = node.prev

    def insert_at_mru(self, node: ListNode):
        self.mru_sentinel.prev.next = node
        node.prev = self.mru_sentinel.prev
        self.mru_sentinel.prev = node
        node.next = self.mru_sentinel

    def remove_at_lru(self, node: ListNode):
        self.lru_sentinel.next = node.next
        node.next.prev = self.lru_sentinel
        del self.hash_map[node.key]
        node = None


def main():
    lRUCache = LRUCache(2)
    lRUCache.put(1, 1)  # cache is {1=1}
    lRUCache.put(2, 2)  # cache is {1=1, 2=2}
    print(lRUCache.get(1))  # return 1
    lRUCache.put(3, 3)  # LRU key was 2, evicts key 2, cache is {1=1, 3=3}
    print(lRUCache.get(2))  # returns -1 (not found)
    lRUCache.put(4, 4)  # LRU key was 1, evicts key 1, cache is {4=4, 3=3}
    print(lRUCache.get(1))  # return -1 (not found)
    print(lRUCache.get(3))  # return 3
    print(lRUCache.get(4))  # return 4


if __name__ == "__main__":
    main()
