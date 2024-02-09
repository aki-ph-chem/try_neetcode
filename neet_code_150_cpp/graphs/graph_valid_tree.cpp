#include <iostream>
#include <vector>
#include <unordered_map>

// AC
// Union-Findでサイクルがあるか否かの判定と連結成分の個数を調べる
class Solution {
    public:
        bool validTree(int n, std::vector<std::vector<int>>& edges) {
            std::vector<int> parents(n, -1);
            std::vector<int> size_graph(n, 1);

            // サイクルがあるかどうか 
            for(auto& v: edges) {
                auto n_1 = v[0];
                auto n_2 = v[1];

                if(!unite(n_1, n_2, parents, size_graph)) {
                    return false;
                }
            }

            // 連結成分が1以上か
            int num_conected = 0;
            for(int i = 0; i < n; ++i) {
                if(root(i, parents) == i) {
                    ++num_conected;
                }
            }

            if(num_conected > 1) {
                return false;
            }

            return true;
        }

    private:
        int root(int x, std::vector<int>& parents) {
            if(parents[x] == -1) {
                return x;
            } else {
                return parents[x] = root(parents[x], parents);
            }
        }

        bool unite(
                int x,
                int y,
                std::vector<int>& paretns,
                std::vector<int>& size_graph
                ) {
            x = root(x, paretns);
            y = root(y, paretns);

            if(x == y) {
                return false;
            }

            if(size_graph[x] < size_graph[y]) {
                std::swap(x, y);
            }

            paretns[y] = x;
            size_graph[x] += size_graph[y];

            return true;
        }
};

// 模範解答
class SolutionAns {
    public:
        bool validTree(int n, std::vector<std::vector<int>>& edges) {
            std::vector<std::vector<int>> adj(n);

            // グラフの構築
            for(auto& v: edges) {
                adj[v[0]].push_back(v[1]);
                adj[v[1]].push_back(v[0]);
            }

            std::vector<bool> visited(n);
            // adjの0から探索をスタート
            if(hasCycle(adj, visited, -1, 0)) {
                return false;
            }

            // つながっていない頂点は未訪問になっている
            for(auto x: visited) {
                if(!x) {
                    return false;
                }
            }

            return true;
        }

    private:
        bool hasCycle(
                std::vector<std::vector<int>>& adj, 
                std::vector<bool>& visited,
                int parent,
                int child
                ) {
            
            if(visited[child]) {
                return true;
            }

            visited[child] = true;

            // childから辿れる頂点を探索
            for(auto& v: adj[child]) {
                if(v != parent && hasCycle(adj, visited, child, v)) {
                    return true;
                }
            }

            return false;
        }
};

// AC
// std::unorderd_map<U, V> を用いた実装 
class Solution2 {
    public:
        bool validTree(int n, std::vector<std::vector<int>>& edges) {
            std::unordered_map<int, std::vector<int>> graph(n);
            // グラフの構築
            for(auto& v: edges) {
                graph[v[0]].push_back(v[1]);
                graph[v[1]].push_back(v[0]);
            }

            std::vector<bool> visited(n);
            // サイクルがあるか否かの判定
            if(hasCycle(graph, visited, -1, 0)) {
                return false;
            }

            // つながっていない頂点は未訪問になっている
            for(auto x: visited) {
                if(!x) {
                    return false;
                }
            }

            return true;
        }

    private:
        bool hasCycle(
                std::unordered_map<int, std::vector<int>>& graph,
                std::vector<bool>& visited,
                int parent,
                int child
                ) {

            if(visited[child]) {
                return true;
            }

            visited[child] = true;

            for(auto& v: graph[child]) {
                if(v != parent && hasCycle(graph, visited, child, v)) {
                    return true;
                }
            }

            return false;
        }
};

int main(void) {
    std::pair<int, std::vector<std::vector<int>>> case_1 = {5, {{0, 1}, {0, 2}, {0, 3}, {1, 4}}};
    // => true
    std::pair<int, std::vector<std::vector<int>>> case_2 = {5, {{0, 1}, {1, 2}, {2, 3}, {1, 3}, {1, 4}}};
    // => false
    std::pair<int, std::vector<std::vector<int>>> case_3 = {4, {{0, 1}, {2, 3}}};
    // => false

    Solution s_1;
    std::cout << s_1.validTree(case_1.first, case_1.second) << std::endl;
    std::cout << s_1.validTree(case_2.first, case_2.second) << std::endl;
    std::cout << s_1.validTree(case_3.first, case_3.second) << std::endl;

    SolutionAns s_ans;
    std::cout << s_ans.validTree(case_1.first, case_1.second) << std::endl;
    std::cout << s_ans.validTree(case_2.first, case_2.second) << std::endl;
    std::cout << s_ans.validTree(case_3.first, case_3.second) << std::endl;

    Solution2 s_2;
    std::cout << s_2.validTree(case_1.first, case_1.second) << std::endl;
    std::cout << s_2.validTree(case_2.first, case_2.second) << std::endl;
    std::cout << s_2.validTree(case_3.first, case_3.second) << std::endl;
}
