//
//  Postordertraversal.swift
//  BinaryTree
//
//  Created by Daisuke Takahashi on 9/10/25.
//

class PostorderTraversal {

    public init() {}

    public func postorderTraversal(_ root: TreeNode?) -> [Int] {
        guard let root else {
            return []
        }
        
        var stack: [TreeNode] = [root]
        var result: [Int] = []

        while !stack.isEmpty {
            let current = stack.removeLast()
            result.append(current.val)
            if let leftChild = current.left {
                stack.append(leftChild)
            }
            if let rightChild = current.right {
                stack.append(rightChild)
            }
        }

        return result.reversed()
    }
}
