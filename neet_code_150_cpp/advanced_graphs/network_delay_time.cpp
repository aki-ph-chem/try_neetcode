#include <iostream>
#include <vector>
#include <climits>
#include <queue>
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
        
        // AC
        // ヒープを用いたダイクストラの実装を用いる
        int networkDelayTimeHeap(std::vector<std::vector<int>>& times, int n, int k) {
            constexpr int INF = INT_MAX; 
            // グラフの構築
            std::vector<std::vector<std::pair<int,int>>> graph_w(n + 1);
            for(auto& v_w: times) {
                auto [from, to, w] = std::tuple(v_w[0], v_w[1], v_w[2]);
                graph_w[from].push_back({to, w});
            }

            std::vector<int> dist(n + 1, INF);
            // スタート地点kへの距離を0に
            dist[k] = 0;
            std::priority_queue<std::pair<int, int>,
                std::vector<std::pair<int, int>>,
                std::greater<std::pair<int, int>>> que;
            que.push(std::pair(dist[k], k));

            while(!que.empty()) {
                auto [v, d] = std::pair(que.top().second, que.top().first);
                que.pop();
                // dが"ゴミ"か否か
                if(d > dist[v]) continue;
                // 緩和処理
                for(auto& e: graph_w[v]) {
                    if(chmin(dist[e.first], dist[v] + e.second)) {
                        // 更新された場合はヒープに値を追加
                        que.push(std::pair(dist[e.first], e.first));
                    }
                }
            }
#ifdef DEBUG
            for(int v = 1; v < n + 1; ++v) {
                if(dist[v] < INF) {
                    std::cout << dist[v] << '\n';
                } else {
                    std::cout << "INF" << '\n';
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

// 模範解答
class SolutionAns {
    public:
        int networkDelayTime(std::vector<std::vector<int>>& times, int n, int k) {
            // グラフの構築
            std::vector<std::vector<std::pair<int, int>>> adj(n + 1);
            for(auto& t: times) {
                auto [from, to, time] = std::tuple(t[0], t[1], t[2]);
                adj[from].push_back({time, to});
            }

            std::vector<int> signalReceiveTime(n + 1, INT_MAX);
            std::priority_queue<std::pair<int, int>,
                std::vector<std::pair<int, int>>,
                std::greater<std::pair<int,int>>> pq;
            pq.push({0, k});

            // time for start node is 0
            signalReceiveTime[k] = 0;

            while(!pq.empty()) {
                auto [currentNodeTime, currentNode] 
                    = std::pair(pq.top().first, pq.top().second);
                pq.pop();

                if(currentNodeTime > signalReceiveTime[currentNode]) {
                    continue;
                }

                // send signal to adjacent nodes
                for(auto& edge: adj[currentNode]) {
                    auto [time, neighborNode] 
                        = std::pair(edge.first, edge.second);

                    if(signalReceiveTime[neighborNode] > currentNodeTime + time) {
                        signalReceiveTime[neighborNode] = currentNodeTime + time;
                        pq.push({signalReceiveTime[neighborNode], neighborNode});
                    }
                }
            }

#ifdef DEBUG
            std::cout <<"signalReceiveTime" << std::endl;
            for(int i = 1; i <= n; ++i) {
                if(signalReceiveTime[i] < INT_MAX) {
                    std::cout << signalReceiveTime[i] << '\n';
                } else {
                    std::cout << "INF" << '\n';
                }
            }
#endif

            int result = INT_MIN;
            for(int i = 1; i <= n; ++i) {
                result = std::max(result, signalReceiveTime[i]);
            }

            if(result == INT_MAX) {
                return -1;
            }


            return result;
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

    std::cout << s_1.networkDelayTimeHeap(std::get<0>(case_1), std::get<1>(case_1), std::get<2>(case_1)) << std::endl;
    std::cout << s_1.networkDelayTimeHeap(std::get<0>(case_2), std::get<1>(case_2), std::get<2>(case_2)) << std::endl;
    std::cout << s_1.networkDelayTimeHeap(std::get<0>(case_3), std::get<1>(case_3), std::get<2>(case_3)) << std::endl;

    SolutionAns s_ans;
    std::cout << s_ans.networkDelayTime(std::get<0>(case_1), std::get<1>(case_1), std::get<2>(case_1)) << std::endl;
    std::cout << s_ans.networkDelayTime(std::get<0>(case_2), std::get<1>(case_2), std::get<2>(case_2)) << std::endl;
    std::cout << s_ans.networkDelayTime(std::get<0>(case_3), std::get<1>(case_3), std::get<2>(case_3)) << std::endl;
}
