// test/binary_tree/inorder_traversal_test.dart

import 'package:test/test.dart';

import 'package:coding_challenges/dsa/binary_tree/tree_node.dart';
import 'package:coding_challenges/dsa/binary_tree/preorder_traversal.dart';

void main() {
  group('Preorder Traversal', () {
    test('[1,null,2,3]', () {
      var list = <int?>[1, null, 2, 3];
      var tree = buildBinaryTree(list);

      var s = Solution();
      var result = s.preorderTraversal(tree);

      expect(result, equals([1, 2, 3]));
    });

    test('[1,2,3,4,5,null,8,null,null,6,7,9]', () {
      var list = <int?>[1, 2, 3, 4, 5, null, 8, null, null, 6, 7, 9];
      var tree = buildBinaryTree(list);

      var s = Solution();
      var result = s.preorderTraversal(tree);

      expect(result, equals([1, 2, 4, 5, 6, 7, 3, 8, 9]));
    });

    test('[]', () {
      var list = <int?>[];
      var tree = buildBinaryTree(list);

      var s = Solution();
      var result = s.preorderTraversal(tree);

      expect(result, equals([]));
    });

    test('[1]', () {
      var list = <int?>[1];
      var tree = buildBinaryTree(list);

      var s = Solution();
      var result = s.preorderTraversal(tree);

      expect(result, equals([1]));
    });
  });
}
