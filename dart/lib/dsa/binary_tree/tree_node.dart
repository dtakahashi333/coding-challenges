//
//  binary_tree.dart
//  BinaryTree
//
//  Created by Daisuke Takahashi on 2025-09-24.
//

/// This file contains the TreeNode class and binary tree implementation.

/// Represents a node in a binary tree.
///
/// Each node contains an integer value [val] and optional left and right child nodes.
class TreeNode {
  /// The value stored in the node.
  ///
  /// Defaults to 0 if not specified.
  int val;

  /// The left child of this node.
  ///
  /// Can be `null` if there is no left child.
  TreeNode? left;

  /// The right child of this node.
  ///
  /// Can be `null` if there is no right child.
  TreeNode? right;

  /// Creates a [TreeNode] with the given [val], [left], and [right].
  ///
  /// If [val] is not provided, it defaults to 0.
  /// [left] and [right] are optional child nodes and default to `null`.
  TreeNode({this.val = 0, this.left, this.right});
}

TreeNode? buildBinaryTree(List<int?>? list) {
  if (list == null || list.isEmpty || list[0] == null) {
    return null;
  }

  var root = TreeNode(val: list[0]!);
  var queue = [root];
  var index = 1;

  while (index < list.length && queue.isNotEmpty) {
    var current = queue.removeAt(0);
    if (list[index] != null) {
      var child = TreeNode(val: list[index]!);
      current.left = child;
      queue.add(child);
    }

    index++;

    if (index >= list.length) {
      break;
    }

    if (list[index] != null) {
      var child = TreeNode(val: list[index]!);
      current.right = child;
      queue.add(child);
    }

    index++;
  }

  return root;
}
