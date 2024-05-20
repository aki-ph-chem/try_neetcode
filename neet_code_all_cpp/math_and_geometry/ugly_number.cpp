#include <iostream>

class Solution {
    public:
        // AC
        bool isUgly(int n) {
            if(n < 0) {
                return false;
            }

            while(n != 0 && n % 2 == 0) {
                n /= 2;
            }
            while(n != 0 && n % 3 == 0) {
                n /= 3;
            }
            while(n != 0 && n % 5 == 0) {
                n /= 5;
            }

            return n == 1;
        }
};

// 模範解答
class SolutionAns {
    public:
        bool isUgly(int n) {
            if(n <= 0) {
                return false;
            }

            for(auto p: {2, 3, 5}) {
                while(n % p == 0) {
                    n /= p;
                }
            }

            return n == 1;
        }
};

int main(void) {
    int case_1 = 6;
    int case_2 = 1;
    int case_3 = 14;

    Solution s_1;

    std::cout << s_1.isUgly(case_1) << std::endl;
    std::cout << s_1.isUgly(case_2) << std::endl;
    std::cout << s_1.isUgly(case_3) << std::endl;

    SolutionAns s_ans;

    std::cout << s_ans.isUgly(case_1) << std::endl;
    std::cout << s_ans.isUgly(case_2) << std::endl;
    std::cout << s_ans.isUgly(case_3) << std::endl;
}
