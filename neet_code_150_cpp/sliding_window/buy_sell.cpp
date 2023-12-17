#include <iostream>
#include <vector>

// Rustの模範解答より
class Solution {
    public:
        int max_profile(std::vector<int>& prices) {
            int max_profile = 0;
            int l = 0, r = 1;

            while(r < prices.size()) {
                if(prices[l] < prices[r]) {
                    int profile = prices[r] - prices[l];
                    max_profile = std::max(profile, max_profile);
                } else {
                    l = r;
                }
                ++r;
            }

            return max_profile;
        }
};

// 模範解答
class SolutionAns {
public:
    int maxProfit(std::vector<int>& prices) {
        int maxP = 0, l = 0, r = 0;
        while (r < prices.size()){
            if (prices[r] > prices[l])
                maxP = std::max(maxP, prices[r] - prices[l]);
            else
                l = r;
            ++r;
        }
        return maxP;
    }
};

int main(void) {
    auto case_1 = std::vector{7, 1, 5, 3, 6, 4};
    auto case_2 = std::vector{7, 6, 4, 3, 1};

    Solution s_1;
    std::cout << s_1.max_profile(case_1) << std::endl;
    std::cout << s_1.max_profile(case_2) << std::endl;

    SolutionAns s_ans;
    std::cout << s_ans.maxProfit(case_1) << std::endl;
    std::cout << s_ans.maxProfit(case_2) << std::endl;
}
