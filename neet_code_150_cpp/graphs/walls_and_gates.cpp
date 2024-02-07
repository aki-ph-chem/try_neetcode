#include <climits>
#include <iostream>
#include <utility>
#include <vector>
#include <queue>

// 解かなかった(セグフォを除けなかった)
class Solution {
    public:
        void islandAndTreasure(std::vector<std::vector<int>>& grid) {
            auto [m, n] = std::pair((int)grid.size(), (int)grid[0].size());

            for(int x = 0; x < m; ++x) {
                for(int y = 0; y < n; ++y) {
                    int step_to_treasure = 0;
                    if(grid[x][y] == INT_MAX) {
                        dfs(grid, x, y, step_to_treasure);
                        grid[x][y] = step_to_treasure;
                    }
                }
            }
        }

    private:
        void dfs(std::vector<std::vector<int>>& grid, int x, int y, int& step_to_treasure) {
            auto [m, n] = std::pair((int)grid.size(), (int)grid[0].size());

            // 境界チェック
            if (x < 0
                    || y < 0
                    || x >= m
                    || y >= n
                    || grid[x][y] == -1 
               ) 
            {
                step_to_treasure = INT_MAX;
                return;
            } 

            if (grid[x][y] == INT_MAX) {
                ++step_to_treasure;
            } else {
                ++step_to_treasure;
                return;
            }

            // 十字方向の再帰的な探索
            std::vector<std::pair<int, int>> directions = {{0, 1}, {1, 0}, {0, -1} ,{-1, 0}};
            for(auto& v: directions) {
                dfs(grid, x + v.first, y + v.second ,step_to_treasure);
            }
        }
};


// AC
// 模範解答
class SolutionAns {
    public:
        void wallsAndGate(std::vector<std::vector<int>>& grid) {
            auto m = (int)grid.size();
            auto n = (int)grid[0].size();

            // 値がゼロである場所をキューに記録しておく
            std::queue<std::pair<int ,int>> q;
            for(int i = 0 ; i < m; ++i) {
                for(int j = 0; j < n; ++j) {
                    if(grid[i][j] == 0) {
                        q.push({i, j});
                    }
                }
            }

            // BFS
            while(!q.empty()) {
                auto row = q.front().first;
                auto col = q.front().second;
                q.pop();

                // 十字方向
                std::vector<std::pair<int, int>> directions = {{0, 1}, {1, 0}, {0, -1} ,{-1, 0}};
                for(auto& v: directions) {
                    auto x = row + v.first;
                    auto y = col + v.second;

                    // 境界チェック
                    if(x < 0 ||
                            x >= m ||
                            y < 0 ||
                            y >= n ||
                            grid[x][y] != INT_MAX
                            ) 
                    {
                        continue;
                    }

                    // 前の値を使って更新
                    grid[x][y] = grid[row][col] + 1;
                    q.push({x, y});
                }
            }
        }
};

void show_result(std::vector<std::vector<int>>& result) {
    for(auto& v: result) {
        for(auto& w: v) {
            std::cout << w << " ";
        }
        std::cout << '\n';
    }
}

int main(void) {
    std::vector<std::vector<int>> case_1 = 
    {
        {2147483647,-1,0,2147483647},
        {2147483647,2147483647,2147483647,-1},
        {2147483647,-1,2147483647,-1},
        {0,-1,2147483647,2147483647}
    };
    // =>
    /*
       [[3,-1,0,1],
       [2,2,1,-1],
       [1,-1,2,-1],
       [0,-1,3,4]]
     */

    std::vector<std::vector<int>> case_2 = 
    {
        {0, -1},
        {2147483647, 2147483647}
    };
    // =>
    /*
       [[0,-1],
       [1,2]]
     */


    /*
    Solution s_1;

    auto res_1 = case_1;
    s_1.islandAndTreasure(res_1);
    show_result(res_1);
    */

    SolutionAns s_ans;
    auto res_1_ans = case_1;
    auto res_2_ans = case_2;
    s_ans.wallsAndGate(res_1_ans);
    s_ans.wallsAndGate(res_2_ans);

    show_result(res_1_ans);
    show_result(res_2_ans);
}
