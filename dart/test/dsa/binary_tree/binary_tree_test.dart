// test/binary_tree/binary_tree_test.dart

import 'package:test/test.dart';

import 'package:coding_challenges/dsa/binary_tree/binary_tree.dart';

void main() {
  group('Binary Tree', () {
    test('build binary tree', () {
      var list = [1, null, 2, 3];
      var tree = buildBinaryTree(list);

      // Assertions
      expect(tree, isNot(equals(null)));
      expect(tree!.val, equals(1));
      expect(tree.left, equals(null));
      expect(tree.right, isNot(equals(null)));
      expect(tree.right!.val, equals(2));
      expect(tree.right!.left, isNot(equals(null)));
      expect(tree.right!.left!.val, equals(3));
    });
  });
}
