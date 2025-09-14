//
//  LeftView.swift
//  BinaryTree
//
//  Created by Daisuke Takahashi on 9/13/25.
//

class LeftView {

    public init() {}

    /// <#Description#>
    /// - Parameter root: <#root description#>
    /// - Returns: <#description#>
    public func leftView(_ root: TreeNode?) -> [Int] {
        guard let root else {
            return []
        }

        var queue: [TreeNode] = [root]
        var result: [Int] = []

        while !queue.isEmpty {
            let queueSize = queue.count
            for i in 0..<queueSize {
                let current = queue.removeFirst()
                if i == 0 {
                    result.append(current.val)
                }
                if let leftChild = current.left {
                    queue.append(leftChild)
                }
                if let rightChild = current.right {
                    queue.append(rightChild)
                }
            }
        }

        return result
    }
}
