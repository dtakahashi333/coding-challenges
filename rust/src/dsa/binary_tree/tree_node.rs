use std::collections::VecDeque;

#[derive(Debug)]
pub struct TreeNode {
    pub value: i32,
    pub left: Option<Box<TreeNode>>,
    pub right: Option<Box<TreeNode>>,
}

impl TreeNode {
    /// Constructs a new TreeNode with the given value and no children.
    #[allow(dead_code)] // 👈 Keep this to silence the warning
    pub fn new(value: i32) -> Self {
        TreeNode {
            value,
            left: None,
            right: None,
        }
    }
}

pub fn build_binary_tree(list: &Vec<Option<i32>>) -> Option<TreeNode> {
    if list.is_empty() || list[0].is_none() {
        return None;
    }

    // let root = Box::new(TreeNode::new(list[0]));
    // let mut queue = vec![root.as_ref()];
    let mut root = TreeNode::new(list[0].unwrap());
    let mut queue: VecDeque<&mut TreeNode> = VecDeque::new();
    queue.push_back(&mut root);
    let mut i = 1;
    while i < list.len() {
        if let Some(front) = queue.pop_front() {
            match list[i] {
                Some(value) => {
                    front.left = Some(Box::new(TreeNode::new(value)));
                    if let Some(ref mut left_node) = front.left {
                        queue.push_back(left_node.as_mut());
                    }
                }
                None => {}
            }

            i = i + 1;

            if i < list.len() {
                match list[i] {
                    Some(value) => {
                        front.right = Some(Box::new(TreeNode::new(value)));
                        if let Some(ref mut right_node) = front.right {
                            queue.push_back(right_node.as_mut());
                        }
                    }
                    None => {}
                }

                i = i + 1;
            }
        }
    }
    Some(root)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let list = vec![Some(1), None, Some(2), Some(3)];
        let tree = build_binary_tree(&list);

        // Root exists
        assert!(tree.is_some());
        let tree = tree.unwrap();
        assert_eq!(tree.value, 1);

        // Left child is None
        assert!(tree.left.is_none());

        // Right child exists
        assert!(tree.right.is_some());
        let right = tree.right.as_ref().unwrap(); // borrow
        assert_eq!(right.value, 2);

        // Right's left child exists
        assert!(right.left.is_some());
        let right_left = right.left.as_ref().unwrap();
        assert_eq!(right_left.value, 3);

        // Right's right child is None
        assert!(right.right.is_none());
    }
}
