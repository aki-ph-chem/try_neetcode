#include <algorithm>
#include <iostream>
#include <vector>
#include <string>
#include <queue>
#include <stack>
#include <set>
#include <unordered_map>

// 模範解答
class SolutionAns {
    public:
        std::vector<std::string> findItinerary(std::vector<std::vector<std::string>>& tickets) {
            std::unordered_map<std::string, std::multiset<std::string>> map;
            for(auto& tkt: tickets) {
                map[tkt[0]].insert(tkt[1]);
            }

            std::vector<std::string> result;
            dfs(map, "JFK", result);
            std::reverse(result.begin(), result.end());

            return result;
        }

    private:
        void dfs(std::unordered_map<std::string, std::multiset<std::string>>& map,
                std::string airport,
                std::vector<std::string>& result
                ) {
            while(!map[airport].empty()) {
                auto next = *(map[airport].begin());
                map[airport].erase(map[airport].begin());
                dfs(map, next, result);
            }

            result.push_back(airport);
        }
};

// AC
// std::multiset<T>ではなくstd::priority_queue<T>を使う
class SolutionAns2 {
    public:
        std::vector<std::string> findItinerary(std::vector<std::vector<std::string>>& tickets) {
            std::unordered_map<std::string, 
                std::priority_queue<std::string, 
                std::vector<std::string>, 
                std::greater<std::string>>> map;
            for(auto& tkt: tickets) {
                map[tkt[0]].push(tkt[1]);
            }

            std::vector<std::string> result;
            dfs(map, "JFK", result);
            std::reverse(result.begin(), result.end());

            return result;
        }

    private:
        void dfs(std::unordered_map<std::string, std::priority_queue<std::string, 
                std::vector<std::string>, std::greater<std::string>>>& map,
                std::string airport,
                std::vector<std::string>& result
                ) {
            while(!map[airport].empty()) {
                auto next = map[airport].top();
                map[airport].pop();
                dfs(map, next, result);
            }

            result.push_back(airport);
        }
};

// AC
// Rustの模範解答より
// stackを使ったDFS
class SolutionAnsRust {
    public:
        std::vector<std::string> findItinerary(std::vector<std::vector<std::string>>& tickets) {
            std::unordered_map<std::string, 
                std::priority_queue<std::string, std::vector<std::string>, 
                std::greater<std::string>>> graph;
            for(auto& tkt: tickets) {
                graph[tkt[0]].push(tkt[1]);
            }

            std::vector<std::string> result;
            std::stack<std::string> stack;
            stack.push("JFK");

            while(!stack.empty()) {
                auto src = stack.top();
                if(graph.find(src) != graph.end()) {

                    if(!graph[src].empty()) {
                        stack.push(graph[src].top());
                        graph[src].pop();
                        continue;
                    }
                }

                result.push_back(stack.top());
                stack.pop();
            }

            std::reverse(result.begin(), result.end());
            return result;
        }
};

void show_result(const std::vector<std::string>& result) {
    for(auto& v: result) {
        std::cout << v << " ";
    }
    std::cout << '\n';
}

int main(void) {
    using Case = std::vector<std::vector<std::string>>;
    Case case_1 = {
        {"MUC", "LHR"},
        {"JFK", "MUC"},
        {"SFO", "SJC"},
        {"LHR", "SFO"},
    };
    // => ["JFK","MUC","LHR","SFO","SJC"}
    Case case_2 = {
        {"JFK", "SFO"},
        {"JFK", "ATL"},
        {"SFO", "ATL"},
        {"ATL", "JFK"},
        {"ATL", "SFO"},
    };
    Case case_3 = {
        {"MUC","LHR"},
        {"JFK","MUC"},
        {"SFO","SJC"},
        {"LHR","SFO"}
    };

    SolutionAns s_ans;
    std::cout << "s_ans" << std::endl;
    auto res_ans1_1 = s_ans.findItinerary(case_1);
    auto res_ans1_2 = s_ans.findItinerary(case_2);
    show_result(res_ans1_1);
    show_result(res_ans1_2);

    SolutionAns2 s_ans2;
    std::cout << "s_ans_2" << std::endl;
    auto res_ans2_1 = s_ans2.findItinerary(case_1);
    auto res_ans2_2 = s_ans2.findItinerary(case_2);
    auto res_ans2_3 = s_ans2.findItinerary(case_3);
    show_result(res_ans2_1);
    show_result(res_ans2_2);
    show_result(res_ans2_3);

    SolutionAnsRust s_ans_rs;
    std::cout << "s_ans_rs" << std::endl;
    auto res_ans_rs_1 = s_ans_rs.findItinerary(case_1);
    auto res_ans_rs_2 = s_ans_rs.findItinerary(case_2);
    auto res_ans_rs_3 = s_ans_rs.findItinerary(case_3);
    show_result(res_ans_rs_1);
    show_result(res_ans_rs_2);
    show_result(res_ans_rs_3);
}
