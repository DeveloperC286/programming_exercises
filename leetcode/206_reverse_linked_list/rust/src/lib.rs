#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

pub fn reverse_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut reversed_list = None;

    while let Some(mut reversed_head) = head {
        head = reversed_head.next;
        reversed_head.next = reversed_list;
        reversed_list = Some(reversed_head);
    }

    reversed_list
}

#[cfg(test)]
mod tests;
