#include <iostream>
#include <vector>
//#define DEBUG

// AC
// ダイクストラでkからの最短距離を求めてその最短距離のうちの最大値を探せば良い
class Solution {
    public:
        int networkDelayTime(std::vector<std::vector<int>>& times, int n, int k) {
            constexpr int INF = 1 << 30; 
            std::vector<bool> used(n + 1, false);
            std::vector<int> dist(n + 1, INF);

            // グラフの構築
            std::vector<std::vector<std::pair<int,int>>> graph_w(n + 1);
            for(auto& v_w: times) {
                auto [from, to, w] = std::tuple(v_w[0], v_w[1], v_w[2]);
                graph_w[from].push_back({to, w});
            }

            // スタート地点kへの距離を0に
            dist[k] = 0;
            for(int iter = 0; iter < n; ++iter) {
                auto min_dist = INF;
                auto min_v = -1;
                for(int v = 1; v < n + 1; ++v) {
                    if(!used[v] && dist[v] < min_dist) {
                        min_dist = dist[v];
                        min_v = v;
                    }
                }

                if(min_v == -1) break;
                
                // min_vを始点とした緩和処理
                for(auto& e: graph_w[min_v]) {
                    chmin(dist[e.first], dist[min_v] + e.second);
                }

                used[min_v] = true;
            }

#ifdef DEBUG
            std::cout << "n = " << n << std::endl;
            for(int i = 1; i < n + 1; ++i) {
                if(dist[i] > -INF) {
                    std::cout << dist[i] << '\n'; 
                } else {
                    std::cout << "-INF" << '\n';
                }
            }
#endif
            int result = -INF;
            for(int i = 1; i < n + 1; ++i) {
                if(i == k) {
                    continue;
                } else if(dist[i] > result) {
                    result = dist[i];
                }

                if(dist[i] == INF) {
                    return -1;
                }
            }

            return result;
        }

    private:
        template<typename T>
            bool chmin(T& a,T b) {
                if(a > b) {
                    a = b;
                    return true;
                } else {
                    return false;
                }
            }
};

int main(void) {
    // {times, n, k}
    std::tuple<std::vector<std::vector<int>>,int,int> case_1 = {
        {{2,1,1}, {2,3,1}, {3,4,1}}, 4, 2};
    // => 2
    std::tuple<std::vector<std::vector<int>>,int,int> case_2 = {
        {{1,2,1}}, 2, 1};
    // => 1
    std::tuple<std::vector<std::vector<int>>,int,int> case_3 = {
        {{1,2,1}}, 2, 2};
    // => -1

    Solution s_1;
    std::cout << s_1.networkDelayTime(std::get<0>(case_1), std::get<1>(case_1), std::get<2>(case_1)) << std::endl;

    std::cout << s_1.networkDelayTime(std::get<0>(case_2), std::get<1>(case_2), std::get<2>(case_2)) << std::endl;

    std::cout << s_1.networkDelayTime(std::get<0>(case_3), std::get<1>(case_3), std::get<2>(case_3)) << std::endl;
}
