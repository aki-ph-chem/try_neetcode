use std::collections::{BinaryHeap, HashMap, VecDeque};

// 解けなかった
struct Soltuion {}
impl Soltuion {
    pub fn least_interval(tasks: Vec<char>, n: i32) -> i32 {
        let mut map: HashMap<char, i32> = HashMap::new();
        for c in tasks {
            *map.entry(c).or_default() += 1;
        }

        2
    }
}

// 模範解答
struct SolutionAns {}
impl SolutionAns {
    pub fn least_interval(tasks: Vec<char>, n: i32) -> i32 {
        let mut count: HashMap<char, i32> = HashMap::new();
        let mut max_heap = BinaryHeap::new();

        for task in tasks {
            *count.entry(task).or_default() += 1;
        }
        for (_key, val) in count.iter() {
            max_heap.push(*val);
        }

        let mut time = 0;
        let mut deque: VecDeque<(i32, i32)> = VecDeque::new();
        while deque.len() > 0 || max_heap.len() > 0 {
            time += 1;

            if max_heap.len() == 0 {
                time = deque[0].1;
            } else {
                let cnt = max_heap.pop().unwrap() - 1;
                if cnt != 0 {
                    deque.push_back((cnt, time + n));
                }
            }
            if deque.len() > 0 && deque[0].1 == time {
                max_heap.push(deque.pop_front().unwrap().0);
            }
        }

        time
    }
}

// C++の模範解答より
struct SolutionAnsCpp {}
impl SolutionAnsCpp {
    pub fn least_interval(tasks: Vec<char>, n: i32) -> i32 {
        let mut pq: BinaryHeap<i32> = BinaryHeap::new();
        let mut q: VecDeque<(i32, i32)> = VecDeque::new();
        let mut map: HashMap<char, i32> = HashMap::new();

        for c in tasks {
            *map.entry(c).or_default() += 1;
        }
        for (_c, v) in map {
            pq.push(v);
        }
        /*
        println!("pq: {:?}", pq);
        println!("q: {:?}", q);
        */

        let mut time = 0;
        while !q.is_empty() || !pq.is_empty() {
            time += 1;
            if !pq.is_empty() {
                if pq.peek().unwrap() - 1 != 0 {
                    q.push_back((pq.peek().unwrap() - 1, time + n));
                }
                pq.pop();
            }

            if !q.is_empty() && q.front().unwrap().1 == time {
                pq.push(q.front().unwrap().0);
                q.pop_front();
            }
            //println!("time: {}", time);
        }

        time
    }
}

fn main() {
    let case_1 = (vec!['A', 'A', 'A', 'B', 'B', 'B'], 2);
    // => 8
    let case_2 = (vec!['A', 'A', 'A', 'B', 'B', 'B'], 0);
    // => 6

    println!(
        "case_1: {}",
        SolutionAns::least_interval(case_1.0.clone(), case_1.1)
    );
    println!(
        "case_2: {}",
        SolutionAns::least_interval(case_2.0.clone(), case_2.1)
    );

    println!(
        "case_1: {}",
        SolutionAnsCpp::least_interval(case_1.0.clone(), case_1.1)
    );
    println!(
        "case_2: {}",
        SolutionAnsCpp::least_interval(case_2.0.clone(), case_2.1)
    );
}
