# python/tests/dsa/binary_tree/test_invert_tree.py

from unittest import TestCase

from dsa.binary_tree.invert_tree import Solution as S1
from dsa.binary_tree.bfs import Solution as S2
from dsa.binary_tree.tree_node import build_binary_tree


class TestInvertTree(TestCase):
    def setUp(self):
        self.solution1 = S1()
        self.solution2 = S2()
        return super().setUp()

    def test_invert_tree1(self):
        root = build_binary_tree([4, 2, 7, 1, 3, 6, 9])
        self.assertListEqual(
            self.solution2.bfs(self.solution1.invertTree(root)), [4, 7, 2, 9, 6, 3, 1]
        )

    def test_invert_tree2(self):
        root = build_binary_tree([2, 1, 3])
        self.assertListEqual(
            self.solution2.bfs(self.solution1.invertTree(root)), [2, 3, 1]
        )

    def test_invert_tree3(self):
        root = build_binary_tree([])
        self.assertListEqual(self.solution2.bfs(self.solution1.invertTree(root)), [])
