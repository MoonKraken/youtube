use std::cell::Cell;

#[derive(Debug)]
struct Node<'a> {
    val: Cell<u32>,
    adjacent: Vec<&'a Node<'a>>,
}

fn main() {
    let a = Node {
        val: Cell::new(3),
        adjacent: vec![],
    };

    let b = Node {
        val: Cell::new(5),
        adjacent: vec![&a],
    };

    let c = Node {
        val: Cell::new(10),
        adjacent: vec![&a],
    };

    add_one(&c);

    dbg!(&c);
    dbg!(&b);
}

fn add_one(node: &Node) {
    let curr_val = node.val.get();
    node.val.set(curr_val + 1);
    for adjacent_node in node.adjacent.iter() {
        add_one(adjacent_node);
    }
}
