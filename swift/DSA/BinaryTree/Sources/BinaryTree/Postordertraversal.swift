//
//  Postordertraversal.swift
//  BinaryTree
//
//  Created by Daisuke Takahashi on 9/10/25.
//

import BinaryTree

class PostorderTraversal {

    public init() {}

    public func postorderTraversal(_ root: TreeNode?) -> [Int] {
        var current: TreeNode? = root
        var stack: [TreeNode] = []
        var result: [Int] = []

        while current != nil || !stack.isEmpty {
            while let node = current {
                stack.append(node)
                current = node.left
                node.left = nil
            }

            // current = nil
            while !stack.isEmpty && stack.last?.right == nil {
                result.append(stack.removeLast().val)
            }

            guard let lastChild = stack.last else {
                break
            }

            current = lastChild.right
            lastChild.right = nil
        }

        return result
    }
}
