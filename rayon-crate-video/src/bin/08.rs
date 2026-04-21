use std::time::Instant;

use rayon_crate_video::tree_helpers::{generate_full_tree, TreeNode};

fn invert_tree(node: &mut Option<Box<TreeNode>>, depth: u8) {
    if let Some(n) = node {
        if depth < 6 {
            rayon::join(
                || invert_tree(&mut n.left, depth + 1),
                || invert_tree(&mut n.right, depth + 1),
            );
        } else {
            invert_tree(&mut n.left, depth + 1);
            invert_tree(&mut n.right, depth + 1);
        }

        std::mem::swap(&mut n.left, &mut n.right);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rayon_crate_video::tree_helpers::TreeNode;

    #[test]
    fn test_invert_single_node() {
        let mut tree = Some(Box::new(TreeNode::new(42)));
        let original_data = tree.as_ref().unwrap().data;

        invert_tree(&mut tree, 1);

        assert!(tree.is_some());
        assert_eq!(tree.as_ref().unwrap().data, original_data);
        assert!(tree.as_ref().unwrap().left.is_none());
        assert!(tree.as_ref().unwrap().right.is_none());
    }

    #[test]
    fn test_invert_two_level_tree() {
        // Create tree: 1 -> left(2), right(3)
        let mut tree = Some(Box::new(TreeNode::with_children(
            1,
            Box::new(TreeNode::new(2)),
            Box::new(TreeNode::new(3)),
        )));

        invert_tree(&mut tree, 1);

        // After inversion: 1 -> left(3), right(2)
        assert_eq!(tree.as_ref().unwrap().data, 1);
        assert_eq!(tree.as_ref().unwrap().left.as_ref().unwrap().data, 3);
        assert_eq!(tree.as_ref().unwrap().right.as_ref().unwrap().data, 2);
    }

    #[test]
    fn test_invert_three_level_tree() {
        // Create tree:
        //       1
        //      / \
        //     2   3
        //    / \
        //   4   5
        let mut tree = Some(Box::new(TreeNode::with_children(
            1,
            Box::new(TreeNode::with_children(
                2,
                Box::new(TreeNode::new(4)),
                Box::new(TreeNode::new(5)),
            )),
            Box::new(TreeNode::new(3)),
        )));

        invert_tree(&mut tree, 1);

        // After inversion:
        //       1
        //      / \
        //     3   2
        //        / \
        //       5   4
        let root = tree.as_ref().unwrap();
        assert_eq!(root.data, 1);
        assert_eq!(root.left.as_ref().unwrap().data, 3);
        assert_eq!(root.right.as_ref().unwrap().data, 2);

        let right_node = root.right.as_ref().unwrap();
        assert_eq!(right_node.left.as_ref().unwrap().data, 5);
        assert_eq!(right_node.right.as_ref().unwrap().data, 4);
    }

    #[test]
    fn test_invert_asymmetric_tree() {
        // Create asymmetric tree:
        //       1
        //      / \
        //     2   3
        //    /     \
        //   4       5
        let mut tree = Some(Box::new(TreeNode::with_children(
            1,
            Box::new(TreeNode::with_children(
                2,
                Box::new(TreeNode::new(4)),
                Box::new(TreeNode::new(3)),
            )),
            Box::new(TreeNode::new(5)),
        )));

        invert_tree(&mut tree, 1);

        // After inversion:
        //       1
        //      / \
        //     5   2
        //        / \
        //       3   4
        let root = tree.as_ref().unwrap();
        assert_eq!(root.data, 1);
        assert_eq!(root.left.as_ref().unwrap().data, 5);
        assert_eq!(root.right.as_ref().unwrap().data, 2);

        let right_node = root.right.as_ref().unwrap();
        assert_eq!(right_node.left.as_ref().unwrap().data, 3);
        assert_eq!(right_node.right.as_ref().unwrap().data, 4);
    }

    #[test]
    fn test_invert_none_tree() {
        let mut tree: Option<Box<TreeNode>> = None;
        invert_tree(&mut tree, 1);
        assert!(tree.is_none());
    }

    #[test]
    fn test_invert_small_generated_tree() {
        // Test with depth 3 tree (should have 7 nodes)
        let mut tree = generate_full_tree(3);
        let original_root_data = tree.as_ref().unwrap().data;
        let original_left_data = tree.as_ref().unwrap().left.as_ref().unwrap().data;
        let original_right_data = tree.as_ref().unwrap().right.as_ref().unwrap().data;

        invert_tree(&mut tree, 1);

        // Root data should stay the same
        assert_eq!(tree.as_ref().unwrap().data, original_root_data);
        // Left and right should be swapped
        assert_eq!(tree.as_ref().unwrap().left.as_ref().unwrap().data, original_right_data);
        assert_eq!(tree.as_ref().unwrap().right.as_ref().unwrap().data, original_left_data);
    }
}

fn main() {
    let depth = 23;
    let mut large_tree = generate_full_tree(depth);

    let start = Instant::now();
    invert_tree(&mut large_tree, 1);
    dbg!(start.elapsed());
}
