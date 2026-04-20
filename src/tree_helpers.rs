#[derive(Debug, Clone)]
pub struct TreeNode {
    pub data: i64,
    pub left: Option<Box<TreeNode>>,
    pub right: Option<Box<TreeNode>>,
}

impl TreeNode {
    fn new(data: i64) -> Self {
        TreeNode {
            data,
            left: None,
            right: None,
        }
    }

    fn with_children(data: i64, left: Box<TreeNode>, right: Box<TreeNode>) -> Self {
        TreeNode {
            data,
            left: Some(left),
            right: Some(right),
        }
    }
}

pub fn generate_full_tree(depth: usize) -> Option<Box<TreeNode>> {
    if depth == 0 {
        return None;
    }

    if depth == 1 {
        return Some(Box::new(TreeNode::new(rand::random())));
    }

    let left = generate_full_tree(depth - 1);
    let right = generate_full_tree(depth - 1);

    match (left, right) {
        (Some(l), Some(r)) => Some(Box::new(TreeNode::with_children(rand::random(), l, r))),
        _ => Some(Box::new(TreeNode::new(rand::random()))),
    }
}

pub fn print_tree(node: &Option<Box<TreeNode>>) {
    if let Some(n) = node {
        print_tree(&n.left);
        print!("{} ", &n.data);
        print_tree(&n.right);
    }
}
