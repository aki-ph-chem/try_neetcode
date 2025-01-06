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

fn gen_tree_dfs(i: usize, array: &[i32], root: &Option<Rc<RefCell<TreeNode>>>) {
    if let Some(node) = root {
        if i == 0 {
            node.borrow_mut().val = array[0];
        }

        if array.len() > 2 * i + 1 {
            node.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(array[2 * i + 1]))));
            gen_tree_dfs(2 * i + 1, array, &node.borrow_mut().left);
        }

        if array.len() > 2 * i + 2 {
            node.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(array[2 * i + 2]))));
            gen_tree_dfs(2 * i + 2, array, &node.borrow_mut().right);
        }
    }
}

struct Solution {}
impl Solution {
    // AC
    fn max_deth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(node) = root {
            let mut node_ref = node.borrow_mut();
            1 + Solution::max_deth(node_ref.left.take())
                .max(Solution::max_deth(node_ref.right.take()))
        } else {
            0
        }
    }
}

fn main() {
    let root_1 = Some(Rc::new(RefCell::new(TreeNode::new(0))));
    gen_tree_dfs(0, &[1, 2, 3, 4], &root_1);
    //println!("{:#?}", root_1);

    println!("root_1: {}", Solution::max_deth(root_1.clone()));
}
