use std::collections::{HashSet, VecDeque};

struct Solution {}
impl Solution {
    // AC
    pub fn open_lock(deadends: Vec<String>, target: String) -> i32 {
        let mut deadend_set: HashSet<Vec<i32>> = HashSet::new();
        for end in deadends {
            deadend_set.insert(end.chars().map(|c| c as i32 - '0' as i32).collect());
        }

        if deadend_set.contains(&vec![0, 0, 0, 0]) {
            return -1;
        }

        let target = target
            .chars()
            .map(|c| c as i32 - '0' as i32)
            .collect::<Vec<i32>>();

        let mut seen = HashSet::new();
        let mut q = VecDeque::new();

        seen.insert(vec![0, 0, 0, 0]);
        q.push_back(vec![0, 0, 0, 0]);

        let mut dist = vec![i32::MAX; 10000];
        dist[0] = 0;
        while let Some(q_front) = q.pop_front() {
            let q_front = q_front;
            let idx_v = Self::state_to_num(&q_front) as usize;

            for i in 0..4 {
                for d in [1, -1] {
                    let mut q_front_next = q_front.clone();
                    q_front_next[i] += d;

                    if q_front_next[i] == -1 {
                        q_front_next[i] = 9;
                    }
                    if q_front_next[i] == 10 {
                        q_front_next[i] = 0;
                    }

                    if q_front_next[i] < -1
                        || q_front_next[i] > 10
                        || seen.contains(&q_front_next)
                        || deadend_set.contains(&q_front_next)
                    {
                        continue;
                    }

                    let idx_next_v = Self::state_to_num(&q_front_next) as usize;
                    dist[idx_next_v] = dist[idx_next_v].min(dist[idx_v] + 1);

                    seen.insert(q_front_next.clone());
                    q.push_back(q_front_next.clone());
                }
            }
        }

        let idx_tareget = Self::state_to_num(&target) as usize;
        if dist[idx_tareget] == i32::MAX {
            return -1;
        }

        dist[idx_tareget]
    }

    fn state_to_num(state: &[i32]) -> i32 {
        let n = state.len() - 1;
        let mut num = 0;

        for (i, d) in state.iter().enumerate().rev() {
            num += d * 10_i32.pow((n - i) as u32);
        }

        num
    }
}

fn main() {
    let case_1 = (
        vec!["0201", "0101", "0102", "1212", "2002"]
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>(),
        "0202".to_string(),
    );
    // => 6
    let case_2 = (
        vec!["8888"]
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>(),
        "0009".to_string(),
    );
    // => 1
    let case_3 = (
        [
            "8887", "8889", "8878", "8898", "8788", "8988", "7888", "9888",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect::<Vec<String>>(),
        "8888".to_string(),
    );
    // => -1

    println!(
        "case_1: {}",
        Solution::open_lock(case_1.0.clone(), case_1.1.clone())
    );
    println!(
        "case_2: {}",
        Solution::open_lock(case_2.0.clone(), case_2.1.clone())
    );
    println!(
        "case_3: {}",
        Solution::open_lock(case_3.0.clone(), case_3.1.clone())
    );
}
