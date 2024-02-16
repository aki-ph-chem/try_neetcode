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

    fn chmin(a: &mut i32, b: i32) -> bool {
        if *a > b {
            *a = b;
            return true;
        }

        false
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
}
