#include <iostream>
#include <vector>
#include <queue>

// 模範解答
class SolutionAns {
    public:
        int swimInWater(std::vector<std::vector<int>>& grid) {
            auto n = (int)grid.size();
            if(n == 1) {
                return 0;
            }

            std::vector<std::vector<bool>> visited(n, std::vector<bool>(n));
            visited[0][0] = true;

            int result = std::max(grid[0][0], grid[n - 1][n - 1]);
            std::priority_queue<std::vector<int>, 
                std::vector<std::vector<int>>, 
                std::greater<std::vector<int>>> pq;
            pq.push({result, 0, 0});

            while(!pq.empty()) {
                std::vector<int> current = pq.top();
                pq.pop();

                result = std::max(result, current[0]);
                std::vector<std::pair<int, int>> directions
                    = {{1, 0}, {0, 1}, {-1, 0}, {0, -1} };
                for(auto& s: directions) {
                    auto [x ,y] 
                        = std::pair(current[1] + s.first, current[2] + s.second);

                    if(x < 0 || x >= n || y < 0 || y >= n || visited[x][y]) {
                        continue;
                    }

                    if(x == n - 1 && y == n - 1) {
                        return result;
                    }

                    pq.push({grid[x][y], x, y});
                    visited[x][y] = true;
                }

            }

            return -1;
        }

        // AC
        int swimInWaterTuple(std::vector<std::vector<int>>& grid) {
            auto n = (int)grid.size();
            if(n == 1) {
                return 0;
            }

            std::vector<std::vector<bool>> visited(n, std::vector<bool>(n));
            visited[0][0] = true;

            int result = std::max(grid[0][0], grid[n - 1][n - 1]);
            std::priority_queue<std::tuple<int, int, int>, 
                std::vector<std::tuple<int, int, int>>, 
                std::greater<std::tuple<int, int, int>>> pq;

            pq.push({result, 0, 0});

            while(!pq.empty()) {
                auto current = pq.top();
                pq.pop();

                result = std::max(result, std::get<0>(current));
                std::vector<std::pair<int, int>> directions
                    = {{1, 0}, {0, 1}, {-1, 0}, {0, -1} };
                for(auto& s: directions) {
                    auto [x ,y] 
                        = std::pair(std::get<1>(current) + s.first, std::get<2>(current) + s.second);

                    if(x < 0 || x >= n || y < 0 || y >= n || visited[x][y]) {
                        continue;
                    }

                    if(x == n - 1 && y == n - 1) {
                        return result;
                    }

                    pq.push({grid[x][y], x, y});
                    visited[x][y] = true;
                }

            }

            return -1;
        }
};

int main(void) {
    using Case = std::vector<std::vector<int>>;
    Case case_1 = {{0, 2}, {1, 3}};
    // => 3
    Case case_2 = {
        {0, 1, 2, 3, 4},
        {24, 23, 22, 21, 5},
        {12, 13, 14, 15, 16},
        {11, 17, 18, 19, 20},
        {10, 9, 8, 7, 6},
    };
    // => 16

    SolutionAns s_ans;
    std::cout << s_ans.swimInWater(case_1) << std::endl;
    std::cout << s_ans.swimInWater(case_2) << std::endl;

    std::cout << s_ans.swimInWaterTuple(case_1) << std::endl;
    std::cout << s_ans.swimInWaterTuple(case_2) << std::endl;
}
