//
//  PostorderTraversalTests.swift
//  BinaryTree
//
//  Created by Daisuke Takahashi on 9/10/25.
//

import Testing

@testable import BinaryTree

@Test func testPostorderTraversal1() async throws {
    let root = buildBinaryTree(items: [1, nil, 2, 3])
    let s = PostorderTraversal()
    let result = s.postorderTraversal(root)

    // Assertions using #expect
    #expect(result == [3, 2, 1])
}

@Test func testPostorderTraversal2() async throws {
    let root = buildBinaryTree(items: [1, 2, 3, 4, 5, nil, 8, nil, nil, 6, 7, 9])
    let s = PostorderTraversal()
    let result = s.postorderTraversal(root)

    // Assertions using #expect
    #expect(result == [4, 6, 7, 5, 2, 9, 8, 3, 1])
}

@Test func testPostorderTraversal3() async throws {
    let root = buildBinaryTree(items: [])
    let s = PostorderTraversal()
    let result = s.postorderTraversal(root)

    // Assertions using #expect
    #expect(result == [])
}

@Test func testPostorderTraversal4() async throws {
    let root = buildBinaryTree(items: [1])
    let s = PostorderTraversal()
    let result = s.postorderTraversal(root)

    // Assertions using #expect
    #expect(result == [1])
}
