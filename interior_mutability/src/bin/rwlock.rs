use std::{thread, sync::{RwLock, Arc}};

#[derive(Debug)]
struct Node {
    val: RwLock<String>,
    adjacent: Vec<Arc<Node>>,
}

fn main() {
    let a = Arc::new(
        Node {
            val: RwLock::new(String::from("asdf")),
            adjacent: vec![],
        }
    );

    let b = Arc::new(
        Node {
            val: RwLock::new(String::from("zzzz")),
            adjacent: vec![a.clone()],
        }
    );

    let c = Arc::new(
        Node {
            val: RwLock::new(String::from("fffff")),
            adjacent: vec![a.clone()]
        }
    );
    let c_t1 = c.clone();
    let t1 = thread::spawn( move || {
        add_urgency(&*c_t1);
    });

    let b_t2 = b.clone();
    let t2 = thread::spawn( move || {
        add_urgency(&*b_t2);
    });

    t1.join();
    t2.join();

    dbg!(&*b);
    dbg!(&*c);
}

fn add_urgency(node: &Node) {
    {
        let mut curr_val = node.val.write().unwrap();
        curr_val.push('!');
    }
    for adjacent_node in node.adjacent.iter() {
        add_urgency(adjacent_node);
    }
}
