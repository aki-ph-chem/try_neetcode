use std::cmp::Reverse;
use std::collections::BinaryHeap;

// AC
// ダイクストラでkからの最短距離を求めてその最短距離のうちの最大値を探せば良い
struct Solution {}
impl Solution {
    pub fn netwok_delay_time(times: Vec<Vec<i32>>, n: i32, k: i32) -> i32 {
        let inf = i32::MAX;
        let mut used = vec![false; n as usize + 1];
        let mut dist = vec![inf; n as usize + 1];

        // グラフの構築
        let mut graph_w: Vec<Vec<(i32, i32)>> = vec![vec![]; n as usize + 1];
        for v_w in times {
            let (from, to, w) = (v_w[0], v_w[1], v_w[2]);
            graph_w[from as usize].push((to, w));
        }

        // スタート地点kを0に
        dist[k as usize] = 0;
        for _iter in 0..n {
            let mut min_dist = inf;
            let mut min_v = -1;
            for i in 1..=n {
                if !used[i as usize] && dist[i as usize] < min_dist {
                    min_dist = dist[i as usize];
                    min_v = i;
                }
            }

            if min_v == -1 {
                break;
            }

            // min_vからの緩和
            for e in &graph_w[min_v as usize] {
                let dist_tmp = dist[min_v as usize] + e.1;
                Self::chmin(&mut dist[e.0 as usize], dist_tmp);
            }

            used[min_v as usize] = true;
        }

        let mut result = -inf;
        for i in 1..=n {
            if i == k {
                continue;
            } else if dist[i as usize] > result {
                result = dist[i as usize];
            }

            if dist[i as usize] == inf {
                return -1;
            }
        }

        result
    }

    // AC
    // ヒープを用いたダイクストラの実装を使う
    pub fn netwok_delay_time_heap(times: Vec<Vec<i32>>, n: i32, k: i32) -> i32 {
        let inf = i32::MAX;
        let mut dist = vec![inf; n as usize + 1];

        // グラフの構築
        let mut graph_w: Vec<Vec<(i32, i32)>> = vec![vec![]; n as usize + 1];
        for v_w in times {
            let (from, to, w) = (v_w[0], v_w[1], v_w[2]);
            graph_w[from as usize].push((to, w));
        }

        // スタート地点kを0に
        dist[k as usize] = 0;
        let mut que: BinaryHeap<(Reverse<i32>, Reverse<i32>)> = BinaryHeap::new();
        que.push((Reverse(dist[k as usize]), Reverse(k)));

        while !que.is_empty() {
            let (v, d) = (que.peek().unwrap().1 .0, que.peek().unwrap().0 .0);
            que.pop();
            // dが"ゴミ"か否か
            if d > dist[v as usize] {
                continue;
            }
            // 緩和処理
            for e in &graph_w[v as usize] {
                let dist_tmp = dist[v as usize] + e.1;
                if Self::chmin(&mut dist[e.0 as usize], dist_tmp) {
                    // 更新された場合はヒープに値を追加
                    que.push((Reverse(dist[e.0 as usize]), Reverse(e.0)));
                }
            }
        }

        let mut result = -inf;
        for i in 1..=n {
            if i == k {
                continue;
            } else if dist[i as usize] > result {
                result = dist[i as usize];
            }

            if dist[i as usize] == inf {
                return -1;
            }
        }

        result
    }

    fn chmin(a: &mut i32, b: i32) -> bool {
        if *a > b {
            *a = b;
            return true;
        }

        false
    }
}

// C++の模範解答より
struct SolutionAnsCpp {}
impl SolutionAnsCpp {
    pub fn netwok_delay_time(times: Vec<Vec<i32>>, n: i32, k: i32) -> i32 {
        let inf = i32::MAX;

        // グラフの構築
        let mut adj: Vec<Vec<(i32, i32)>> = vec![vec![]; n as usize + 1];
        for v_w in times {
            let (from, to, time) = (v_w[0], v_w[1], v_w[2]);
            adj[from as usize].push((time, to));
        }

        let mut signal_receive_time = vec![inf; n as usize + 1];
        let mut que: BinaryHeap<(Reverse<i32>, Reverse<i32>)> = BinaryHeap::new();
        // スタート地点kを0に
        signal_receive_time[k as usize] = 0;
        que.push((Reverse(0), Reverse(k)));

        while !que.is_empty() {
            let (current_node_time, current_node) =
                (que.peek().unwrap().0 .0, que.peek().unwrap().1 .0);
            que.pop();

            // dが"ゴミ"か否か
            if current_node_time > signal_receive_time[current_node as usize] {
                continue;
            }

            // send signal to adjacent nodes
            for edge in &adj[current_node as usize] {
                let (time, neighbor_node) = (edge.0, edge.1);

                if signal_receive_time[neighbor_node as usize] > current_node_time + time {
                    signal_receive_time[neighbor_node as usize] = current_node_time + time;
                    que.push((
                        Reverse(signal_receive_time[neighbor_node as usize]),
                        Reverse(neighbor_node),
                    ));
                }
            }
        }

        //println!("singla_receive_time: {:?}", signal_receive_time);

        let mut result = i32::MIN;
        for i in 1..=n {
            result = result.max(signal_receive_time[i as usize]);
        }

        if result == inf {
            return -1;
        }

        result
    }
}

fn main() {
    let case_1 = (vec![vec![2, 1, 1], vec![2, 3, 1], vec![3, 4, 1]], 4, 2);
    // => 2
    let case_2 = (vec![vec![1, 2, 1]], 2, 1);
    // => 1
    let case_3 = (vec![vec![1, 2, 1]], 2, 2);
    // => -1

    println!(
        "case_1: {}",
        Solution::netwok_delay_time(case_1.0.clone(), case_1.1, case_1.2)
    );
    println!(
        "case_2: {}",
        Solution::netwok_delay_time(case_2.0.clone(), case_2.1, case_2.2)
    );
    println!(
        "case_3: {}",
        Solution::netwok_delay_time(case_3.0.clone(), case_1.1, case_1.2)
    );

    println!(
        "case_1: {}",
        Solution::netwok_delay_time_heap(case_1.0.clone(), case_1.1, case_1.2)
    );
    println!(
        "case_2: {}",
        Solution::netwok_delay_time_heap(case_2.0.clone(), case_2.1, case_2.2)
    );
    println!(
        "case_3: {}",
        Solution::netwok_delay_time_heap(case_3.0.clone(), case_1.1, case_1.2)
    );

    println!(
        "case_1: {}",
        SolutionAnsCpp::netwok_delay_time(case_1.0.clone(), case_1.1, case_1.2)
    );
    println!(
        "case_2: {}",
        SolutionAnsCpp::netwok_delay_time(case_2.0.clone(), case_2.1, case_2.2)
    );
    println!(
        "case_3: {}",
        SolutionAnsCpp::netwok_delay_time(case_3.0.clone(), case_3.1, case_3.2)
    );
}
