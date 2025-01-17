import gleam/option.{type Option, None, Some}

pub type ListNode {
    ListNode(
        content: Int,
        next: Option(ListNode)
    )
}

pub fn main() {
    let second = ListNode (
        content: 3,
        next: None
    )
    let first = ListNode (
        content: 3,
        next: Some(second)
    )
}
