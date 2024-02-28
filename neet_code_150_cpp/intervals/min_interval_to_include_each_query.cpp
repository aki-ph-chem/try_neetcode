#include <iostream>
#include <unordered_map>
#include <vector>
#include <queue>
#include <algorithm>

// 模範解答
class SolutionAns {
    public:
        std::vector<int> minInterval(std::vector<std::vector<int>>& intervals, std::vector<int>& queries) {
            std::vector<int> sortedQueries = queries;

            std::priority_queue<std::pair<int, int>,
                std::vector<std::pair<int, int>>,
                std::greater<std::pair<int, int>>> pq;

            std::unordered_map<int,int> m;

            std::sort(intervals.begin(), intervals.end());
            std::sort(sortedQueries.begin(), sortedQueries.end());

            std::vector<int> result;

            int i = 0;
            for(auto& query: sortedQueries) {
                while(i < intervals.size() && intervals[i][0] <= query) {
                    auto [left, right] = std::pair(intervals[i][0], intervals[i][1]);
                    pq.push({right - left + 1, right});
                    ++i;
                }

                while(!pq.empty() && pq.top().second < query) {
                    pq.pop();
                }

                if(!pq.empty()) {
                    m[query] = pq.top().first;
                } else {
                    m[query] = -1;
                }
            }

            for(auto& q: queries) {
                result.push_back(m[q]);
            }

            return result;
        }
};

void show_result(std::vector<int>& result) {
    for(auto&v: result) {
        std::cout << v << " "; 
    }
    std::cout << '\n';
}

int main(void) {
    using Case = std::pair<std::vector<std::vector<int>>, std::vector<int>>;
    Case case_1 = {
        {{1,4},{2,4},{3,6},{4,4}},
        {2,3,4,5}
    };
    // => [3,3,1,4]
    Case case_2 = {
        {{2,3},{2,5},{1,8},{20,25}},
        {2,19,5,22}
    };
    // => [2,-1,4,6]

    SolutionAns s_ans;
    auto res_1_ans = s_ans.minInterval(case_1.first, case_1.second);
    auto res_2_ans = s_ans.minInterval(case_2.first, case_2.second);
    show_result(res_1_ans);
    show_result(res_2_ans);
}
