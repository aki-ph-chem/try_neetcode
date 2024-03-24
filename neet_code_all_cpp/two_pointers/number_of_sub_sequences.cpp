#include <iostream>
#include <vector>
#include <algorithm>

// 模範解答
class SolutionAns {
    public:
        int numSubseq(std::vector<int> nums, int target) {
            std::sort(nums.begin(), nums.end());
            auto n = (int)nums.size();

            int left = 0, right = n - 1; 
            int result = 0, mod = 1e9 + 7;
            while(left <= right) {
                if(nums[left] + nums[right] > target) {
                    --right;
                } else {
                    result = (result + fastPower(2, right - left, mod)) % mod;
                    ++left;
                }
            }

            return result;
        }

        int fastPower(int a, int b, int mod) {
            long long result = 1;
            long long base = a;

            while(b != 0) {
                if(b % 2 == 1) {
                    result = (result * base) % mod;
                }
                base = (base * base) % mod;
                b /= 2;
            }

            return result;
        }
};

int main(void) {
    using Case = std::pair<std::vector<int>, int>;
    Case case_1 = {{3, 5, 6, 7}, 9};
    // 4: ([3], [3,5], [3,5,6], [3,6])
    Case case_2 = {{3, 3, 6, 8}, 10};
    // 6: ([3], [3], [3,3], [3,6], [3,6], [3,3,6])
    Case case_3 = {{2, 3, 3, 4, 6, 7}, 12};
    // 61

    SolutionAns s_ans;

    std::cout << "case_1: " << s_ans.numSubseq(case_1.first, case_1.second) << std::endl;
    std::cout << "case_2: " << s_ans.numSubseq(case_2.first, case_2.second) << std::endl;
    std::cout << "case_3: " << s_ans.numSubseq(case_3.first, case_3.second) << std::endl;
}
