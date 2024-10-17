#include <algorithm>
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

// 後で解いたときの別解
class SolutionAnsLatter {
    public:
        int maxProfit(std::vector<int>& prices) {
            std::vector<int> min_list(prices.size());
            min_list[0] = prices[0];
            for(int i = 1; i < prices.size(); ++i) {
                min_list[i] = std::min(min_list[i - 1], prices[i]);
            }

            int result = -1;
            for(int i = 0; i < prices.size(); ++i) {
                result = std::max(result, prices[i] - min_list[i]);
            }

            return result;
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

    SolutionAnsLatter s_ans_latter;
    std::cout << s_ans_latter.maxProfit(case_1) << std::endl;
    std::cout << s_ans_latter.maxProfit(case_2) << std::endl;
}
