use std::time::Instant;

use rayon_crate_video::tree_helpers::{generate_full_tree, TreeNode};

fn invert_tree(node: &mut Option<Box<TreeNode>>, depth: u8) {
    if let Some(n) = node {
        if depth < 4 {
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

fn main() {
    let depth = 23;
    let mut large_tree = generate_full_tree(depth);

    let start = Instant::now();
    invert_tree(&mut large_tree, 1);
    dbg!(start.elapsed());
}
