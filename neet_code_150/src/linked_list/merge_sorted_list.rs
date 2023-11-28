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

struct Solution {}
impl Solution {
    pub fn merge_tow_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut list_1 = list1;
        let mut list_2 = list2;

        let mut current = Some(Box::new(ListNode::new(0)));
        let mut result = current;

        result
    }
}

// 模範解答
struct SolutionAns {}
// 再帰による実装
impl SolutionAns {
    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        match (list1, list2) {
            // listが空かチェック
            (Some(list1), None) => Some(list1),
            (None, Some(list2)) => Some(list2),
            (None, None) => None,
            // 両方空でない場合
            (Some(l1), Some(l2)) => {
                if l1.val < l2.val {
                    return Some(Box::new(ListNode {
                        val: l1.val,
                        next: SolutionAns::merge_two_lists(l1.next, Some(l2)),
                    }));
                } else {
                    return Some(Box::new(ListNode {
                        val: l2.val,
                        next: SolutionAns::merge_two_lists(Some(l1), l2.next),
                    }));
                }
            }
        }
    }

    // ループによる実装
    // 手元だとビルド&実行ができるがsubmitするとコンパイルできない
    pub fn merge_two_lists_loop(
        mut list1: Option<Box<ListNode>>,
        mut list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        // 結果用のノード
        let mut ans = ListNode::new(1);
        // 作業用のノード
        let mut cur = &mut ans;

        while let (Some(node1), Some(node2)) = (list1.as_mut(), list2.as_mut()) {
            // node1.valを採用する場合
            if node1.val < node2.val {
                // cur.nextにlist1の値を入れる
                cur.next = list1.take();
                // list1の値を一個進める(C++ではlist1 = list1->next)
                list1 = cur.next.as_mut().unwrap().next.take();
            } else {
                cur.next = list2.take();
                list2 = cur.next.as_mut().unwrap().next.take();
            }
            // curの値を一個進める(C++ではcur = cur->next)
            cur = cur.next.as_mut().unwrap();
        }
        // 最後にNoneでない方の値を末尾に追加
        cur.next = if list1.is_some() { list1 } else { list2 };
        // ansの先頭のSome(1)を飛ばしてreturn
        ans.next
    }
}

fn main() {
    let mut l_1_0 = ListNode::new(1);
    let mut l_1_1 = ListNode::new(8);
    let l_1_2 = ListNode::new(9);
    l_1_1.next = Some(Box::new(l_1_2));
    l_1_0.next = Some(Box::new(l_1_1));
    let ref_l_1 = Some(Box::new(l_1_0));
    show_list_ref(ref_l_1.clone());

    let mut l_2_0 = ListNode::new(3);
    let mut l_2_1 = ListNode::new(5);
    let l_2_2 = ListNode::new(11);
    l_2_1.next = Some(Box::new(l_2_2));
    l_2_0.next = Some(Box::new(l_2_1));
    let ref_l_2 = Some(Box::new(l_2_0));
    show_list_ref(ref_l_2.clone());

    let res_1 = SolutionAns::merge_two_lists(ref_l_1.clone(), ref_l_2.clone());
    show_list_ref(res_1);
    let res_1_loop = SolutionAns::merge_two_lists_loop(ref_l_1.clone(), ref_l_2.clone());
    show_list_ref(res_1_loop);
}
