use std::collections::VecDeque;

// 模範解答
// これは縛りを破った解答に見えるのだが...(top())
struct MystackAns {
    q: VecDeque<i32>,
}
impl MystackAns {
    pub fn new() -> Self {
        Self { q: VecDeque::new() }
    }

    pub fn push(&mut self, x: i32) {
        self.q.push_back(x);
    }

    pub fn pop(&mut self) -> i32 {
        for _i in 0..self.q.len() - 1 {
            let tmp = self.q.pop_front().unwrap();
            self.q.push_back(tmp);
        }
        self.q.pop_front().unwrap()
    }

    pub fn top(&self) -> i32 {
        self.q[self.q.len() - 1]
    }

    pub fn empty(&self) -> bool {
        self.q.is_empty()
    }
}

// C++の模範解答より
// AC
struct MystackAnsCpp {
    q_1: VecDeque<i32>,
    q_2: VecDeque<i32>,
}

impl MystackAnsCpp {
    pub fn new() -> Self {
        Self {
            q_1: VecDeque::new(),
            q_2: VecDeque::new(),
        }
    }

    pub fn push(&mut self, x: i32) {
        self.q_1.push_back(x);
    }

    pub fn pop(&mut self) -> i32 {
        while self.q_1.len() != 1 {
            self.q_2.push_back(self.q_1.pop_front().unwrap());
        }
        let result = self.q_1.pop_front().unwrap().clone();

        let q_1_copy = self.q_1.clone();
        self.q_1 = self.q_2.clone();
        self.q_2 = q_1_copy;

        result
    }

    pub fn top(&mut self) -> i32 {
        while self.q_1.len() != 1 {
            self.q_2.push_back(*self.q_1.front().unwrap());
            self.q_1.pop_front();
        }

        let result = self.q_1.front().unwrap().clone();
        self.q_1.pop_front();

        let q_1_copy = self.q_1.clone();
        self.q_1 = self.q_2.clone();
        self.q_2 = q_1_copy;
        self.q_1.push_back(result);

        result
    }

    pub fn empty(&self) -> bool {
        self.q_1.is_empty() && self.q_2.is_empty()
    }
}

fn main() {
    let mut s_ans = MystackAns::new();
    s_ans.push(1);
    s_ans.push(2);
    println!("{}", s_ans.top());
    println!("{}", s_ans.pop());
    println!("{}", s_ans.empty());

    let mut s_ans_cpp = MystackAnsCpp::new();
    s_ans_cpp.push(1);
    s_ans_cpp.push(2);
    println!("{}", s_ans_cpp.top());
    println!("{}", s_ans_cpp.pop());
    println!("{}", s_ans_cpp.empty());
}
