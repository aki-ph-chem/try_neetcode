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
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let mut height = 0;
        Solution::dfs(root, &mut height)
    }

    fn dfs(root: Option<Rc<RefCell<TreeNode>>>, height: &mut i32) -> bool {
        if let Some(node) = root {
            let (mut left, mut right) = (0, 0);
            let mut node_ref = node.borrow_mut();

            if !Solution::dfs(node_ref.left.take(), &mut left)
                || !Solution::dfs(node_ref.right.take(), &mut right)
            {
                return false;
            }

            if (left - right).abs() > 1 {
                return false;
            }

            *height = 1 + left.max(right);

            true
        } else {
            *height = -1;
            true
        }
    }
}

fn main() {
    let root_1 = Some(Rc::new(RefCell::new(TreeNode::new(0))));
    //[3,9,20,null,null,15,7]
    gen_tree_dfs_2(
        0,
        &[Some(3), Some(9), Some(20), None, None, Some(15), Some(7)],
        &root_1,
    );
    println!("root_1: {}", Solution::is_balanced(root_1.clone()));

    let root_2 = Some(Rc::new(RefCell::new(TreeNode::new(0))));
    //[1,2,2,3,3,null,null,4,4]
    gen_tree_dfs_2(
        0,
        &[
            Some(1),
            Some(2),
            Some(2),
            Some(3),
            Some(3),
            None,
            None,
            Some(4),
            Some(4),
        ],
        &root_2,
    );
    println!("root_2: {}", Solution::is_balanced(root_2.clone()));
}
