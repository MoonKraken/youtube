use std::{pin::Pin, time::Instant};

use rayon_crate_video::tree_helpers::{generate_full_tree, TreeNode};

fn invert_tree(
    node: Option<Box<TreeNode>>,
    depth: u8,
) -> Pin<Box<dyn Future<Output = Option<Box<TreeNode>>> + Send + 'static>> {
    Box::pin(async move {
        // Use `mut n` so we can modify the fields
        if let Some(mut n) = node {
            let (left_child, right_child) = if depth > 4 {
                let left_task = tokio::spawn(invert_tree(n.left, depth + 1));
                let right_task = tokio::spawn(invert_tree(n.right, depth + 1));
                
                let (left_res, right_res) = tokio::join!(left_task, right_task);
                (left_res.unwrap(), right_res.unwrap())
            } else {
                let l = n.left.take();
                let r = n.right.take();
                
                let left_res = invert_tree(l, depth + 1).await;
                let right_res = invert_tree(r, depth + 1).await;
                (left_res, right_res)
            };

            n.left = right_child;
            n.right = left_child;

            Some(n)
        } else {
            None
        }
    })
}

#[tokio::main]
async fn main() {
    let depth = 23;
    let large_tree = generate_full_tree(depth);

    let start = Instant::now();
    let _inverted_tree = invert_tree(large_tree, 1).await;
    dbg!(start.elapsed());
}
