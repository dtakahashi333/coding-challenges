#!/bin/env python3

# python/src/dsa/recursion/exist.py

# 79. Word Search
# https://leetcode.com/problems/word-search/description/

from typing import List


class Solution:
    def exist(self, board: List[List[str]], word: str) -> bool:
        rows = len(board)
        cols = len(board[0])
        visited = [[False] * cols for _ in range(rows)]

        for i in range(rows):
            for j in range(cols):
                if self.exist_helper(board, word, i, j, 0, visited):
                    return True

        return False

    def exist_helper(
        self,
        board: List[List[str]],
        word: str,
        i: int,
        j: int,
        word_index: int,
        visited: List[List[bool]],
    ):
        if word[word_index] != board[i][j]:
            return False

        if word_index == len(word) - 1:
            return True

        visited[i][j] = True
        result = False

        for dr, dc in [(0, -1), (0, 1), (-1, 0), (1, 0)]:
            nr, nc = i + dr, j + dc

            if 0 <= nr < len(board) and 0 <= nc < len(board[0]) and not visited[nr][nc]:
                result = self.exist_helper(board, word, nr, nc, word_index + 1, visited)

            if result:
                return True

        visited[i][j] = False

        return result
