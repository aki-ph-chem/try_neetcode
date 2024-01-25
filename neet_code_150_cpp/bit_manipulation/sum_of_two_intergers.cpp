#include <iostream>

// and: carryを計算する, xor: carryなしの加算
// 模範解答
class SolutionAns {
    public:
        int getSum(int a, int b) {
            while (b != 0) {
                int carry = a & b;
                a = a ^ b;
                b = (unsigned)carry << 1;
            }
            return a;
        }
};

// Rustの模範解答(再帰で同じことをしている)
class SolutionAnsRust {
    public:
        int getSum(int a, int b) {
            return rec(a, b);
        }

    private:
        int rec(int a, int b) {
            if((a & b) << 1 == 0) {
                return a ^ b;
            }

            return rec(a ^ b, (a & b) << 1);
        }
};

int main(void) {
    SolutionAns s_ans;

    std::cout << s_ans.getSum(2,1) << std::endl;
    std::cout << s_ans.getSum(2,3) << std::endl;

    SolutionAnsRust s_ans_rs;
    std::cout << s_ans_rs.getSum(2,1) << std::endl;
    std::cout << s_ans_rs.getSum(2,3) << std::endl;
}
