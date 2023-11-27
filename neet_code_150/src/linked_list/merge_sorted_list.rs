#[derive(PartialEq, Eq, Debug, Clone)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub fn show_list_ref(head: Option<Box<ListNode>>) {
    let mut node_current = head;

    while let Some(node) = node_current {
        print!("{} -> ", node.val);
        node_current = node.next;
    }
    println!("");
}

struct Solution{}
impl Solution {
    pub fn merge_tow_lists(list1: Option<Box<ListNode>>, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut list_1 = list1;
        let mut list_2 = list2;

        let mut current = Some(Box::new(ListNode::new(0)));
        let mut result = current;

        result
    }
}

fn main() {
    let mut l_1_0 = ListNode::new(1);
    let mut l_1_1 = ListNode::new(8);
    let  l_1_2 = ListNode::new(9);
    l_1_1.next = Some(Box::new(l_1_2));
    l_1_0.next = Some(Box::new(l_1_1));
    let ref_l_1 = Some(Box::new(l_1_0));
    show_list_ref(ref_l_1.clone());

    let mut l_2_0 = ListNode::new(3);
    let mut l_2_1 = ListNode::new(5);
    let  l_2_2 = ListNode::new(11);
    l_2_1.next = Some(Box::new(l_2_2));
    l_2_0.next = Some(Box::new(l_2_1));
    let ref_l_2 = Some(Box::new(l_2_0));
    show_list_ref(ref_l_2.clone());
}
