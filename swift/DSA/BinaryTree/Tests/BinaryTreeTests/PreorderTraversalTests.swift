//
//  PreorderTraversalTests.swift
//  BinaryTree
//
//  Created by Daisuke Takahashi on 9/8/25.
//

import Testing

@testable import BinaryTree

@Test func testPreorderTraversal1() async throws {
    let root = buildBinaryTree(items: [1, nil, 2, 3])
    let s = PreorderTraversal()
    let result = s.preorderTraversal(root)

    // Assertions using #expect
    #expect(result == [1, 2, 3])
}

@Test func testPreorderTraversal2() async throws {
    let root = buildBinaryTree(items: [1, 2, 3, 4, 5, nil, 8, nil, nil, 6, 7, 9])
    let s = PreorderTraversal()
    let result = s.preorderTraversal(root)

    // Assertions using #expect
    #expect(result == [1, 2, 4, 5, 6, 7, 3, 8, 9])
}

@Test func testPreorderTraversal3() async throws {
    let root = buildBinaryTree(items: [])
    let s = PreorderTraversal()
    let result = s.preorderTraversal(root)

    // Assertions using #expect
    #expect(result == [])
}

@Test func testPreorderTraversal4() async throws {
    let root = buildBinaryTree(items: [1])
    let s = PreorderTraversal()
    let result = s.preorderTraversal(root)

    // Assertions using #expect
    #expect(result == [1])
}
