// 提出するコードはguess() の中のpickの値の変更ができない
// 模範解答
struct SolutionAns {}
impl SolutionAns {
    unsafe fn guessNumber(n: i32) -> i32 {
        Self::binary_search(1, n)
    }

    unsafe fn binary_search(left: i32, right: i32) -> i32 {
        let mid = left + (right - left) / 2;

        if Self::guess(mid) < 0 {
            Self::binary_search(1, mid)
        } else if Self::guess(mid) > 0 {
            Self::binary_search(mid + 1, right)
        } else {
            mid
        }
    }

    unsafe fn guess(num: i32) -> i32 {
        let pick = 6;

        if num > pick {
            return -1;
        }

        if num < pick {
            return 1;
        }

        0
    }
}

// 提出用コードをローカルで動くようにした
struct SolutionAnsLocal {
    pick: i32,
}

impl SolutionAnsLocal {
    fn guess_number(&self, n: i32) -> i32 {
        self.binary_search(1, n)
    }

    fn binary_search(&self, left: i32, right: i32) -> i32 {
        let mid = left + (right - left) / 2;

        if self.guess(mid) < 0 {
            self.binary_search(1, mid)
        } else if self.guess(mid) > 0 {
            self.binary_search(mid + 1, right)
        } else {
            mid
        }
    }

    fn guess(&self, num: i32) -> i32 {
        if num > self.pick {
            return -1;
        }

        if num < self.pick {
            return 1;
        }

        0
    }
}

fn main() {
    let s = SolutionAnsLocal { pick: 6 };
    println!("case_1: {}", s.guess_number(10));

    let s = SolutionAnsLocal { pick: 1 };
    println!("case_1: {}", s.guess_number(1));

    let s = SolutionAnsLocal { pick: 1 };
    println!("case_1: {}", s.guess_number(2));
}
