#include <iostream>
#include <vector>

// AC
class Solution {
    public:
        int maxAreaOfIsland(std::vector<std::vector<int>> grid) {
            auto [m, n] = std::pair((int)grid.size(), (int)grid[0].size());
            auto [result, result_prev] = std::pair(0, 0);

            for(int x = 0; x < m; ++x) {
                for(int y = 0; y < n; ++y) {
                    if(grid[x][y] == 1) {
                        result_prev = result;
                        result = 0;
                        dfs(grid, x, y, result);
                    }

                    result = std::max(result, result_prev);
                }
            }

            return result;
        }

    private:
        void dfs(std::vector<std::vector<int>>& grid, int x, int y, int& result) {
            int m = grid.size();
            int n = grid[0].size();

            // 境界チェック
            if(x < 0 
                    || y < 0
                    || x >= m
                    || y >= n
                    || grid[x][y] == 0
              ) 
            {
                return;
            }

            // 面積をインクリメント
            ++result;

            // 訪問済み
            grid[x][y] = 0;
            // 十字方向の再帰的な探索
            std::vector<std::pair<int, int>> directions = {{0, 1}, {1, 0}, {0, -1} ,{-1, 0}};
            for(auto& v: directions) {
                dfs(grid, x + v.first, y + v.second, result);
            }

        }
};

// 模範解答
class SolutionAns {
    public:
        int maxAreaOfIsland(std::vector<std::vector<int>> grid) {
            int m = grid.size();
            int n = grid[0].size();

            int result = 0;

            for (int i = 0; i < m; i++) {
                for (int j = 0; j < n; j++) {
                    if (grid[i][j] == 1) {
                        result = std::max(result, dfs(grid, i, j, m, n));
                    }
                }
            }

            return result;
        }

    private:
        int dfs(std::vector<std::vector<int>>& grid, int i, int j, int m, int n) {
            if (i < 0 || i >= m || j < 0 || j >= n || grid[i][j] == 0) {
                return 0;
            }
            grid[i][j] = 0;

            return 1 + dfs(grid, i - 1, j, m, n) + dfs(grid, i + 1, j, m, n)
                + dfs(grid, i, j - 1, m, n) + dfs(grid, i, j + 1, m, n);
        }
};

int main(void) {
    std::vector<std::vector<int>> case_1 = {
        {0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0},
        {0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0},
        {0, 1, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0},
        {0, 1, 0, 0, 1, 1, 0, 0, 1, 0, 1, 0, 0},
        {0, 1, 0, 0, 1, 1, 0, 0, 1, 1, 1, 0, 0},
        {0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0},
        {0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0},
        {0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0},
    };
    // => 6

    std::vector<std::vector<int>> case_2 = {{0, 0, 0, 0, 0, 0, 0, 0}};
    // => 0

    Solution s_1;
    std::cout << s_1.maxAreaOfIsland(case_1) << std::endl;
    std::cout << s_1.maxAreaOfIsland(case_2) << std::endl;

    SolutionAns s_ans;
    std::cout << s_ans.maxAreaOfIsland(case_1) << std::endl;
    std::cout << s_ans.maxAreaOfIsland(case_2) << std::endl;
}
