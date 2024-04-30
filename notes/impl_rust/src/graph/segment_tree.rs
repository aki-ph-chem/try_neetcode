// 参考
#[derive(Debug)]
struct SgumentTreeSum {
    tree: Vec<i32>,
    array_size: usize,
}

impl SgumentTreeSum {
    pub fn new(array: &Vec<i32>) -> Self {
        let n = array.len();
        let mut segment_tree = Self {
            // なんで 4n ?
            tree: vec![0; 4 * n],
            array_size: n,
        };

        segment_tree.build(1, 0, n - 1, array);
        segment_tree
    }

    pub fn build(&mut self, node: usize, left: usize, right: usize, array: &Vec<i32>) {
        if left == right {
            self.tree[node] = array[left];
        } else {
            let mid = (left + right) / 2;
            Self::build(self, 2 * node, left, mid, array);
            Self::build(self, 2 * node + 1, mid + 1, right, array);

            self.tree[node] = self.tree[2 * node] + self.tree[2 * node + 1];
        }
    }
}

fn main() {
    let case_1 = vec![0, 1, 3, 5, -2, 3];
    let seg_tree_1 = SgumentTreeSum::new(&case_1);
    println!("seg_tree_1: \n{:?}", seg_tree_1);
}
