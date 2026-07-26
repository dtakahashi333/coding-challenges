#!/usr/bin/env python3

# python/tests/dsa/stack/test_min_stack.py

from unittest import TestCase
from dsa.stack.min_stack import MinStack


class TestMinStack(TestCase):
    def test_min_stack1(self):
        st = MinStack()
        st.push(-2)
        st.push(0)
        st.push(-3)
        self.assertEqual(st.getMin(), -3)  # return -3
        st.pop()
        self.assertEqual(st.top(), 0)  # return 0
        self.assertEqual(st.getMin(), -2)  # return -2
