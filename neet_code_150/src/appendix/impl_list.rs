// RustでunsafeなしでのLinked Listの実装
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct ListNode<T>
where
    T: std::fmt::Display,
{
    pub val: T,
    pub next: Option<Box<ListNode<T>>>,
}

impl<T> ListNode<T>
where
    T: std::fmt::Display,
{
    #[inline]
    fn new(val: T) -> Self {
        ListNode { next: None, val }
    }
}

// listを表示する
pub fn show_list<T>(head: ListNode<T>)
where
    T: std::fmt::Display,
{
    let mut node_current = head;
    loop {
        print!("{} -> ", node_current.val);

        if let Some(_node) = &node_current.next {
            node_current = *(node_current.next.unwrap());
        } else {
            break;
        }
    }
    println!("{}", node_current.val);
}

fn main() {
    let mut l_1_0 = ListNode::new(101);
    let mut l_1_1 = ListNode::new(102);
    let l_1_2 = ListNode::new(103);
    l_1_1.next = Some(Box::new(l_1_2));
    l_1_0.next = Some(Box::new(l_1_1));

    show_list(l_1_0.clone());

    let mut l_2_0 = ListNode::new(111);
    let mut l_2_1 = ListNode::new(222);
    let mut l_2_2 = ListNode::new(333);
    let l_2_3 = ListNode::new(444);
    l_2_2.next = Some(Box::new(l_2_3));
    l_2_1.next = Some(Box::new(l_2_2));
    l_2_0.next = Some(Box::new(l_2_1));

    show_list(l_2_0.clone());
}
