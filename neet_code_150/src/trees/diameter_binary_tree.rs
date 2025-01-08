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
    pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut result = 0_i32;
        let _ = Solution::dfs(root, &mut result);

        result
    }

    fn dfs(root: Option<Rc<RefCell<TreeNode>>>, dst: &mut i32) -> i32 {
        if let Some(node) = root {
            let mut node_ref = node.borrow_mut();

            let left = Solution::dfs(node_ref.left.take(), dst);
            let right = Solution::dfs(node_ref.right.take(), dst);
            *dst = (*dst).max(left + right);

            1 + left.max(right)
        } else {
            0
        }
    }
}

fn main() {
    let root_1 = Some(Rc::new(RefCell::new(TreeNode::new(0))));
    gen_tree_dfs(0, &[1, 2, 3, 4, 5], &root_1);
    let root_2 = Some(Rc::new(RefCell::new(TreeNode::new(0))));
    gen_tree_dfs(0, &[1, 2], &root_2);

    println!(
        "root_1: {}",
        Solution::diameter_of_binary_tree(root_1.clone())
    );
    println!(
        "root_2: {}",
        Solution::diameter_of_binary_tree(root_2.clone())
    );
}
