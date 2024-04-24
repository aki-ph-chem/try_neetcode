use std::collections::VecDeque;

pub struct CompleteBinaryTree {
    bin_tree: Vec<i32>,
}

impl CompleteBinaryTree {
    pub fn new() -> Self {
        Self { bin_tree: vec![] }
    }

    pub fn push(&mut self, x: i32) {
        self.bin_tree.push(x);
    }

    pub fn show_dfs(&self) {
        Self::dfs(&self, 0);
        println!("end");
    }

    fn dfs(&self, i: usize) {
        let n = self.bin_tree.len();
        if i >= n {
            return;
        }
        print!("{} -> ", self.bin_tree[i]);

        for x in [2 * i + 1, 2 * i + 2] {
            Self::dfs(&self, x);
        }
    }

    pub fn show_bfs(&self) {
        Self::bfs(&self);
        println!("end");
    }

    fn bfs(&self) {
        let n = self.bin_tree.len();
        let mut todo = VecDeque::new();

        todo.push_back(0);

        while let Some(top) = todo.pop_front() {
            print!("{} -> ", self.bin_tree[top]);
            for x in [2 * top + 1, 2 * top + 2] {
                if x >= n {
                    continue;
                }

                todo.push_back(x);
            }
        }
    }
}

fn main() {
    let mut complete_bin_tree_0 = CompleteBinaryTree::new();
    for i in 1..=8 {
        complete_bin_tree_0.push(i);
    }

    println!("DFS");
    complete_bin_tree_0.show_dfs();
    println!("BFS");
    complete_bin_tree_0.show_bfs();
}
