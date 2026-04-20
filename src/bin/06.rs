use std::time::Instant;

use rayon_crate_video::tree_helpers::{generate_full_tree, TreeNode};

fn invert_tree(node: &mut Option<Box<TreeNode>>) {
    if let Some(n) = node {
        invert_tree(&mut n.left);
        invert_tree(&mut n.right);

        std::mem::swap(&mut n.left, &mut n.right);
    }
}

fn main() {
    let depth = 23;
    let mut large_tree = generate_full_tree(depth);

    let start = Instant::now();
    invert_tree(&mut large_tree);
    dbg!(start.elapsed());
}
