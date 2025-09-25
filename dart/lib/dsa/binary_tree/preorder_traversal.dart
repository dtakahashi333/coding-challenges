// lib/dsa/binary_tree/preorder_traversal.dart

import 'package:coding_challenges/dsa/binary_tree/binary_tree.dart';

class Solution {
  List<int> preorderTraversal(TreeNode? root) {
    if (root == null) {
      return [];
    }

    TreeNode? current = root;
    List<TreeNode> stack = [];
    List<int> result = [];

    while (current != null || stack.isNotEmpty) {
      while (current != null) {
        result.add(current.val);
        stack.add(current);
        current = current.left;
      }

      // current is null
      current = stack.removeLast();

      current = current.right;
    }

    return result;
  }
}
