#include <csignal>
#include <iostream>
#include <iterator>
#include <vector>
#include <algorithm>

//#define DEBUG_P
class Solution {
    public:
        int car_fleet(int target, std::vector<int>& position, std::vector<int>& speed) {
            std::vector<std::pair<double, double>> p_v_pair;
            for(std::size_t i = 0; i < position.size(); ++i) {
                p_v_pair.push_back({static_cast<double>(position[i]), static_cast<double>(speed[i])}); }
#ifdef DEBUG_P
            std::cout << "before sort" << std::endl;
            for(const auto &[p, v]: p_v_pair) {
                std::cout << "(" << p << "," << v << ")" << std::endl;
            }
#endif
            // positionでsort
            std::sort(p_v_pair.begin(), p_v_pair.end());
#ifdef DEBUG_P
            std::cout << "after sort" << std::endl;
            for(const auto &[p, v]: p_v_pair) {
                std::cout << "(" << p << "," << v << ")" << std::endl;
            }
#endif
            std::vector<double> stack;
            for(auto iter = std::rbegin(p_v_pair); iter != std::rend(p_v_pair); ++iter) {
                stack.push_back((static_cast<double>(target) - iter->first) / iter->second);
                if(stack.size() >= 2 && stack.back() <= stack[stack.size() - 2]) {
                    stack.pop_back();
                }
            }

            return static_cast<int>(stack.size());
        }
};

// 模範解答
class SolutionAns {
    public:
        int carFleet(int target, std::vector<int>& position, std::vector<int>& speed) {
            int n = position.size();

            std::vector<std::pair<int, double>> cars;
            for (int i = 0; i < n; i++) {
                double time = (double) (target - position[i]) / speed[i];
                cars.push_back({position[i], time});
            }
            sort(cars.begin(), cars.end());

            double maxTime = 0.0;
            int result = 0;

            for (int i = n - 1; i >= 0; i--) {
                double time = cars[i].second;
                if (time > maxTime) {
                    maxTime = time;
                    result++;
                }
            }

            return result;
        }
};

struct Casess {
    int target;
    std::vector<int> position;
    std::vector<int> speed;

    Casess(int target, std::vector<int>&& position, std::vector<int>&& speed)
        :target(target),position(position), speed(speed)
    {}
};

int main(void) {
    Casess case_1(12, std::vector{10, 8, 0, 5, 3}, std::vector{2, 4, 1, 1, 3}); // 3
    Casess case_2(10, std::vector{3}, std::vector{3}); // 1
    Casess case_3(100, std::vector{0, 2, 4}, std::vector{4, 2, 1}); // 1

    // case_1
    // delta {2, 4, 12, 7, 9} => delta time {1, 1, 12, 7, 3} => sort by position {12, 3, 7, 1, 1}
    // case_3
    // delta {100, 98, 96} => delta time {25, 49, 96} => sort by position {25, 49, 96}
    Solution s_1;
    std::cout << s_1.car_fleet(case_1.target, case_1.position, case_1.speed) << std::endl;
    std::cout << s_1.car_fleet(case_2.target, case_2.position, case_2.speed) << std::endl;
    std::cout << s_1.car_fleet(case_3.target, case_3.position, case_3.speed) << std::endl;

    SolutionAns s_2;
    std::cout << s_2.carFleet(case_1.target, case_1.position, case_1.speed) << std::endl;
    std::cout << s_2.carFleet(case_2.target, case_2.position, case_2.speed) << std::endl;
    std::cout << s_2.carFleet(case_3.target, case_3.position, case_3.speed) << std::endl;
}
