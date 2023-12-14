#include <iostream>
#include <vector>
#include <algorithm>

#define DEBUG

class Solution {
    public:
        // 模範解答
        int rob(std::vector<int>& nums) {
            int prev = 0;
            int curr = 0;
            int next = 0;

            for (int i = 0; i < nums.size(); i++) {
                next = std::max(prev + nums[i], curr);
                prev = curr;
                curr = next;

#ifdef DEBUG
                std::cout <<"prev, curr, next: " << prev << " ," <<  curr << ", " << next << std::endl;
#endif
            }

            return curr;
        }

        // 別解 
        int rob_2(std::vector<int>& nums) {
            int n = nums.size();
            std::vector<int> dp(n + 1, -(1<<30));

            if(n == 1) {
                return nums[0];
            }

            dp[0] = 0;
            dp[1] = nums[0];
            for(int i = 2; i < n + 1; ++i) {
                dp[i] = std::max(dp[i - 1], dp[i - 2] + nums[i - 1]);
            }

            return dp[n];
        }
};

int main(void) {
    auto case_1 = std::vector{1, 2, 3, 1};
    auto case_2 = std::vector{2, 7, 9, 3, 1};

    Solution s_ans;

    std::cout << s_ans.rob(case_1) << std::endl;
    std::cout << s_ans.rob(case_2) << std::endl;

    std::cout << s_ans.rob_2(case_1) << std::endl;
    std::cout << s_ans.rob_2(case_2) << std::endl;
}
