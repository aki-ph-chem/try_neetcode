#include <iostream>
#include <algorithm>
#include <vector>

// AC
class Solution {
    public:
        int minCostConnectPoints(std::vector<std::vector<int>>& points) {
            auto l_1_norm = [](auto& p_1, auto& p_2) {return std::abs(p_1[0] - p_2[0]) + std::abs(p_1[1] - p_2[1]);};
            // 重み付きグラフの構築
            auto n = (int)points.size();
            std::vector<std::pair<int, std::pair<int,int>>> graph_w;
            for(int i = 0; i < n; ++i) {
                for(int j = i + 1; j < n; ++j) {
                    graph_w.push_back(std::pair(l_1_norm(points[i], points[j]), std::pair(i, j)));
                }
            }

            // 重みでsort
            std::sort(graph_w.begin(), graph_w.end());

            // クラスカル法
            int res = 0;
            std::vector<int> uf(n, -1);
            std::vector<int> graph_size(n, 1);
            for(auto& e: graph_w) {
                auto [w, u, v] = std::tuple(e.first, e.second.first, e.second.second);

                if(!unite(u, v, uf, graph_size)) {
                    continue;
                }

                res += w;
            }

            return res;
        }

    private:
        // Union-Findの実装
        int root(int x, std::vector<int>& uf) {
            if(uf[x] == -1) {
                return x;
            }
            uf[x] = root(uf[x], uf);

            return uf[x];
        }

        bool unite(int x, int y, std::vector<int>& uf, std::vector<int>& graph_size) {
            x = root(x, uf);
            y = root(y, uf);

            if(x == y) {
                return false;
            }

            if(graph_size[x] < graph_size[y]) {
                std::swap(x, y);
            }
            uf[y] = x;
            graph_size[x] += graph_size[y];

            return true;
        }
};

int main(void) {
    std::vector<std::vector<int>> case_1 = {{0, 0}, {2, 2}, {3, 10}, {5, 2}, {7, 10}};
    // => 20
    std::vector<std::vector<int>> case_2 = {{3, 12}, {-2, 5}, {-4, 1}};
    // => 18

    Solution s_1;
    std::cout << s_1.minCostConnectPoints(case_1) << std::endl;
    std::cout << s_1.minCostConnectPoints(case_2) << std::endl;
}
