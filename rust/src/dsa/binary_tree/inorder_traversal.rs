// rust/src/dsa/binary_tree/inorder_traversal.rs

use super::tree_node;

#[derive(Debug)]
pub struct Solution;

impl Solution {
    pub fn inorder_traversal_recursive(&self, root: &Option<&tree_node::TreeNode>) -> Vec<i32> {
        let mut result: Vec<i32> = Vec::new();
        self.inorder_traversal_recursive_helper(root, &mut result);
        result
    }

    fn inorder_traversal_recursive_helper(
        &self,
        node: &Option<&tree_node::TreeNode>,
        result: &mut Vec<i32>,
    ) {
        if let Some(n) = node {
            // left
            self.inorder_traversal_recursive_helper(&n.left.as_deref(), result);
            // current value;
            result.push(n.value);
            // right
            self.inorder_traversal_recursive_helper(&n.right.as_deref(), result);
        }
    }

    pub fn inorder_traversal(&self, root: &Option<&tree_node::TreeNode>) -> Vec<i32> {
        let mut result = Vec::new();
        let mut stack = Vec::new();

        // Start from the root (Option<&TreeNode>)
        let mut current = *root;

        while current.is_some() || !stack.is_empty() {
            // 1. Walk left as far as possible
            while let Some(node) = current {
                stack.push(node);
                current = node.left.as_deref();
            }

            // 2. Pop from stack → visit node
            let node = stack.pop().unwrap();
            result.push(node.value);

            // 3. Then visit the right subtree
            current = node.right.as_deref();
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dsa::binary_tree::tree_node;

    #[test]
    fn test1() {
        let list = vec![Some(1), None, Some(2), Some(3)];
        let tree = tree_node::build_binary_tree(&list);
        let s = Solution;
        let result = s.inorder_traversal_recursive(&tree.as_ref());
        assert_eq!(result, vec![1, 3, 2]);
    }

    #[test]
    fn test2() {
        let list = vec![Some(1), None, Some(2), Some(3)];
        let tree = tree_node::build_binary_tree(&list);
        let s = Solution;
        let result = s.inorder_traversal(&tree.as_ref());
        assert_eq!(result, vec![1, 3, 2]);
    }
}
