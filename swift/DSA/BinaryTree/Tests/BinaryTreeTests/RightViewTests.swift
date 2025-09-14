//
//  RightViewTests.swift
//  BinaryTree
//
//  Created by Daisuke Takahashi on 9/13/25.
//

import Testing

@testable import BinaryTree

/// <#Description#>
/// - Throws: <#description#>
@Test func testRightView1() async throws {
    let root = buildBinaryTree(items: [1, 2, 3, nil, 5, nil, 4])
    let s = RightView()
    let result = s.rightView(root)

    // Assertions using #expect
    #expect(result == [1, 3, 4])
}

/// <#Description#>
/// - Throws: <#description#>
@Test func testRightView2() async throws {
    let root = buildBinaryTree(items: [1, 2, 3, 6, 5, 8, 4])
    let s = RightView()
    let result = s.rightView(root)

    // Assertions using #expect
    #expect(result == [1, 3, 4])
}
