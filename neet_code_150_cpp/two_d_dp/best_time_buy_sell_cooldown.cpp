#include <algorithm>
#include <iostream>
#include <vector>

// 模範解答
class SolutionAns {
    public:
        // ?
        int maxProfit(std::vector<int>& prices) {
            int sold = 0;
            int hold = -(1 << 30);
            int rest = 0;

            for (int i = 0; i < prices.size(); i++) {
                int prevSold = sold;
                sold = hold + prices[i];
                hold = std::max(hold, rest - prices[i]);
                rest = std::max(rest, prevSold);
            }

            return std::max(sold, rest);
        }

        // Rustの模範解答より
        int maxProfit2(std::vector<int>& prices) {
            int p1 = 0;
            int p2 = 0;

            for(int i = 1; i < prices.size(); ++i) {
                auto tmp = p1;

                p1 = std::max(p1 + prices[i] - prices[i - 1], p2);
                p2 = std::max(tmp, p2);
            }

            return std::max(p1, p2);
        }

};

int main(void) {
    auto case_1 = std::vector{1, 2, 3, 0, 2};
    // 3
    auto case_2= std::vector{1};
    // 0

    SolutionAns s_ans;

    std::cout << s_ans.maxProfit(case_1) << std::endl;
    std::cout << s_ans.maxProfit(case_2) << std::endl;

    std::cout << s_ans.maxProfit2(case_1) << std::endl;
    std::cout << s_ans.maxProfit2(case_2) << std::endl;
}
