// lib/dsa/binary_tree/inorder_traversal.dart

import 'package:coding_challenges/dsa/binary_tree/binary_tree.dart';

class Solution {
  List<int> inorderTraversal(TreeNode? root) {
    if (root == null) {
      return [];
    }

    TreeNode? current = root;
    List<TreeNode> stack = [];
    List<int> result = [];

    while (current != null || stack.isNotEmpty) {
      while (current != null) {
        stack.add(current);
        current = current.left;
      }

      // current is null
      current = stack.removeLast();
      result.add(current.val);

      current = current.right;
    }

    return result;
  }
}
