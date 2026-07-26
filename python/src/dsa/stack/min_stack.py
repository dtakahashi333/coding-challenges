#!/usr/bin/env python3

# python/src/dsa/stack/min_stack.py

# 155. Min Stack
# https://leetcode.com/problems/min-stack/description/

import sys


class MinStack:
    def __init__(self):
        self.stack: list[int] = []
        self.min_stack: list[int] = []

    def push(self, value: int) -> None:
        self.stack.append(value)
        if not self.min_stack:
            self.min_stack.append(value)
        elif self.min_stack[-1] >= value:
            self.min_stack.append(value)

    def pop(self) -> None:
        value = self.stack.pop()
        if self.min_stack[-1] == value:
            self.min_stack.pop()

    def top(self) -> int:
        return self.stack[-1]

    def getMin(self) -> int:
        return self.min_stack[-1]
