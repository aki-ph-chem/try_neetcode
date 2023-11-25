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

pub fn show_list(head: ListNode) {
    let mut node_current = head;
    loop {
        if let Some(_node) = &node_current.next {
            print!("{} -> ", node_current.val);
            node_current = *(node_current.next.unwrap());
        } else {
            break;
        }
    }
    println!("{}", node_current.val);
}

pub fn show_list_ref(head: Option<Box<ListNode>>) {
    let mut node_current = head;

    while let Some(node) = node_current {
        print!("{} -> ", node.val);
        node_current = node.next;
    }
    println!("");
}

struct Solution {}
// 模範解答
impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let (mut prev, mut current) = (None, head);
        while let Some(mut node) = current {
            current = node.next;
            node.next = prev;
            prev = Some(node);
        }
        prev
    }
}

fn main() {
    let mut l_1_0 = ListNode::new(1);
    let mut l_1_1 = ListNode::new(2);
    let mut l_1_2 = ListNode::new(3);
    let mut l_1_3 = ListNode::new(4);
    let l_1_4 = ListNode::new(5);
    l_1_3.next = Some(Box::new(l_1_4));
    l_1_2.next = Some(Box::new(l_1_3));
    l_1_1.next = Some(Box::new(l_1_2));
    l_1_0.next = Some(Box::new(l_1_1));

    show_list(l_1_0.clone());

    let ref_l_1_0 = Some(Box::new(l_1_0));
    show_list_ref(ref_l_1_0.clone());

    let res_1 = Solution::reverse_list(ref_l_1_0); 
    show_list_ref(res_1);
}
