#!/usr/bin/env python3

# python/src/dsa/arrays/eval_rpn.py

# 150. Evaluate Reverse Polish Notation
# https://leetcode.com/problems/evaluate-reverse-polish-notation/description/

from typing import List


class Solution:
    def evalRPN(self, tokens: List[str]) -> int:
        stack = []
        for t in tokens:
            try:
                stack.append(int(t))
            except ValueError:
                operand2, operand1 = stack.pop(), stack.pop()
                match t:
                    case "+":
                        stack.append(operand1 + operand2)
                    case "-":
                        stack.append(operand1 - operand2)
                    case "*":
                        stack.append(operand1 * operand2)
                    case "/":
                        stack.append(int(operand1 / operand2))

        return stack.pop()


def main():
    tokens = ["10", "6", "9", "3", "+", "-11", "*", "/", "*", "17", "+", "5", "+"]
    # tokens = ["6", "-132", "/"]
    s = Solution()
    print(s.evalRPN(tokens))


if __name__ == "__main__":
    main()
