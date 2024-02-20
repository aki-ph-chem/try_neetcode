#include <iostream>
#include <vector>
#include <queue>
#include <cmath>

class SolutionAns {
    public:
        std::vector<std::vector<int>> kClosest(std::vector<std::vector<int>>& points, int k) {
            std::priority_queue<std::tuple<int,int,int>,
                std::vector<std::tuple<int, int, int>>,
                std::greater<std::tuple<int,int,int>>> pts;

            for(auto& p: points) {
                auto dist = std::pow(p[0], 2) + std::pow(p[1], 2);
                pts.push({dist, p[0], p[1]});
            }

            std::vector<std::vector<int>> result;
            for(int i = 0; i < k; ++i) {
                if(!pts.empty()) {
                    auto [d, x, y] = pts.top();
                    pts.pop();
                    result.push_back(std::vector{x, y});
                }
            }

            return result;
        }
};

void show_result(std::vector<std::vector<int>>& result) {
    for(auto& v: result) {
        for(auto& w: v) {
            std::cout << w <<" ";
        }
        std::cout << '\n';
    }
}

int main(void) {
    using Case = std::pair<std::vector<std::vector<int>>, int>;

    Case case_1 = {{{1, 3}, {-2, 2}}, 1};
    Case case_2 = {{{3, 3}, {5, -1}, {-2, 4}}, 2};

    SolutionAns s_ans;
    auto res_1 = s_ans.kClosest(case_1.first, case_1.second);
    auto res_2 = s_ans.kClosest(case_2.first, case_2.second);
    show_result(res_1);
    show_result(res_2);
}
