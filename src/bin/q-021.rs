// // Python code
// def mergeTwoLists(l1: ListNode, l2: ListNode) -> ListNode:
//     tmp = ListNode()
//     curr = tmp

//     while l1 is not None and l2 is not None:
//         if l1.val < l2.val:
//             curr.next = l1
//             l1 = l1.next
//         else:
//             curr.next = l2
//             l2 = l2.next

//         curr = curr.next

//     if l1 is not None:
//         curr.next = l1
//         l1 = l1.next

//     if l2 is not None:
//         curr.next = l2
//         l2 = l2.next

//     return tmp.next

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

// Can't figure this out... ._.
fn merge_two_lists(
    mut l1: Option<Box<ListNode>>,
    mut l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    todo!();
}
