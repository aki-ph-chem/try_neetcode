#include <iostream>
#include <cmath>

class Solution {
    public:
        // AC
        bool isPerfectSquare(int num) {
            if(num == 1) {
                return true;
            }

            long long num_long = num;

            long long left = 0;
            long long right = num_long;
            while(left <= right) {
                long long mid = left + (right - left) / 2;

                if(num == std::pow(mid, 2)) {
                    return true;
                }

                if(std::pow(mid, 2) < num) {
                    left = mid + 1;
                } else {
                    right = mid - 1;
                }
            }

            return false;
        }
};

// 模範解答
class SolutionAns {
    public:
        bool isPerfectSquare(int num) {
            if(num == 1) {
                return true;
            }

            int left = 0, right = num;
            while(left < right) {
                int m = left + (right - left) / 2;
                float x = (float)num / (float)m;

                if(x == m) {
                    return true;
                } else if(x < m) {
                    right = m;
                } else {
                    left = m + 1;
                }
            }

            return false;
        }
};

int main(void) {
    int case_1 = 16;
    // => true
    int case_2 = 14;
    // => false
    int case_3 = 2147483647;
    // => false

    Solution s_1;
    std::cout << s_1.isPerfectSquare(case_1) << std::endl;
    std::cout << s_1.isPerfectSquare(case_2) << std::endl;
    std::cout << s_1.isPerfectSquare(case_3) << std::endl;

    SolutionAns s_ans;
    std::cout << s_ans.isPerfectSquare(case_1) << std::endl;
    std::cout << s_ans.isPerfectSquare(case_2) << std::endl;
    std::cout << s_ans.isPerfectSquare(case_3) << std::endl;
}
