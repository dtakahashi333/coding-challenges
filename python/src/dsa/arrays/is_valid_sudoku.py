#!/usr/bin/env python3

# python/src/dsa/arrays/is_valid_sudoku.py

# 36. Valid Sudoku
# https://leetcode.com/problems/valid-sudoku/description/

from typing import List


class Solution:
    def isValidSudoku(self, board: List[List[str]]) -> bool:
        # row validation
        for i in range(9):
            seen = [False] * 10
            for j in range(9):
                if board[i][j] != ".":
                    num = int(board[i][j])
                    if seen[num]:
                        return False
                    seen[num] = True
        # column validation
        for j in range(9):
            seen = [False] * 10
            for i in range(9):
                if board[i][j] != ".":
                    num = int(board[i][j])
                    if seen[num]:
                        return False
                    seen[num] = True
        # 3x3 sub-box validation
        for i in range(0, 9, 3):
            for j in range(0, 9, 3):
                seen = [False] * 10
                for ii in range(i, i + 3, 1):
                    for jj in range(j, j + 3, 1):
                        if board[ii][jj] != ".":
                            num = int(board[ii][jj])
                            if seen[num]:
                                return False
                            seen[num] = True

        return True

    def isValidSudoku2(self, board: List[List[str]]) -> bool:
        rows = [set() for _ in range(9)]
        cols = [set() for _ in range(9)]
        subboxes = [set() for _ in range(9)]
        for i in range(9):
            for j in range(9):
                num = board[i][j]
                if num != ".":
                    subbox = i // 3 * 3 + j // 3
                    if num in rows[i] or num in cols[j] or num in subboxes[subbox]:
                        return False
                    rows[i].add(num)
                    cols[j].add(num)
                    subboxes[subbox].add(num)

        return True
