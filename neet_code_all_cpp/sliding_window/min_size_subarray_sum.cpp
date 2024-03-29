#include <climits>
#include <iostream>
#include <vector>

class Solution {
    public:
        // AC
        // O(NlogN)
        int minSubArrayLen(int target, std::vector<int>& nums) {
            int n = nums.size();
            std::vector<int> prefix_sum(n + 1);
            prefix_sum[0] = 0;
            for(int i = 0; i < n; ++i) {
                prefix_sum[i + 1] = nums[i] + prefix_sum[i];
            }

            int result = INT_MAX;

            for(int i = 0; i <= n; ++i) {
                auto diff = target + prefix_sum[i];

                auto [left, right] = std::pair(0, n);
                while(left <= right) {
                    auto mid = left + (right - left) / 2;
                    if(prefix_sum[mid] == diff) {
                        result = std::min(result, std::abs(mid - i));
                        break;
                    }

                    if(prefix_sum[mid] < diff) {
                        left = mid + 1;
                    } else {
                        right = mid - 1;
                        result = std::min(result, std::abs(mid - i));
                    }
                }
            }

            if(result == INT_MAX) {
                return 0;
            }

            return result;
        }
};

// 模範解答
class SolutionAns {
    public:
        // AC
        int minSubArrayLen(int target, std::vector<int>& nums) {
            int fp = 0, sp = 1;
            int sum = nums[0];
            int min = INT_MAX;

            while(fp != sp) {
                if(sum >= target) {
                    min = std::min(sp - fp, min);
                    sum = sum - nums[fp];
                    ++fp;
                } else {
                    if(sp < nums.size()) {
                        sum += nums[sp];
                        ++sp;
                    } else {
                        ++fp;
                    }
                }
            }

            if(min == INT_MAX) {
                return 0;
            }

            return min;
        }
};

int main(void) {
    using Case = std::pair<int, std::vector<int>>;
    Case case_1 = {7, {2, 3, 1, 2, 4, 3}};
    // => 2
    Case case_2 = {4, {1, 4, 4}};
    // => 1
    Case case_3 = {11, {1, 1, 1, 1, 1, 1, 1, 1}};
    // => 0
    Case case_4 = {11, {1, 2, 3, 4, 5}};
    // => 3


    Solution s_1;
    std::cout << s_1.minSubArrayLen(case_1.first, case_1.second) << std::endl;
    std::cout << s_1.minSubArrayLen(case_2.first, case_2.second) << std::endl;
    std::cout << s_1.minSubArrayLen(case_3.first, case_3.second) << std::endl;
    std::cout << s_1.minSubArrayLen(case_4.first, case_4.second) << std::endl;

    SolutionAns s_ans;
    std::cout << s_ans.minSubArrayLen(case_1.first, case_1.second) << std::endl;
    std::cout << s_ans.minSubArrayLen(case_2.first, case_2.second) << std::endl;
    std::cout << s_ans.minSubArrayLen(case_3.first, case_3.second) << std::endl;
    std::cout << s_ans.minSubArrayLen(case_4.first, case_4.second) << std::endl;
}
