#include <iostream>
#include <vector>
#define FOR_DEBUG

// 模範解答
class SolutionAns {
    public:
        std::vector<std::vector<int>> pacificAtlantic(std::vector<std::vector<int>>& heights) {
            int m = heights.size();
            int n = heights[0].size();

            std::vector<std::vector<bool>> pacific(m, std::vector<bool>(n));
            std::vector<std::vector<bool>> atlantic(m, std::vector<bool>(n));

            // 左端、右端、上端、下端からそれぞれ山を再帰的に登る
            //
            for (int i = 0; i < m; i++) {
                // (i, 0)から再帰的に探索
                // 左端からスタート
                dfs(heights, pacific, i, 0, m, n);
                // (i, n - 1)から再帰的に探索
                // 右端からスタート
                dfs(heights, atlantic, i, n - 1, m, n);
            }

            for (int j = 0; j < n; j++) {
                // (0, j)から再帰的に探索
                // 上端からスタート
                dfs(heights, pacific, 0, j, m, n);
                // (m - 1, j)から再帰的に探索
                // 下端からスタート
                dfs(heights, atlantic, m - 1, j, m, n);
            }

            std::vector<std::vector<int>> result;

#ifdef FOR_DEBUG
            // 訪問済みの点を表示
            std::cout << "pacific:" << std::endl;
            for(const auto &a: pacific) {
                for(const auto &b: a) {
                    std::cout << b << " ";
                }
                std::cout << std::endl;
            }

            std::cout << "atlantic:" << std::endl;
            for(const auto &a: atlantic) {
                for(const auto &b: a) {
                    std::cout << b << " ";
                }
                std::cout << std::endl;
            }
#endif

            for (int i = 0; i < m; i++) {
                for (int j = 0; j < n; j++) {
                    // pacific, atlantic共に訪問済みならばok
                    if (pacific[i][j] && atlantic[i][j]) {
                        result.push_back({i, j});
                    }
                }
            }

            return result;
        }

    private:
        void dfs(std::vector<std::vector<int>>& heights, std::vector<std::vector<bool>>& visited, int i, int j, int m, int n) {
            // (i, j)を訪問済みにする
            visited[i][j] = true;

            // 十字方向に再帰的に探索
            //
            if (i > 0 && !visited[i - 1][j] && heights[i - 1][j] >= heights[i][j]) {
                dfs(heights, visited, i - 1, j, m, n);
            }

            if (i < m - 1 && !visited[i + 1][j] && heights[i + 1][j] >= heights[i][j]) {
                dfs(heights, visited, i + 1, j, m, n);
            }

            if (j > 0 && !visited[i][j - 1] && heights[i][j - 1] >= heights[i][j]) {
                dfs(heights, visited, i, j - 1, m, n);
            }

            if (j < n - 1 && !visited[i][j + 1] && heights[i][j + 1] >= heights[i][j]) {
                dfs(heights, visited, i, j + 1, m, n);
            }
        }
};

// 模範解答
class SolutionAns2 {
    public:
        // AC
        std::vector<std::vector<int>> pacificAtlantic(std::vector<std::vector<int>>& heights) {
            int m = heights.size();
            int n = heights[0].size();

            std::vector<std::vector<bool>> pacific(m, std::vector<bool>(n));
            std::vector<std::vector<bool>> atlantic(m, std::vector<bool>(n));

            // 左端、右端、上端、下端からそれぞれ山を再帰的に登る
            //
            for (int i = 0; i < m; i++) {
                // (i, 0)から再帰的に探索
                // 左端からスタート
                dfs(heights, pacific, i, 0);
                // (i, n - 1)から再帰的に探索
                // 右端からスタート
                dfs(heights, atlantic, i, n - 1);
            }

            for (int j = 0; j < n; j++) {
                // (0, j)から再帰的に探索
                // 上端からスタート
                dfs(heights, pacific, 0, j);
                // (m - 1, j)から再帰的に探索
                // 下端からスタート
                dfs(heights, atlantic, m - 1, j);
            }

            std::vector<std::vector<int>> result;

#ifdef FOR_DEBUG
            // 訪問済みの点を表示
            std::cout << "pacific:" << std::endl;
            for(const auto &a: pacific) {
                for(const auto &b: a) {
                    std::cout << b << " ";
                }
                std::cout << std::endl;
            }

            std::cout << "atlantic:" << std::endl;
            for(const auto &a: atlantic) {
                for(const auto &b: a) {
                    std::cout << b << " ";
                }
                std::cout << std::endl;
            }
#endif

            for (int i = 0; i < m; i++) {
                for (int j = 0; j < n; j++) {
                    // pacific, atlantic共に訪問済みならばok
                    if (pacific[i][j] && atlantic[i][j]) {
                        result.push_back({i, j});
                    }
                }
            }

            return result;
        }

    private:
        void dfs(std::vector<std::vector<int>>& heights, std::vector<std::vector<bool>>& visited, int x, int y) {
            auto [m, n] = std::pair(heights.size(), heights[0].size());
            // (i, j)を訪問済みにする
            visited[x][y] = true;

            // 十字方向に再帰的に探索
            std::vector<std::pair<int,int>> directions = {
                {1, 0},
                {0, 1},
                {-1, 0},
                {0, -1}
            };

            for (auto& [dx, dy]: directions) {
                auto [x_new, y_new] = std::pair(x + dx, y + dy);
                if(x_new >= 0 && y_new >= 0 && x_new < m && y_new < n) {
                    if(!visited[x_new][y_new] && heights[x_new][y_new] >= heights[x][y]) {
                        dfs(heights, visited, x_new, y_new);
                    }
                }
            }
        }
};

int main(void) {
    auto case_1 = std::vector{
        std::vector{1, 2, 2, 3, 5},
        std::vector{3, 2, 3, 4, 4},
        std::vector{2, 4, 5, 3, 1},
        std::vector{6, 7, 1, 4, 5},
        std::vector{5, 1, 1, 2, 4},
    };
    //=> [[0,4],[1,3],[1,4],[2,2],[3,0],[3,1],[4,0]]

    SolutionAns s_anas;
    auto res = s_anas.pacificAtlantic(case_1);
    std::cout << "s_ans" << std::endl;
    for(const auto &a: res) {
        for(const auto &b: a) {
            std::cout << b << " ";
        }
        std::cout << std::endl;
    }

    SolutionAns2 s_ans_2;
    auto res_2 = s_ans_2.pacificAtlantic(case_1);
    std::cout << "s_ans_2" << std::endl;
    for(const auto &a: res_2) {
        for(const auto &b: a) {
            std::cout << b << " ";
        }
        std::cout << std::endl;
    }
}
