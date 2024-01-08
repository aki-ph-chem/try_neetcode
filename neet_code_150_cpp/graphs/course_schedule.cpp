#include <iostream>
#include <vector>
#include <algorithm>
#include <unordered_set>
#include <unordered_map>

// 解けなかった
class Solution {
#define DEBUG
    using Graph = std::vector<std::vector<int>>;
    public:
        bool canFinish(int numCourse, std::vector<std::vector<int>>& prerequisites) {
            // グラフの構築
            Graph couse_graph(numCourse); 
            for(const auto &v: prerequisites) {
                couse_graph[v[1]].push_back(v[0]);
            }

            std::vector<bool> seen(numCourse, false);
            std::vector<int> order;
            for(int i = 0; i < numCourse; ++i) {
                if(seen[i]) {
                    continue;
                }
                dfs(couse_graph, i, seen, order);
            }
            std::reverse(order.begin(), order.end());

#ifdef DEBUG
            std::cout << "toological sort" << std::endl;
            for(auto &v: order) {
                std::cout << v << " ";
            }
            std::cout << std::endl;
#endif

            return false;
        }

    private:
        void dfs(Graph& couses, int v, std::vector<bool>& seen, std::vector<int>& order) {
            seen[v] = true;
            for(auto & next_v: couses[v]) {
                if(seen[next_v]) {
                    continue;
                }
                dfs(couses, next_v, seen, order);
            }
            order.push_back(v);
        }
};

// 模範解答 
class SolutionAns {
    public:
        bool canFinish(int numCourses, std::vector<std::vector<int>>& prerequisites) {
            // map each course to prereq list
            std::unordered_map<int, std::vector<int>> m;
            for (int i = 0; i < prerequisites.size(); i++) {
                m[prerequisites[i][0]].push_back(prerequisites[i][1]);
            }
            // all courses along current DFS path
            std::unordered_set<int> visited;

            for (int course = 0; course < numCourses; course++) {
                if (!dfs(course, m, visited)) {
                    return false;
                }
            }
            return true;
        }

    private:
        bool dfs(int course, std::unordered_map<int, std::vector<int>>& m, std::unordered_set<int>& visited) {
            if (visited.find(course) != visited.end()) {
                return false;
            }
            if (m[course].empty()) {
                return true;
            }
            visited.insert(course);
            for (int i = 0; i < m[course].size(); i++) {
                int nextCourse = m[course][i];
                if (!dfs(nextCourse, m, visited)) {
                    return false;
                }
            }
            m[course].clear();
            visited.erase(course);
            return true;
        }
};

int main(void) {
    auto case_1 = std::pair(2, std::vector(1, std::vector{1, 0}));
    // => true
    auto case_2 = std::pair(2, std::vector{std::vector{1, 0}, std::vector{0, 1}});
    // => false
    auto case_3 = std::pair(5, std::vector{std::vector{1, 0}, std::vector{2, 0}, std::vector{3, 1}, std::vector{4, 2}});
    // => true
    auto case_4 = std::pair(5, std::vector{std::vector{1, 0}, std::vector{2, 1}, std::vector{0, 2}, std::vector{3, 1}, std::vector{4, 2}});
    // => false 

    Solution s_1;
    /*
    std::cout << s_1.canFinish(case_1.first, case_1.second) << std::endl;
    std::cout << s_1.canFinish(case_2.first, case_2.second) << std::endl;
    */

    SolutionAns s_ans;
    std::cout << s_ans.canFinish(case_1.first, case_1.second) << std::endl;
    std::cout << s_ans.canFinish(case_2.first, case_2.second) << std::endl;
    std::cout << s_ans.canFinish(case_3.first, case_3.second) << std::endl;
    std::cout << s_ans.canFinish(case_4.first, case_4.second) << std::endl;
}
