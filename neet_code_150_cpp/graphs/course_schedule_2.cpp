#include <iostream>
#include <vector>
#include <unordered_map>
#include <unordered_set>

// 模範解答 
class SolutionAns {
    public:
        std::vector<int> findOrder(int numCourses, std::vector<std::vector<int>>& prerequisites) {
            std::unordered_map<int, std::vector<int>> map;
            for(auto& v: prerequisites) {
                map[v[0]].push_back(v[1]);
            }
            std::unordered_set<int> visit, cycle;

            std::vector<int> result;
            for(int course = 0; course < numCourses; ++course) {
                if(!dfs(course, map, visit, cycle, result)) {
                    return {};
                }
            }

            return result;
        }

    private:
        bool dfs(int course, 
                std::unordered_map<int, std::vector<int>>& map, 
                std::unordered_set<int>& visit, 
                std::unordered_set<int>& cycle, 
                std::vector<int>& result) {

            if(cycle.find(course) != cycle.end()) {
                return false;
            }

            if(visit.find(course) != visit.end()) {
                return true;
            }

            cycle.insert(course);

            for(auto& next_course: map[course]) {
                if(!dfs(next_course, map, visit, cycle, result)) {
                    return false;
                }
            }

            cycle.erase(course);
            visit.insert(course);
            result.push_back(course);

            return true;
        }
};

void show_result(const std::vector<int>& result) {
        for(auto& w: result) {
            std::cout << w << " ";
        }

        std::cout << '\n';
}

int main(void) {
    std::pair<int, std::vector<std::vector<int>>> case_1 = {2, {{1, 0}}};
    // => [0, 1]
    std::pair<int, std::vector<std::vector<int>>> case_2 = {4, {{1, 0}, {2, 0}, {3, 1}, {3, 2}}};
    // => [0,1,2,3] or [0,2,1,3]

    SolutionAns s_ans;

    auto res_1 = s_ans.findOrder(case_1.first, case_1.second);
    auto res_2 = s_ans.findOrder(case_2.first, case_2.second);

    show_result(res_1);
    show_result(res_2);
}
