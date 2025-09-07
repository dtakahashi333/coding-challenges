package dsa.binarytree

import kotlin.test.Test
import kotlin.test.assertEquals

class SolutionTest {

    @Test
    fun testInorderTraversal() {
        val items: List<Int?> = listOf(1, null, 2, 3)
        val root: TreeNode? = buildBinaryTree(items)
        val s = Solution()
        val result = s.inorderTraversal(root)
        assertEquals(result, listOf(1, 3, 2))
    }
}