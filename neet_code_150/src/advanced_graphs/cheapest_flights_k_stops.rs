use std::cmp::Reverse;
use std::collections::BinaryHeap;

// 解けなかった
struct Solution {}
impl Solution {
    pub fn find_cheapes_price(n: i32, flights: Vec<Vec<i32>>, src: i32, dst: i32, k: i32) -> i32 {
        // graph_w[from]: Vec<(i32, i32)> = vec![(to_1, price_1), (to_2, price_2),...];
        // グラフの構築
        let inf = i32::MAX;
        let mut dp: Vec<Vec<i32>> = vec![vec![inf; n as usize]; n as usize];
        for e in flights {
            let (from, to, price) = (e[0], e[1], e[2]);
            dp[from as usize][to as usize] = price;
        }

        // 境界条件
        for i in 0..n as usize {
            dp[i][i] = 0;
        }

        //dp遷移
        for bit in 0..(1 << n as u32) {
            for tmp in 0..n as u32 {
                if Self::num_1(bit) == k {
                    if bit & (1 << tmp) != 0 {
                        let dp_tmp =
                            dp[src as usize][tmp as usize] + dp[tmp as usize][dst as usize];
                        dp[src as usize][dst as usize] =
                            std::cmp::min(dp[src as usize][dst as usize], dp_tmp);
                    }
                }
            }
        }

        if dp[src as usize][dst as usize] == inf {
            return -1;
        }

        dp[src as usize][dst as usize]
    }

    fn num_1(n: u32) -> i32 {
        let mut sum = 0;
        for i in 0..32 {
            if n & (1 << i) != 0 {
                sum += 1;
            }
        }

        sum
    }
}

// AC
struct SolutionAnsCpp {}
impl SolutionAnsCpp {
    pub fn find_cheapes_price(n: i32, flights: Vec<Vec<i32>>, src: i32, dst: i32, k: i32) -> i32 {
        let mut adj = vec![vec![0; n as usize]; n as usize];
        for edge in flights {
            let (from, to, price) = (edge[0], edge[1], edge[2]);
            adj[from as usize][to as usize] = price;
        }

        let mut dist = vec![i32::MAX; n as usize];
        dist[src as usize] = 0;
        let mut current_stops = vec![i32::MAX; n as usize];
        current_stops[src as usize] = 0;

        // (cost, node, stops)
        let mut pq: BinaryHeap<(Reverse<i32>, Reverse<i32>, Reverse<i32>)> = BinaryHeap::new();
        pq.push((Reverse(0), Reverse(src), Reverse(0)));

        while !pq.is_empty() {
            let (cost, node, stops) = (
                pq.peek().unwrap().0 .0,
                pq.peek().unwrap().1 .0,
                pq.peek().unwrap().2 .0,
            );
            pq.pop();

            if node == dst {
                return cost;
            }

            if stops == k + 1 {
                continue;
            }

            for neighbor in 0..n {
                if adj[node as usize][neighbor as usize] > 0 {
                    let (current_cost, neighbor_dist, neighbor_weight) = (
                        cost,
                        dist[neighbor as usize],
                        adj[node as usize][neighbor as usize],
                    );

                    let current_dist = current_cost + neighbor_weight;
                    if current_dist < neighbor_dist || stops + 1 < current_stops[neighbor as usize]
                    {
                        pq.push((Reverse(current_dist), Reverse(neighbor), Reverse(stops + 1)));
                        dist[neighbor as usize] = current_dist;
                        current_stops[neighbor as usize] = stops;
                    } else if stops < current_stops[neighbor as usize] {
                        pq.push((Reverse(current_dist), Reverse(neighbor), Reverse(stops + 1)));
                    }
                    current_stops[neighbor as usize] = stops;
                }
            }
        }

        if dist[dst as usize] == i32::MAX {
            return -1;
        }

        dist[dst as usize]
    }
}

fn main() {
    // (n, flights, src, dst, k)
    let case_1 = (
        3,
        vec![vec![0, 1, 100], vec![1, 2, 100], vec![0, 2, 500]],
        0,
        2,
        1,
    );
    // => 200
    let case_2 = (
        3,
        vec![vec![0, 1, 100], vec![1, 2, 100], vec![0, 2, 500]],
        0,
        2,
        0,
    );
    // => 500

    /*
    println!(
        "case_1: {}",
        Solution::find_cheapes_price(case_1.0, case_1.1.clone(), case_1.2, case_1.3, case_1.4)
    );
    println!(
        "case_2: {}",
        Solution::find_cheapes_price(case_2.0, case_2.1.clone(), case_2.2, case_2.3, case_2.4)
    );
    */

    println!(
        "case_1: {}",
        SolutionAnsCpp::find_cheapes_price(
            case_1.0,
            case_1.1.clone(),
            case_1.2,
            case_1.3,
            case_1.4
        )
    );
    println!(
        "case_2: {}",
        SolutionAnsCpp::find_cheapes_price(
            case_2.0,
            case_2.1.clone(),
            case_2.2,
            case_2.3,
            case_2.4
        )
    );
}
