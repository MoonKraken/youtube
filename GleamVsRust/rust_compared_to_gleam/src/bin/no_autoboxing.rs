struct ListNode {
    content: i64,
    next: Option<ListNode>
}

pub fn main() {
    let second = ListNode {
        content: 2,
        next: None,
    };
    let first = ListNode {
        content: 2,
        next: Some(second),
    };
}
