//
//  InorderTraversal.swift
//  BinaryTree
//
//  Created by Daisuke Takahashi on 9/8/25.
//

class InorderTraversal {

    public init() {}

    public func inorderTraversal(_ root: TreeNode?) -> [Int] {
        var current: TreeNode? = root
        var stack: [TreeNode] = []
        var result: [Int] = []

        while current != nil || !stack.isEmpty {
            // Reach the leftmost node of the current node
            while let node = current {  // This is the same as if current != nill {
                stack.append(node)
                current = node.left
            }

            // Current is now nil, pop from stack
            let node = stack.removeLast()
            result.append(node.val)

            // Visit the right subtree
            current = node.right
        }

        return result
    }
}
