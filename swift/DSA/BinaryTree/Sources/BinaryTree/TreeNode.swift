//
//  TreeNode.swift
//  TreeNode
//
//  Created by Daisuke Takahashi on 9/8/25.
//

class TreeNode {
    var val: Int
    var left: TreeNode?
    var right: TreeNode?

    init(val: Int = 0, left: TreeNode? = nil, right: TreeNode? = nil) {
        self.val = val
    }
}

func buildBinaryTree(items: [Int?]?) -> TreeNode? {
    guard let list = items, !list.isEmpty else {
        return nil
    }

    var root: TreeNode? = nil
    var queue: [TreeNode] = []
    if let value = list[0] {
        root = TreeNode(val: value)
        queue.append(root!)
    }

    var index = 1
    while queue.count > 0 {
        let current = queue.removeFirst()
        if index < list.count, let value = list[index] {
            current.left = TreeNode(val: value)
            queue.append(current.left!)
        }
        index += 1
        if index < list.count, let value = list[index] {
            current.right = TreeNode(val: value)
            queue.append(current.right!)
        }
        index += 1
    }
    return root
}
