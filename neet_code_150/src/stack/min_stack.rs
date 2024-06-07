#[derive(Debug)]
struct MinStack {
    volume: Vec<i32>,
    min_list: Vec<i32>,
}

// push, pop, top, get_minをO(1)で
// AC
// しかし、配列を二本使っているせいでメモリ効率が悪い
impl MinStack {
    fn new() -> Self {
        MinStack {
            volume: vec![],
            min_list: vec![],
        }
    }

    fn push(&mut self, val: i32) {
        if let Some(min_value) = self.min_list.iter().rev().next() {
            if val <= *min_value {
                self.min_list.push(val);
            }
        } else {
            self.min_list.push(val);
        }
        self.volume.push(val);
    }

    fn pop(&mut self) {
        match self.volume.pop() {
            Some(t) => {
                if let Some(min_value) = self.min_list.iter().rev().next() {
                    if t == *min_value {
                        match self.min_list.pop() {
                            Some(_t) => {}
                            None => {}
                        }
                    }
                }
            }
            None => {}
        }
    }

    fn top(&self) -> i32 {
        match self.volume.iter().rev().next() {
            Some(t) => {
                eprintln!("{t}");
                *t
            }
            None => panic!("empty"),
        }
    }

    fn get_min(&self) -> i32 {
        if let Some(min_value) = self.min_list.iter().rev().next() {
            eprintln!("{min_value}");
            *min_value
        } else {
            panic!("empty");
        }
    }
}

#[derive(Debug)]
struct MinStackC {
    volume: Vec<(i32, i32)>,
}

/*
struct Hoge {
    a: Vec<i32>,
    b: Vec<i32>,
}

よりも

struct Hoge {
    a: Vec<(i32, i32)>
}

の方がメモリ効率が良い？
*/

// push, pop, top, get_minをO(1)で
// AC
// こっちの方がメモリ効率が良かった
impl MinStackC {
    fn new() -> Self {
        MinStackC { volume: vec![] }
    }

    fn push(&mut self, val: i32) {
        if self.volume.is_empty() {
            self.volume.push((val, val));
            return;
        }

        if let Some(current) = self.volume.iter().rev().next() {
            let min_current = current.1;
            if val <= min_current {
                self.volume.push((val, val));
            } else {
                self.volume.push((val, min_current));
            }
        }
    }

    fn pop(&mut self) {
        match self.volume.pop() {
            Some(_current) => {}
            None => {}
        }
    }

    fn top(&self) -> i32 {
        match self.volume.iter().rev().next() {
            Some(top) => {
                eprintln!("{}", top.0);
                (*top).0
            }
            None => panic!("empty"),
        }
    }

    fn get_min(&self) -> i32 {
        match self.volume.iter().rev().next() {
            Some(top) => {
                //println!("{}", (*top).1);
                (*top).1
            }
            None => {
                panic!("empty");
            }
        }
    }
}

#[derive(Debug)]
struct MinStackAns {
    stack: Vec<i32>,
    min_stack: Vec<i32>,
}

impl MinStackAns {
    fn new() -> Self {
        Self {
            stack: vec![],
            min_stack: vec![],
        }
    }

    fn push(&mut self, val: i32) {
        self.stack.push(val);
        if self.min_stack.is_empty() || Some(&val) <= self.min_stack.last() {
            self.min_stack.push(val);
        }
    }

    fn pop(&mut self) {
        let val = self.stack.pop().unwrap();
        if Some(&val) == self.min_stack.last() {
            self.min_stack.pop();
        }
    }

    fn top(&self) -> i32 {
        self.stack.last().cloned().unwrap()
    }

    fn get_min(&self) -> i32 {
        self.min_stack.last().cloned().unwrap()
    }
}

// 時間を開けて解いたときの別解
// AC
struct MinStackLatter {
    data: Vec<(i32, i32)>,
}

impl MinStackLatter {
    fn new() -> Self {
        Self { data: vec![] }
    }

    pub fn push(&mut self, val: i32) {
        if let Some(stack_top) = self.data.last() {
            self.data.push((val, stack_top.1.min(val)));
        } else {
            self.data.push((val, val));
        }
    }

    pub fn pop(&mut self) {
        self.data.pop();
    }

    pub fn top(&mut self) -> i32 {
        self.data.last().unwrap().0
    }

    pub fn get_min(&self) -> i32 {
        self.data.last().unwrap().1
    }
}

fn main() {
    let mut ms_1 = MinStack::new();
    println!("ms_1");
    ms_1.push(-2);
    ms_1.push(0);
    ms_1.push(-3);

    ms_1.get_min();
    ms_1.pop();
    ms_1.top();
    ms_1.get_min();

    let mut ms_2 = MinStack::new();
    println!("ms_2");
    ms_2.push(0);
    ms_2.push(1);
    ms_2.push(0);

    ms_2.get_min();
    ms_2.pop();
    ms_2.get_min();

    let mut ms_4 = MinStackC::new();
    println!("ms_4");
    println!("ms_4: {:?}", ms_4);
    ms_4.push(0);
    println!("ms_4: {:?}", ms_4);
    ms_4.push(1);
    println!("ms_4: {:?}", ms_4);
    ms_4.push(0);
    println!("ms_4: {:?}", ms_4);

    ms_4.get_min();
    ms_4.pop();
    ms_4.get_min();

    let mut ms_5 = MinStackAns::new();
    println!("ms_5");
    println!("ms_5: {:?}", ms_5);
    ms_5.push(0);
    println!("ms_5: {:?}", ms_5);
    ms_5.push(1);
    println!("ms_5: {:?}", ms_5);
    ms_5.push(0);
    println!("ms_5: {:?}", ms_5);

    let mut s_0 = MinStackLatter::new();
    s_0.push(-2);
    s_0.push(0);
    s_0.push(-3);
    println!("s_0.get_min(): {}", s_0.get_min());
    s_0.pop();
    println!("s_0.top(): {}", s_0.top());
    println!("s_0.get_min(): {}", s_0.get_min());
}
