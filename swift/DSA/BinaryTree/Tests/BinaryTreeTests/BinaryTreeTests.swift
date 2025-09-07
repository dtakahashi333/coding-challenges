import Testing
@testable import BinaryTree

@Test func testBuildBinaryTree1() async throws {
    let tree = buildBinaryTree(items: [1, nil, 2, 3])
    
    // Assertions using #expect
    #expect(tree != nil)
    #expect(tree!.val == 1)
    #expect(tree!.left == nil)
    #expect(tree!.right != nil)
    #expect(tree!.right!.val == 2)
    #expect(tree!.right!.left != nil)
    #expect(tree!.right!.left!.val == 3)
}