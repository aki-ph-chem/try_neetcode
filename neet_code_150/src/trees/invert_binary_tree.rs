use std::{cell::RefCell, rc::Rc};

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

struct Solution {}
impl Solution {
    // AC
    fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(node) = root {
            let mut node_ref = node.borrow_mut();
            let (left, right) = (node_ref.left.take(), node_ref.right.take());

            node_ref.right = Solution::invert_tree(left);
            node_ref.left = Solution::invert_tree(right);

            Some(node.clone())
        } else {
            None
        }
    }

    // 模範解答
    fn invert_tree_ans(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        root.map(|node| {
            {
                let mut node_ref = node.borrow_mut();
                let left = node_ref.left.take();
                let right = node_ref.right.take();
                node_ref.right = Solution::invert_tree(left);
                node_ref.left = Solution::invert_tree(right);
            }
            node
        })
    }
}

fn main() {
    let case_1 = [4, 2, 7, 1, 3, 6, 9];
    let root_1_0 = Rc::new(RefCell::new(TreeNode::new(case_1[0])));
    let root_1_1 = Rc::new(RefCell::new(TreeNode::new(case_1[1])));
    let root_1_2 = Rc::new(RefCell::new(TreeNode::new(case_1[2])));

    let root_1_3 = Rc::new(RefCell::new(TreeNode::new(case_1[3])));
    let root_1_4 = Rc::new(RefCell::new(TreeNode::new(case_1[4])));
    let root_1_5 = Rc::new(RefCell::new(TreeNode::new(case_1[5])));
    let root_1_6 = Rc::new(RefCell::new(TreeNode::new(case_1[6])));

    root_1_0.borrow_mut().left = Some(Rc::clone(&root_1_1));
    root_1_1.borrow_mut().left = Some(Rc::clone(&root_1_3));
    root_1_1.borrow_mut().right = Some(Rc::clone(&root_1_4));

    root_1_0.borrow_mut().right = Some(Rc::clone(&root_1_2));
    root_1_2.borrow_mut().left = Some(Rc::clone(&root_1_5));
    root_1_2.borrow_mut().right = Some(Rc::clone(&root_1_6));

    let root_1 = Some(root_1_0);
    //println!("root_1: {:?}", root_1);
    let res_1 = Solution::invert_tree(root_1.clone());
    let res_1_2 = Solution::invert_tree_ans(root_1.clone());

    assert_eq!(res_1, res_1_2);

    let root_2 = Rc::new(RefCell::new(TreeNode::new(2)));
    root_2.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(1))));
    root_2.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(3))));
    let root_2 = Some(root_2);

    println!("root_2: {:?}", root_2);
    let res_2 = Solution::invert_tree(root_2.clone());
    println!("res_2: {:?}", res_2);
}
