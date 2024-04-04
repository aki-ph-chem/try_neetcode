#include <iostream>
#include <vector>

// AC
// Pythonの模範解答より
class SolutionAnsPython {
    public:
        int shipWithinDays(std::vector<int>& weights, int days) {
            // 重さ制限capdで日数daysで完了できるか
            auto can_ship = [&](int cap) {
                auto [ships, current_cap] = std::pair(1, cap);
                for(auto&w: weights) {
                    if(current_cap - w < 0) {
                        ++ships;
                        current_cap = cap;
                    }
                    current_cap -= w;
                }

                return ships <= days;
            };

            auto [left, right] = std::pair(0, 0);
            for(auto&v: weights) {
                left = std::max(left, v);
                right += v;
            }
            int result = right;

            while(left <= right) {
                int cap = left + (right - left) / 2;
                if(can_ship(cap)) {
                    result = std::min(result, cap);
                    right = cap - 1;
                } else {
                    left = cap + 1;
                }
            }

            return result;
        }
};

int main(void) {
    using Case = std::pair<std::vector<int>, int>;
    Case case_1 = {{1, 2, 3, 4, 5, 6, 7, 8, 9, 10}, 5};
    // => 15
    Case case_2 = {{3, 2, 2, 4, 1, 4}, 3};
    // => 6
    Case case_3 = {{1, 2, 3, 1, 1}, 4};
    // => 3

    SolutionAnsPython s_ans_py;

    std::cout << s_ans_py.shipWithinDays(case_1.first, case_1.second) << std::endl;
    std::cout << s_ans_py.shipWithinDays(case_2.first, case_2.second) << std::endl;
    std::cout << s_ans_py.shipWithinDays(case_3.first, case_3.second) << std::endl;
}
