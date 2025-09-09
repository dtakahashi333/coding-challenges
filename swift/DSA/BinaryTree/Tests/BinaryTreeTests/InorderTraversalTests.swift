//
//  InorderTraversalTests.swift
//  BinaryTree
//
//  Created by Daisuke Takahashi on 9/8/25.
//

import Testing

@testable import BinaryTree

@Test func testInorderTraversal1() async throws {
    let root = buildBinaryTree(items: [1, nil, 2, 3])
    let s = InorderTraversal()
    let result = s.inorderTraversal(root)

    // Assertions using #expect
    #expect(result == [1, 3, 2])
}

@Test func testInorderTraversal2() async throws {
    let root = buildBinaryTree(items: [1, 2, 3, 4, 5, nil, 8, nil, nil, 6, 7, 9])
    let s = InorderTraversal()
    let result = s.inorderTraversal(root)

    // Assertions using #expect
    #expect(result == [4, 2, 6, 5, 7, 1, 3, 9, 8])
}

@Test func testInorderTraversal3() async throws {
    let root = buildBinaryTree(items: [])
    let s = InorderTraversal()
    let result = s.inorderTraversal(root)

    // Assertions using #expect
    #expect(result == [])
}

@Test func testInorderTraversal4() async throws {
    let root = buildBinaryTree(items: [1])
    let s = InorderTraversal()
    let result = s.inorderTraversal(root)

    // Assertions using #expect
    #expect(result == [1])
}
