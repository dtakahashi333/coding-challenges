// test/binary_tree/inorder_traversal_test.dart

import 'package:test/test.dart';

import 'package:coding_challenges/dsa/binary_tree/binary_tree.dart';
import 'package:coding_challenges/dsa/binary_tree/inorder_traversal.dart';

void main() {
  group('Inorder Traversal', () {
    test('[1,null,2,3]', () {
      var list = <int?>[1, null, 2, 3];
      var tree = buildBinaryTree(list);

      var s = Solution();
      var result = s.inorderTraversal(tree);

      expect(result, equals([1, 3, 2]));
    });

    test('[1,2,3,4,5,null,8,null,null,6,7,9]', () {
      var list = <int?>[1, 2, 3, 4, 5, null, 8, null, null, 6, 7, 9];
      var tree = buildBinaryTree(list);

      var s = Solution();
      var result = s.inorderTraversal(tree);

      expect(result, equals([4, 2, 6, 5, 7, 1, 3, 9, 8]));
    });

    test('[]', () {
      var list = <int?>[];
      var tree = buildBinaryTree(list);

      var s = Solution();
      var result = s.inorderTraversal(tree);

      expect(result, equals([]));
    });

    test('[1]', () {
      var list = <int?>[1];
      var tree = buildBinaryTree(list);

      var s = Solution();
      var result = s.inorderTraversal(tree);

      expect(result, equals([1]));
    });
  });
}
