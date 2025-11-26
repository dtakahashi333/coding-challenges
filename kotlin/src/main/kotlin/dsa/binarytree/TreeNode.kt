package dsa.binarytree

class TreeNode(var value: Int = 0, var left: TreeNode? = null, var right: TreeNode? = null) {
}

fun buildBinaryTree(items: List<Int?>?): TreeNode? {
    if (items == null || items.isEmpty() || items[0] == null) {
        return null
    }
    val root: TreeNode = TreeNode(items[0]!!)
    val queue: MutableList<TreeNode> = mutableListOf(root)
    var index = 1
    while (queue.isNotEmpty()) {
        val current = queue.removeFirst()
        if (index < items.size && items[index] != null) {
            current.left = TreeNode(items[index]!!)
            queue.add(current.left!!)
        }
        index += 1
        if (index < items.size && items[index] != null) {
            current.right = TreeNode(items[index]!!)
            queue.add(current.right!!)
        }
        index += 1
    }
    return root
}
