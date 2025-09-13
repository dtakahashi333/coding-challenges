//
//  PreorderTraversal.swift
//  BinaryTree
//
//  Created by Daisuke Takahashi on 9/8/25.
//

class PreorderTraversal {
    
    public init() {}
    
    public func preorderTraversal(_ root: TreeNode?) -> [Int] {
        var current: TreeNode? = root
        var stack: [TreeNode] = []
        var result: [Int] = []
        
        while current != nil || !stack.isEmpty {
            while let node = current {
                result.append(node.val)
                stack.append(node)
                current = node.left
            }

            // current = nil
            let node = stack.removeLast()
            current = node.right
        }
        
        return result
    }
}
