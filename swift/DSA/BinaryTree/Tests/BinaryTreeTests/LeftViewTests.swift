//
//  LeftViewTests.swift
//  BinaryTree
//
//  Created by Daisuke Takahashi on 9/13/25.
//

import Testing

@testable import BinaryTree

/// <#Description#>
/// - Throws: <#description#>
@Test func testLeftView1() async throws {
    let root = buildBinaryTree(items: [1, 2, 3, nil, 5, nil, 4])
    let s = LeftView()
    let result = s.leftView(root)

    // Assertions using #expect
    #expect(result == [1, 2, 5])
}

/// <#Description#>
/// - Throws: <#description#>
@Test func testLeftView2() async throws {
    let root = buildBinaryTree(items: [1, 2, 3, 6, 5, 8, 4])
    let s = LeftView()
    let result = s.leftView(root)

    // Assertions using #expect
    #expect(result == [1, 2, 6])
}
