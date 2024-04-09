#include <iostream>
#include <vector>
#include <set>
#include <unordered_set>
#include <stack>

// Javaの模範解答より(DFS)
class SolutionAnsJava {
    public:
        // AC
        int minReorder(int n, std::vector<std::vector<int>>& connections) {
            std::vector<std::vector<int>> graph(n, std::vector<int>{});
            std::set<std::pair<int,int>> old_edges;
            std::unordered_set<int> visit;

            for(auto& edge: connections) {
                auto [from, to] = std::pair(edge[0], edge[1]);
                old_edges.insert({from, to});

                graph[from].push_back(to);
                graph[to].push_back(from);
            }

            int result = 0;
            dfs(0, -1, graph, old_edges, visit, result);

            return result - 1;
        }

    private:
        void dfs(
                int current_noce,
                int parent_node,
                std::vector<std::vector<int>>& graph,
                std::set<std::pair<int,int>>& old_edges,
                std::unordered_set<int>& visit,
                int& result
                ){
            if(visit.count(current_noce)) {
                return;
            }
            visit.insert(current_noce);

            if(!old_edges.count({current_noce, parent_node})) {
                ++result;
            }
            for(auto& child_node: graph[current_noce]) {
                dfs(child_node, current_noce, graph, old_edges, visit, result);
            }
        }
};

// 模範解答(BFS)
class SolutionAns {
    public:
        // AC
        int minReorder(int n, std::vector<std::vector<int>>& connections) {
            std::vector<std::vector<std::pair<int ,bool>>> graph(n, std::vector<std::pair<int,bool>>{});
            std::unordered_set<int> visted;

            for(auto& edge: connections) {
                auto [from, to] = std::pair(edge[0], edge[1]);
                graph[from].push_back({to, true});
                graph[to].push_back({from, false});
            }
            int result = 0;
            std::stack<int> stack;
            stack.push(0);
            visted.insert(0);

            while(!stack.empty()) {
                int u = stack.top();
                stack.pop();

                for(auto& v: graph[u]){
                    if(visted.count(v.first)) {
                        continue;
                    }
                    stack.push(v.first);
                    visted.insert(v.first);
                    if(v.second) {
                        ++result;
                    }
                }
            }

            return result;
        }
};

int main(void) {
    using Case = std::pair<int, std::vector<std::vector<int>>>;

    Case case_1 = {6, {{0,1},{1,3},{2,3},{4,0},{4,5}}};
    // => 3
    Case case_2 = {5, {{1,0},{1,2},{3,2},{3,4}}};
    // => 2 
    Case case_3 = {3, {{1,0}, {2,0}}};
    // => 0

    SolutionAnsJava s_ans_java;
    std::cout << s_ans_java.minReorder(case_1.first, case_1.second) << std::endl;
    std::cout << s_ans_java.minReorder(case_2.first, case_2.second) << std::endl;
    std::cout << s_ans_java.minReorder(case_3.first, case_3.second) << std::endl;

    SolutionAns s_ans;
    std::cout << s_ans.minReorder(case_1.first, case_1.second) << std::endl;
    std::cout << s_ans.minReorder(case_2.first, case_2.second) << std::endl;
    std::cout << s_ans.minReorder(case_3.first, case_3.second) << std::endl;
}
