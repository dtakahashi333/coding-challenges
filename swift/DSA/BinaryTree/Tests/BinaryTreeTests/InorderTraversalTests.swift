import Testing
@testable import BinaryTree

@Test func testInorderTraversal1() async throws {
    let root = buildBinaryTree(items: [1,nil,2,3])
    let s = Solution()
    let result = s.inorderTraversal(root)
}