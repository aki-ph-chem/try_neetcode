#include <cwchar>
#include <iostream>
#include <vector>

class SolutionAnsRust {
    public:
        std::vector<std::vector<int>> insert(std::vector<std::vector<int>>& intervals, std::vector<int>& newInterval) {
            std::vector<std::vector<int>> res;

            for(int i = 0; i < intervals.size(); ++i) {
                if(newInterval[1] < intervals[i][0]) {
                    res.push_back(newInterval);
                    for(int j = i; j < intervals.size(); ++j) {
                        res.push_back(intervals[j]);
                    }

                    return res;
                } else if(intervals[i][1] < newInterval[0]) {
                    res.push_back(intervals[i]);
                } else {
                    newInterval = std::vector{
                        std::min(newInterval[0], intervals[i][0]),
                        std::max(newInterval[1], intervals[i][1]),
                    };
                }
            }

            res.push_back(newInterval);
            return res;
        }
};

// 模範解答
class SolutionAns {
public:
    std::vector<std::vector<int>> insert(std::vector<std::vector<int>>& intervals, std::vector<int>& newInterval) {
        std::vector<std::vector<int>> ans;
        int newStart = newInterval[0];
        int newEnd = newInterval[1];
        int n = intervals.size();

        for (int i = 0; i < n; i++) {
            // Case 1: Non overlapping interval
            // If new interval is before the current interval
            if (intervals[i][0] > newEnd) {
                ans.push_back(newInterval);
                copy(intervals.begin() + i, intervals.end(), back_inserter(ans));
                return ans;
            }
            // If new interval is after the current interval
            else if (intervals[i][1] < newStart) {
                ans.push_back(intervals[i]);
            }
            // Case 2: Overlapping interval
            else {
                newInterval[0] = std::min(newInterval[0], intervals[i][0]);
                newInterval[1] = std::max(newInterval[1], intervals[i][1]);
            }
        }

        ans.push_back(newInterval);
        return ans;
    }
};

int main(void) {
    auto case_1 = std::pair(
            std::vector{std::vector{1,3}, std::vector{6,9}}, 
            std::vector{2,5}
            );
    // => [[1, 5], [6, 9]]
    auto case_2 = std::pair(
            std::vector{std::vector{1,2}, std::vector{3, 5}, std::vector{6, 7}, std::vector{8, 10} , std::vector{12, 16}}, 
            std::vector{4,8}
            );
    // => [[1,2], [3, 10], [12, 16]]

    SolutionAnsRust s_ans_rs;
    auto res_1 = s_ans_rs.insert(case_1.first, case_1.second); 
    auto res_2 = s_ans_rs.insert(case_2.first, case_2.second); 

    for(auto v: res_1) {
        for(auto w: v) {
            std::cout << w << " ";
        }
        std::cout << ":";
    }
    std::cout << std::endl;
    for(auto v: res_2) {
        for(auto w: v) {
            std::cout << w << " ";
        }
        std::cout << ":";
    }
    std::cout << std::endl;

    SolutionAns s_ans;
    auto res_1_rs = s_ans.insert(case_1.first, case_1.second); 
    auto res_2_rs = s_ans.insert(case_2.first, case_2.second); 

    for(auto v: res_1_rs) {
        for(auto w: v) {
            std::cout << w << " ";
        }
        std::cout << ":";
    }
    std::cout << std::endl;
    for(auto v: res_2_rs) {
        for(auto w: v) {
            std::cout << w << " ";
        }
        std::cout << ":";
    }
    std::cout << std::endl;
}
