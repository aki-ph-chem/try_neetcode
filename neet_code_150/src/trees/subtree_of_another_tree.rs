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
    // まだACしていない途中
    pub fn is_subtree(
        root: Option<Rc<RefCell<TreeNode>>>,
        sub_root: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        match (root, sub_root) {
            (None, _) => false,

            (Some(node_root), None) => {
                let mut node_root_ref = node_root.borrow_mut();
                Self::is_subtree(node_root_ref.left.take(), None)
                    || Self::is_subtree(node_root_ref.right.take(), None)
            }

            (Some(node_root), Some(node_sub_root)) => {
                if Self::is_same(Some(Rc::clone(&node_root)), Some(Rc::clone(&node_sub_root))) {
                    return true;
                }

                let mut node_root_ref = node_root.borrow_mut();
                Self::is_subtree(node_root_ref.left.take(), Some(Rc::clone(&node_sub_root)))
                    || Self::is_subtree(node_root_ref.right.take(), Some(Rc::clone(&node_sub_root)))
            }
        }
    }

    fn is_same(
        root: Option<Rc<RefCell<TreeNode>>>,
        subroot: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        match (root, subroot) {
            (None, None) => true,
            (Some(_node_p), None) => false,
            (None, Some(_node_q)) => false,

            (Some(node_p), Some(node_q)) => {
                let mut node_p_ref = node_p.borrow_mut();
                let mut node_q_ref = node_q.borrow_mut();
                if node_p_ref.val != node_q_ref.val {
                    return false;
                }

                Self::is_same(node_p_ref.left.take(), node_q_ref.left.take())
                    && Self::is_same(node_p_ref.right.take(), node_q_ref.right.take())
            }
        }
    }
}

fn main() {
    let root_1_1 = Some(Rc::new(RefCell::new(TreeNode::new(0))));
    gen_tree_dfs_2(0, &[Some(3), Some(4), Some(5), Some(1), Some(2)], &root_1_1);
    let root_1_2 = Some(Rc::new(RefCell::new(TreeNode::new(0))));
    gen_tree_dfs_2(0, &[Some(4), Some(1), Some(2)], &root_1_2);

    // => true
    println!("root_1: {}", Solution::is_subtree(root_1_1, root_1_2));

    let root_2_1 = Some(Rc::new(RefCell::new(TreeNode::new(0))));
    // [3,4,5,1,2,null,null,null,null,0]
    gen_tree_dfs_2(
        0,
        &[
            Some(3),
            Some(4),
            Some(5),
            Some(1),
            Some(2),
            None,
            None,
            None,
            None,
            Some(0),
        ],
        &root_2_1,
    );
    let root_2_2 = Some(Rc::new(RefCell::new(TreeNode::new(0))));
    gen_tree_dfs_2(0, &[Some(4), Some(1), Some(2)], &root_2_2);

    // => false
    println!("root_2: {}", Solution::is_subtree(root_2_1, root_2_2));

    let root_3_1 = Some(Rc::new(RefCell::new(TreeNode::new(0))));
    gen_tree_dfs_2(0, &[Some(1), Some(1)], &root_3_1);
    let root_3_2 = Some(Rc::new(RefCell::new(TreeNode::new(0))));
    gen_tree_dfs_2(0, &[Some(1)], &root_3_2);

    // => true
    // このケースがACしない
    println!("root_3: {}", Solution::is_subtree(root_3_1, root_3_2));
}
