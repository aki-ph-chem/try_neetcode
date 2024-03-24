#include <iostream>

class Solution {
    public:
        // AC
        // 直線探索
        int arrangeCoinsLinear(int n) {
            long long target = n; 
            auto sum_n = [](auto x) {return x *(x + 1)/ 2;};

            long long k = 0;
            while(true) {
                if(target == sum_n(k)) {
                    return k;
                }

                if(target < sum_n(k)) {
                    --k;
                    break;
                }

                ++k;
            }

            return k;
        }

        // AC
        // 二分探索
        int arrangeCoins(int n) {
            long long m = n; 
            auto sum_n = [](auto x) {return x *(x + 1)/ 2;};

            long long left = 0, mid = 0, right = m;
            while(left <= right) {
                mid = left + (right - left) / 2;

                if(n == sum_n(mid)) {
                    return mid;
                }

                if(n < sum_n(mid)) {
                    right = mid - 1;
                } else {
                    left = mid + 1;
                }
            }

            if(n < sum_n(mid)) {
                return mid - 1;
            }

            return mid;
        }
};

// 模範解答
// 二分探索
class SolutionAns {
    public:
        int arrangeCoins(int n) {
            int left = 1, right = n, result = 1;
            long long sum, m;

            while(left <= right) {
                m = left + (right - left) / 2;

                sum = m * (m + 1) / 2;

                if(sum == n) {
                    return (int)m;
                }

                if(n < sum) {
                    right = m - 1;
                } else {
                    result = (int)m;
                    left = m + 1;
                }
            }

            return result;
        }
};

int main(void) {
    int case_1 = 5;
    // => 2
    int case_2 = 8;
    // => 3
    int case_3 = 1804289383;

    Solution s_1;

    std::cout << s_1.arrangeCoins(case_1) << std::endl;
    std::cout << s_1.arrangeCoins(case_2) << std::endl;
    std::cout << s_1.arrangeCoins(case_3) << std::endl;

    std::cout << s_1.arrangeCoinsLinear(case_1) << std::endl;
    std::cout << s_1.arrangeCoinsLinear(case_2) << std::endl;
    std::cout << s_1.arrangeCoinsLinear(case_3) << std::endl;

    SolutionAns s_ans;

    std::cout << s_ans.arrangeCoins(case_1) << std::endl;
    std::cout << s_ans.arrangeCoins(case_2) << std::endl;
    std::cout << s_ans.arrangeCoins(case_3) << std::endl;
}
