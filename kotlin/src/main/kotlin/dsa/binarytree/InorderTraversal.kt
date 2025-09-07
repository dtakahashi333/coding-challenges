package dsa.binarytree

class Solution() {

    fun inorderTraversal(root: TreeNode?): MutableList<Int> {
        if (root == null) {
            return mutableListOf<Int>()
        }

        val result: MutableList<Int> = mutableListOf<Int>()
        var current: TreeNode? = root
        val stack: MutableList<TreeNode> = mutableListOf<TreeNode>()

        while (current != null || stack.count() > 0) {
            if (current != null) {
                if (current.left != null) {
                    stack.add(current)
                    current = current.left
                } else {
                    result.add(current.value)
                    current = current.right
                }
            } else if (stack.count() > 0) {
                current = stack.removeLast()
                result.add(current.value)
                current = current.right
            }
        }
        return result
    }

}