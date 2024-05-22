#include <algorithm>
#include <iostream>
#include <tuple>
#include<vector>

class SolutionAnsKotlin {
    public:
        // AC
        std::vector<int> missingRolls(std::vector<int>& rolls, int mean, int n) {
            int m = rolls.size();
            int m_total = 0; 
            std::for_each(rolls.begin(), rolls.end(), [&](int x) {m_total += x;});
            int n_total = (mean * (m + n)) - m_total;
            if(n_total < n || n_total > n * 6) {
                return std::vector<int>{};
            }

            std::vector<int> result;
            while(n_total > 0) {
                int dice = std::min(n_total - n + 1, 6);
                result.push_back(dice);
                n_total -= dice;
                --n;
            }

            return result;
        }
};

void show_result(std::vector<int>& result) {
    for(auto& v: result) {
        std::cout << v << " ";
    }
    std::cout << '\n';
}

int main(void) {
    using Case = std::tuple<std::vector<int>, int, int>;
    Case case_1 = {{3, 2, 4, 3}, 4, 2};
    // [6, 6}
    Case case_2 = {{1, 5, 6}, 3, 4};
    // [2, 3, 2, 2}
    Case case_3 = {{1, 2, 3, 4}, 6, 4};
    // []

    SolutionAnsKotlin s_ans_kt;

    auto res_1 = s_ans_kt.missingRolls(std::get<0>(case_1), std::get<1>(case_1), std::get<2>(case_1));
    auto res_2 = s_ans_kt.missingRolls(std::get<0>(case_2), std::get<1>(case_2), std::get<2>(case_2));
    auto res_3 = s_ans_kt.missingRolls(std::get<0>(case_3), std::get<1>(case_3), std::get<2>(case_3));

    show_result(res_1);
    show_result(res_2);
    show_result(res_3);
}
