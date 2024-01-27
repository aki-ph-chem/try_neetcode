#include <cmath>
#include <iostream>
#include <vector>
#include <algorithm>

// 模範解答
class SolutionAns {
    public:
        std::vector<std::vector<int>> merge(std::vector<std::vector<int>>& intervals) {
            int n = intervals.size();
            if (n == 1) {
                return intervals;
            }

            // 左側でsort
            std::sort(intervals.begin(), intervals.end(), [](const auto& a, const auto& b) {
                    return a[0] < b[0]; 
                    });

            std::vector<std::vector<int>> result;

            int i = 0;
            while (i < n - 1) {
                // i番目の右がi+1番目の左よりも大きい(範囲がオーバーラップしている)
                if (intervals[i][1] >= intervals[i+1][0]) {
                    //i+1番目の左をi番目の左に交換
                    intervals[i+1][0] = intervals[i][0];
                    //i+1番目の右を i,i+1番目のうち大きい方に
                    intervals[i+1][1] = std::max(intervals[i][1], intervals[i+1][1]);
                } else {
                    result.push_back(intervals[i]);
                }
                i++;
            }
            result.push_back(intervals[i]);

            return result;
        }
};

void show_res(const std::vector<std::vector<int>>& result) {
    for(auto v: result) {
        for(auto w: v) {
            std::cout << w << " ";
        }
        std::cout << ":";
    }
    std::cout << std::endl;
}

int main(void) {
    SolutionAns s_ans;

    auto case_1 = std::vector{std::vector{1,3}, std::vector{2,6}, std::vector{8,10}, std::vector{15,18}};
    // => [[1,6],[8,10],[15,18]]
    auto case_2 = std::vector{std::vector{1,4}, std::vector{4,5}};
    // => [[1,5]]
    auto case_3 = std::vector{std::vector{1,4}, std::vector{5,6}};
    // => [[1,4],[5,6]]

    auto res_1 = s_ans.merge(case_1);
    show_res(res_1);
    auto res_2 = s_ans.merge(case_2);
    show_res(res_2);
    auto res_3 = s_ans.merge(case_3);
    show_res(res_3);
}
