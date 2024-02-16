// AC
// 最小全域木の重みの合計を計算すれば良い
struct Solution {}
impl Solution {
    pub fn min_const_connect_points(points: Vec<Vec<i32>>) -> i32 {
        let l_1_nrom =
            |p_1: &Vec<i32>, p_2: &Vec<i32>| (p_1[0] - p_2[0]).abs() + (p_1[1] - p_2[1]).abs();

        // 重み付きグラフの構築
        let n = points.len();
        let mut graph_w: Vec<(i32, (i32, i32))> = vec![];

        for i in 0..n {
            for j in i + 1..n {
                graph_w.push((l_1_nrom(&points[i], &points[j]), (i as i32, j as i32)));
            }
        }

        // 重みでsort
        graph_w.sort_by(|a, b| a.0.cmp(&b.0));

        // クラスカル法
        let mut res = 0;
        let mut uf = vec![-1; n];
        let mut graph_size = vec![1; n];

        for e in &graph_w {
            let (w, u, v) = (e.0, e.1 .0, e.1 .1);

            if !Self::unite(u, v, &mut uf, &mut graph_size) {
                continue;
            }

            res += w;
        }

        res
    }

    // Union-Findの実装
    fn root(x: i32, uf: &mut Vec<i32>) -> i32 {
        {
            if uf[x as usize] == -1 {
                return x;
            }
            uf[x as usize] = Self::root(uf[x as usize], uf);

            uf[x as usize]
        }
    }

    fn unite(mut x: i32, mut y: i32, uf: &mut Vec<i32>, graph_size: &mut Vec<i32>) -> bool {
        x = Self::root(x, uf);
        y = Self::root(y, uf);

        if x == y {
            return false;
        }

        if graph_size[x as usize] < graph_size[y as usize] {
            std::mem::swap(&mut x, &mut y);
        }

        uf[y as usize] = x;
        graph_size[x as usize] += graph_size[y as usize];

        true
    }
}

// AC
// C++の模範解答より
struct SolutionAnsCpp {}
impl SolutionAnsCpp {
    pub fn min_const_connect_points(points: Vec<Vec<i32>>) -> i32 {
        let n = points.len() as i32;
        let inf = i32::MAX;

        let mut edge_used = 0;
        let mut in_mst = vec![false; n as usize];
        let mut min_dist = vec![inf; n as usize];
        min_dist[0] = 0;
        let mut result = 0;

        while edge_used < n {
            let mut current_min_edge = inf;
            let mut current_node = -1;

            // 貪欲法
            for i in 0..n {
                if !in_mst[i as usize] && current_min_edge > min_dist[i as usize] {
                    current_min_edge = min_dist[i as usize];
                    current_node = i;
                }
            }

            result += current_min_edge;
            edge_used += 1;
            in_mst[current_node as usize] = true;

            // update
            for i in 0..n {
                let cost = (points[current_node as usize][0] - points[i as usize][0]).abs()
                    + (points[current_node as usize][1] - points[i as usize][1]).abs();

                if !in_mst[i as usize] && min_dist[i as usize] > cost {
                    min_dist[i as usize] = cost;
                }
            }
        }

        result
    }
}

fn main() {
    let case_1 = vec![vec![0, 0], vec![2, 2], vec![3, 10], vec![5, 2], vec![7, 0]];
    // => 20
    let case_2 = vec![vec![3, 12], vec![-2, 5], vec![-4, 1]];
    // => 18

    println!(
        "case_1: {}",
        Solution::min_const_connect_points(case_1.clone())
    );
    println!(
        "case_2: {}",
        Solution::min_const_connect_points(case_2.clone())
    );

    println!(
        "case_1: {}",
        SolutionAnsCpp::min_const_connect_points(case_1.clone())
    );
    println!(
        "case_2: {}",
        SolutionAnsCpp::min_const_connect_points(case_2.clone())
    );
}
