#include <climits>
#include <iostream>
#include <vector>
#include <queue>

// 模範解答
class SolutionAns {
    public:
    int findCheapestPrice(int n, std::vector<std::vector<int>>& flights, int src, int dst, int k) {
        std::vector<std::vector<int>> adj(n, std::vector<int>(n, 0));
        for(auto& edge: flights) {
            auto [from, to, price] = std::tuple(edge[0], edge[1], edge[2]);
            adj[from][to] = price;
        }

        std::vector<int> dist(n, INT_MAX);
        dist[src] = 0;
        std::vector<int> currentStops(n, INT_MAX);
        currentStops[src] = 0;

        // (cost, node, stops)
        std::priority_queue<std::vector<int>, 
            std::vector<std::vector<int>>,
            std::greater<std::vector<int>>> pq;
        pq.push({0, src, 0});

        while(!pq.empty()) {
            auto [cost, node, stops] 
                = std::tuple(pq.top()[0], pq.top()[1], pq.top()[2]);
            pq.pop();

            if(node == dst) {
                return cost;
            }

            if(stops == k + 1) {
                continue;
            }

            for(int neighbor = 0; neighbor < n; ++neighbor) {
                if(adj[node][neighbor] > 0) {
                    auto [currentCost, neighborDist, neighborWeigght]
                        = std::tuple(cost, dist[neighbor], adj[node][neighbor]);

                    int currentDist = currentCost + neighborWeigght;
                    if(currentDist < neighborDist || stops + 1 < currentStops[neighbor]) {
                        pq.push({currentDist, neighbor, stops + 1});
                        dist[neighbor] = currentDist;
                        currentStops[neighbor] = stops;
                    } else if (stops < currentStops[neighbor]) {
                        pq.push({currentDist, neighbor, stops + 1});
                    }
                    currentStops[neighbor] = stops;
                }
            }
        }


        if(dist[dst] == INT_MAX) {
            return -1;
        }

        return dist[dst];
    }
};

int main(void) {
    using Case = std::tuple<int, std::vector<std::vector<int>>, int, int, int>;

    Case case_1 = {
        3,
        {{0, 1, 100}, {1, 2, 100}, {0, 2, 500}},
        0,
        2,
        1,
    };
    // => 200
    Case case_2 = {
        3,
        {{0, 1, 100}, {1, 2, 100}, {0, 2, 500}},
        0,
        2,
        0,
    };
    // => 500

    SolutionAns s_ans;

    std::cout << "case_1 " 
        << s_ans.findCheapestPrice(
                std::get<0>(case_1), 
                std::get<1>(case_1), 
                std::get<2>(case_1),
                std::get<3>(case_1),
                std::get<4>(case_1)) << std::endl;

    std::cout << "case_2 " 
        << s_ans.findCheapestPrice(
                std::get<0>(case_2), 
                std::get<1>(case_2), 
                std::get<2>(case_2),
                std::get<3>(case_2),
                std::get<4>(case_2)) << std::endl;
}
