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

fn gen_tree_dfs_2(i: usize, array: &[Option<i32>], root: &Option<Rc<RefCell<TreeNode>>>) {
    if let Some(node) = root {
        if i == 0 {
            node.borrow_mut().val = array[0].unwrap();
        }

        if array.len() > 2 * i + 1 {
            node.borrow_mut().left = if let Some(value) = array[2 * i + 1] {
                Some(Rc::new(RefCell::new(TreeNode::new(value))))
            } else {
                None
            };
            gen_tree_dfs_2(2 * i + 1, array, &node.borrow_mut().left);
        }

        if array.len() > 2 * i + 2 {
            node.borrow_mut().right = if let Some(value) = array[2 * i + 2] {
                Some(Rc::new(RefCell::new(TreeNode::new(value))))
            } else {
                None
            };
            gen_tree_dfs_2(2 * i + 2, array, &node.borrow_mut().right);
        }
    }
}

struct Solution {}
impl Solution {
    // AC
    pub fn is_same_tree(
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        match (p, q) {
            (None, None) => true,
            (Some(_node_p), None) => false,
            (None, Some(_node_q)) => false,

            (Some(node_p), Some(node_q)) => {
                let mut node_p_ref = node_p.borrow_mut();
                let mut node_q_ref = node_q.borrow_mut();
                if node_p_ref.val != node_q_ref.val {
                    return false;
                }

                Self::is_same_tree(node_p_ref.left.take(), node_q_ref.left.take())
                    && Self::is_same_tree(node_p_ref.right.take(), node_q_ref.right.take())
            }
        }
    }
}

fn main() {
    let root_1_1 = Some(Rc::new(RefCell::new(TreeNode::new(0))));
    let root_1_2 = Some(Rc::new(RefCell::new(TreeNode::new(0))));
    gen_tree_dfs_2(0, &[Some(1), Some(2), Some(3)], &root_1_1);
    gen_tree_dfs_2(0, &[Some(1), Some(2), Some(3)], &root_1_2);

    println!(
        "root_1: {}",
        Solution::is_same_tree(root_1_1.clone(), root_1_2.clone())
    );

    let root_2_1 = Some(Rc::new(RefCell::new(TreeNode::new(0))));
    let root_2_2 = Some(Rc::new(RefCell::new(TreeNode::new(0))));
    gen_tree_dfs_2(0, &[Some(1), Some(2)], &root_2_1);
    gen_tree_dfs_2(0, &[Some(1), None, Some(3)], &root_2_2);

    println!(
        "root_2: {}",
        Solution::is_same_tree(root_2_1.clone(), root_2_2.clone())
    );
}
