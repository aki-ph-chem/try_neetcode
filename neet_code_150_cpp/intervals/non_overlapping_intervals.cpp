#include <iostream>
#include <vector>
#include <algorithm>

class SolutionAns {
public:
    int eraseOverlapIntervals(std::vector<std::vector<int>>& intervals) {
        int n = intervals.size();
        if (n == 1) {
            return 0;
        }
        
        std::sort(intervals.begin(), intervals.end(), [](const auto& a, const auto& b) {
            return a[0] < b[0];
        });
        
        int result = 0;
        
        int i = 0;
        while (i < n - 1) {
            if (intervals[i][1] > intervals[i+1][0]) {
                if (intervals[i][1] < intervals[i+1][1]) {
                    intervals[i+1] = intervals[i];
                }
                result++;
            }
            i++;
        }
        
        return result;
    }
};

int main(void) {
    auto case_1 = std::vector{std::vector{1, 2}, std::vector{2, 3}, std::vector{3, 4}, std::vector{1, 3}};
    // => 1
    auto case_1_2 = std::vector{std::vector{1, 2}, std::vector{1, 3}, std::vector{2, 3}, std::vector{3, 4}};
    // => 1
    auto case_2 = std::vector{std::vector{1, 2}, std::vector{1, 2}, std::vector{1, 2}};
    // => 2
    auto case_3 = std::vector{std::vector{1, 2}, std::vector{2, 3}};
    // => 0
    auto case_4 = std::vector{std::vector{1, 100}, std::vector{11, 22}, std::vector{1, 11}, std::vector{2, 12}};
    // => 2

    SolutionAns s_ans;

    std::cout << s_ans.eraseOverlapIntervals(case_1) << std::endl;
    std::cout << s_ans.eraseOverlapIntervals(case_1_2) << std::endl;
    std::cout << s_ans.eraseOverlapIntervals(case_2) << std::endl;
    std::cout << s_ans.eraseOverlapIntervals(case_3) << std::endl;
    std::cout << s_ans.eraseOverlapIntervals(case_4) << std::endl;
}
