#include <algorithm>
#include <iostream>
#include <vector>
#include <climits>

// 模範解答
class SolutionAns {
    public:
        int maxProfit(std::vector<int>& prices) {
            int result = 0;

            for(int i = 1; i < prices.size(); ++i) {
                if(prices[i] > prices[i - 1]) {
                    result += prices[i] - prices[i - 1];
                }
            }

            return result;
        }
};

//Rustの模範解答より
class SolutionAnsRust {
    public:
        int maxProfit(std::vector<int>& prices) {
            int buy = INT_MAX; 
            int result = 0;

            for(auto& p: prices) {
                buy = std::min(buy, p - result);
                result = std::max(result, p - buy);
            }

            return result;
        }
};

int main(void) {
    std::vector<int> case_1 = {7, 1, 5, 3, 6, 4};
    // => 7
    std::vector<int> case_2 = {1, 2, 3, 4, 5};
    // => 4
    std::vector<int> case_3 = {7, 6, 4, 3, 1};
    // => 0

    SolutionAns s_ans;

    std::cout << s_ans.maxProfit(case_1) << std::endl;
    std::cout << s_ans.maxProfit(case_2) << std::endl;
    std::cout << s_ans.maxProfit(case_3) << std::endl;

    SolutionAnsRust s_ans_rs;

    std::cout << s_ans_rs.maxProfit(case_1) << std::endl;
    std::cout << s_ans_rs.maxProfit(case_2) << std::endl;
    std::cout << s_ans_rs.maxProfit(case_3) << std::endl;
}
