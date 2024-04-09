use std::collections::HashMap;
use std::collections::HashSet;

// 再帰によるDFS
struct Dfs1 {}
impl Dfs1 {
    pub fn search(graph: &Vec<Vec<i32>>, num_node: i32) {
        let mut visit = vec![false; num_node as usize];
        for i in 0..num_node {
            if visit[i as usize] == true {
                continue;
            }
            Self::dfs_rec(&graph, &mut visit, i as i32);
        }
    }

    // Vec<bool>で訪問済みかを判断
    fn dfs_rec(graph: &Vec<Vec<i32>>, visit: &mut Vec<bool>, node: i32) {
        visit[node as usize] = true;
        for next_v in &graph[node as usize] {
            if visit[*next_v as usize] {
                continue;
            }
            print!("{} -> ", next_v);
            Self::dfs_rec(graph, visit, *next_v);
        }
        println!("");
    }
}

struct Dfs2 {}
impl Dfs2 {
    pub fn search(graph: &Vec<Vec<i32>>, num_node: i32) {
        let mut visit = HashSet::new();
        for i in 0..num_node {
            if visit.contains(&i) {
                continue;
            }

            Self::dfs_rec(&graph, &mut visit, i as i32);
        }
    }

    // HashSet<i32>で訪問済みかを判断
    pub fn dfs_rec(graph: &Vec<Vec<i32>>, visit: &mut HashSet<i32>, node: i32) {
        visit.insert(node);

        for next_v in &graph[node as usize] {
            if visit.contains(next_v) {
                continue;
            }
            print!("{} -> ", next_v);
            Self::dfs_rec(graph, visit, *next_v);
        }
        println!("");
    }
}

// グラフをHashMap<i32,Vec<i32>>で表す
struct Dfs3 {}
impl Dfs3 {
    pub fn search(graph: &HashMap<i32, Vec<i32>>, num_node: i32) {
        let mut visit = HashSet::new();
        for i in 0..num_node {
            if visit.contains(&i) {
                continue;
            }

            Self::dfs_rec(&graph, &mut visit, i as i32);
        }
    }

    // HashSet<i32>で訪問済みかを判断
    pub fn dfs_rec(graph: &HashMap<i32, Vec<i32>>, visit: &mut HashSet<i32>, node: i32) {
        visit.insert(node);

        for next_v in &graph[&node] {
            if visit.contains(next_v) {
                continue;
            }
            print!("{} -> ", next_v);
            Self::dfs_rec(graph, visit, *next_v);
        }
        println!("");
    }
}

// [from, to]の配列を受け取って自分でHashMapでグラフを構築する場合はこっち
struct Dfs4 {}
impl Dfs4 {
    pub fn search(graph: &HashMap<i32, Vec<i32>>, num_node: i32) {
        let mut visit = HashSet::new();
        for i in 0..num_node {
            if visit.contains(&i) {
                continue;
            }

            Self::dfs_rec(&graph, &mut visit, i as i32);
        }
    }

    // HashSet<i32>で訪問済みかを判断
    pub fn dfs_rec(graph: &HashMap<i32, Vec<i32>>, visit: &mut HashSet<i32>, node: i32) {
        visit.insert(node);

        if let Some(next_v_list) = graph.get(&node) {
            for next_v in next_v_list {
                if visit.contains(next_v) {
                    continue;
                }
                print!("{} -> ", next_v);
                Self::dfs_rec(graph, visit, *next_v);
            }
        }
        println!("");
    }
}

fn main() {
    let graph = (5, vec![vec![1, 2], vec![3], vec![3], vec![4], vec![]]);

    println!("Dfs1");
    Dfs1::search(&graph.1, graph.0);
    println!("Dfs2");
    Dfs2::search(&graph.1, graph.0);

    let graph_map_0 = (
        5,
        HashMap::from([
            (0, vec![1, 2]),
            (1, vec![3]),
            (2, vec![3]),
            (3, vec![4]),
            (4, vec![]),
        ]),
    );

    println!("Dfs3");
    Dfs3::search(&graph_map_0.1, graph_map_0.0);
    println!("Dfs4");
    Dfs4::search(&graph_map_0.1, graph_map_0.0);

    let graph_map_1 = (
        5,
        HashMap::from([(0, vec![1, 2]), (1, vec![3]), (2, vec![3]), (3, vec![4])]),
    );

    println!("Dfs4");
    Dfs4::search(&graph_map_1.1, graph_map_1.0);
}
