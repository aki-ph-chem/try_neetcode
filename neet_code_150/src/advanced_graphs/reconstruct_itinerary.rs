use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

// 解けなかった(全く手が出なかった..)
struct Solution {}
impl Solution {
    pub fn find_itinerary(tickets: Vec<Vec<String>>) -> Vec<String> {
        vec!["foo".to_string()]
    }
}

// 模範解答
struct SolutionAns {}
impl SolutionAns {
    pub fn find_itinerary(tickets: Vec<Vec<String>>) -> Vec<String> {
        let mut graph: HashMap<&str, BinaryHeap<Reverse<&str>>> = HashMap::new();
        for ticket in tickets.iter() {
            graph
                .entry(&ticket[0])
                .or_insert_with(BinaryHeap::new)
                .push(Reverse(&ticket[1]));
        }

        let mut answer = vec![];
        let mut stack: Vec<&str> = vec!["JFK"];
        while let Some(src) = stack.last() {
            if let Some(dsts) = graph.get_mut(src) {
                if !dsts.is_empty() {
                    if let Some(dst) = dsts.pop() {
                        stack.push(dst.0);
                    }
                    continue;
                }
            }
            if let Some(last) = stack.pop() {
                answer.push(last.to_string());
            }
        }

        answer.reverse();
        answer
    }
}

fn main() {
    let case_1 = vec![
        vec!["MUC".to_string(), "LHR".to_string()],
        vec!["JFK".to_string(), "MUC".to_string()],
        vec!["SFO".to_string(), "SJC".to_string()],
        vec!["LHR".to_string(), "SFO".to_string()],
    ];
    // => ["JFK","MUC","LHR","SFO","SJC"]
    let case_2 = vec![
        vec!["JFK".to_string(), "SFO".to_string()],
        vec!["JFK".to_string(), "ATL".to_string()],
        vec!["SFO".to_string(), "ATL".to_string()],
        vec!["ATL".to_string(), "JFK".to_string()],
        vec!["ATL".to_string(), "SFO".to_string()],
    ];
    // => ["JFK","ATL","JFK","SFO","ATL","SFO"]

    println!("case_1: {:?}", SolutionAns::find_itinerary(case_1.clone()));
    println!("case_2: {:?}", SolutionAns::find_itinerary(case_2.clone()));
}
