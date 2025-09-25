import 'package:coding_challenges/dsa/binary_tree/binary_tree.dart';

class Solution {
  List<int> inorderTraversal(TreeNode? root) {
    if (root == null) {
      return [];
    }

    TreeNode? current = root;
    List<TreeNode> stack = [current];
    List<int> result = [];

    while (current != null || stack.isNotEmpty) {
      while (current != null) {
        if (current.left != null) {
          stack.add(current.left!);
        }
        current = current.left;
      }

      // current is null
      current = stack.removeLast();
      result.add(current.val);

      current = current.right;

      if (current != null) {
        stack.add(current);
      }
    }

    return result;
  }
}
