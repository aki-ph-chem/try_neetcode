use std::collections::HashSet;
type Graph = Vec<Vec<i32>>;

struct TopologicalSort {}
impl TopologicalSort {
    pub fn topological_sort(num_nodes: i32, graph: &Graph) -> Vec<i32> {
        let mut seen = HashSet::new();
        let mut order = vec![];
        for node in 0..num_nodes {
            if seen.contains(&node) {
                continue;
            }
            Self::dfs(node, &graph, &mut seen, &mut order);
        }

        order.reverse();
        order
    }

    fn dfs(node: i32, graph: &Graph, seen: &mut HashSet<i32>, order: &mut Vec<i32>) {
        seen.insert(node);

        for next_node in &graph[node as usize] {
            if seen.contains(&next_node) {
                continue;
            }
            Self::dfs(*next_node, graph, seen, order);
        }
        order.push(node);
    }
}

fn main() {
    let case_1 = (4, vec![vec![0, 1], vec![1, 2], vec![2, 3]]);
    let mut graph_1 = vec![vec![]; case_1.0];
    for node in case_1.1 {
        let (from, to) = (node[0], node[1]);
        graph_1[from as usize].push(to);
    }

    let case_2 = (
        5,
        vec![vec![0, 1], vec![0, 2], vec![1, 3], vec![2, 3], vec![3, 4]],
    );
    let mut graph_2 = vec![vec![]; case_2.0];
    for node in case_2.1 {
        let (from, to) = (node[0], node[1]);
        graph_2[from as usize].push(to);
    }

    println!(
        "case_1: {:?}",
        TopologicalSort::topological_sort(case_1.0 as i32, &graph_1)
    );
    println!(
        "case_2: {:?}",
        TopologicalSort::topological_sort(case_2.0 as i32, &graph_2)
    );
}
