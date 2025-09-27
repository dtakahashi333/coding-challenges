package dsa.binarytree

import kotlin.test.Test
import kotlin.test.assertEquals

class SolutionTest {

    @Test
    fun testInorderTraversal1() {
        val items: List<Int?> = listOf(1, null, 2, 3)
        val root: TreeNode? = buildBinaryTree(items)
        val s = Solution()
        val result = s.inorderTraversal(root)
        assertEquals(result, listOf(1, 3, 2))
    }

    @Test
    fun testInorderTraversal2() {
        val items: List<Int?> = listOf(1, 2, 3, 4, 5, null, 8, null, null, 6, 7, 9)
        val root: TreeNode? = buildBinaryTree(items)
        val s = Solution()
        val result = s.inorderTraversal(root)
        assertEquals(result, listOf(4, 2, 6, 5, 7, 1, 3, 9, 8))
    }

    @Test
    fun testInorderTraversal3() {
        val items: List<Int?> = emptyList()
        val root: TreeNode? = buildBinaryTree(items)
        val s = Solution()
        val result = s.inorderTraversal(root)
        assertEquals(result, emptyList())
    }

    @Test
    fun testInorderTraversal4() {
        val items: List<Int?> = listOf(1)
        val root: TreeNode? = buildBinaryTree(items)
        val s = Solution()
        val result = s.inorderTraversal(root)
        assertEquals(result, listOf(1))
    }
}