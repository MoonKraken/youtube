#[derive(Debug)]
struct Node {
    val: u32,
    adjacent: Vec<Node>,
}

fn main() {
    let a = Node {
        val: 3,
        adjacent: vec![],
    };

    let b = Node {
        val: 5,
        adjacent: vec![a],
    };

    let c = Node {
        val: 10,
        adjacent: vec![b],
    };

    // add_one(c);

    // dbg!(&a);
}

// fn add_one(node: Node) {
//     node.val = node.val + 1;
//     for adjacent_node in node.adjacent.into_iter() {
//         add_one(adjacent_node);
//     }
// }
