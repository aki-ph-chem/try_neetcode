#include <iostream>
#include <vector>
#include <algorithm>

// Rustの模範解答より
class SolutionAnsRust {
    public:
        // AC
        std::vector<int> addToArrayForm(std::vector<int> num, int k) {
            std::reverse(num.begin(), num.end());
            auto i = 0;

            while(k > 0) {
                auto digit = k % 10;

                if(i < num.size()) {
                    num[i] += digit;
                } else {
                    num.push_back(digit);
                }

                auto carry = num[i] / 10;
                num[i] %= 10;

                k /= 10;
                k += carry;
                ++i;
            }

            std::reverse(num.begin(), num.end());
            return num;
        }
};

void show_result(std::vector<int>& result) {
    for(auto&v: result) {
        std::cout << v << " ";
    }
    std::cout << std::endl;
}

int main(void) {
    using Case = std::pair<std::vector<int> ,int>;
    Case case_1 = {{1, 2, 0, 0}, 34};
    // => [1,2,3,4]
    Case case_2 = {{2, 7, 4}, 181};
    // => [4,5,5]
    Case case_3 = {{2, 1, 5}, 806};
    // => [1,0,2,1]

    SolutionAnsRust s_ans_rs;
    auto res_1_ans_rs = s_ans_rs.addToArrayForm(case_1.first, case_1.second);
    show_result(res_1_ans_rs);

    auto res_2_ans_rs = s_ans_rs.addToArrayForm(case_2.first, case_2.second);
    show_result(res_2_ans_rs);

    auto res_3_ans_rs = s_ans_rs.addToArrayForm(case_3.first, case_3.second);
    show_result(res_3_ans_rs);
}
