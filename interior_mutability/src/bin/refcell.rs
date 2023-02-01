use std::cell::RefCell;

#[derive(Debug)]
struct Node<'a> {
    val: RefCell<String>,
    adjacent: Vec<&'a Node<'a>>,
}

fn main() {
    let a = Node {
        val: RefCell::new(String::from("asdf")),
        adjacent: vec![],
    };

    let b = Node {
        val: RefCell::new(String::from("zzzz")),
        adjacent: vec![&a],
    };

    let c = Node {
        val: RefCell::new(String::from("fffff")),
        adjacent: vec![&a],
    };

    add_urgency(&c);

    dbg!(&c);
    dbg!(&b);
}

fn add_urgency(node: &Node) {
    let mut curr_val = node.val.borrow_mut();
    curr_val.push('!');
    for adjacent_node in node.adjacent.iter() {
        add_urgency(adjacent_node);
    }
}
