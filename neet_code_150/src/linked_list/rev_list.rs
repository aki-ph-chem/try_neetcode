#[derive(PartialEq,Eq,Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode{
            next: None,
            val,
        }
    }
}

pub fn show_list(head: &ListNode) {
    let mut node_current = head;
    loop {
        match &node_current.next {
            None => break,
            Some(v) =>{
                print!("{} -> ",v.val);
            }         
        }
    }
}

fn main() {
    let mut l_1_0 = ListNode::new(1);
    let mut l_1_1 = ListNode::new(2);
    let mut l_1_2 = ListNode::new(3);
    let mut l_1_3 = ListNode::new(4);
    let  l_1_4 = ListNode::new(5);

    l_1_3.next = Some(Box::new(l_1_4));
    l_1_2.next = Some(Box::new(l_1_3));
    l_1_1.next = Some(Box::new(l_1_2));
    l_1_0.next = Some(Box::new(l_1_1));

    println!("l_1_0: {:#?}",l_1_0);
}
