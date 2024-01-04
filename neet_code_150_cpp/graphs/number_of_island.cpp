#include <iostream>
#include <vector>

// Rustの模範解答より
class SolutionRustAns {
    public:
        int numIslands(const std::vector<std::vector<char>>& grid) {
            auto count = 0;
            auto new_grid = grid;

            for(int x = 0; x < grid.size(); ++x) {
                for(int y = 0; y < grid[0].size(); ++y) {
                    if(new_grid[x][y] == '1') {
                        ++count;
                        dfs(new_grid, x, y);
                    }
                }
            }

            return count;
        }

    private:
        void dfs(std::vector<std::vector<char>>& grid, int x, int y) {
            if(x < 0
                    || y < 0
                    || x >= grid.size()
                    || y >= grid[0].size()
                    || grid[x][y] == '0'
              ) 
            {
                return;
            }

            // 訪れた点は'0'に書き換える
            grid[x][y] = '0';

            // 十字方向を再帰的に探索
            dfs(grid, x + 1, y);
            dfs(grid, x, y + 1);
            dfs(grid, x - 1, y);
            dfs(grid, x, y - 1);
        }
};

// 模範解答
class SolutionAns {
public:
    int numIslands(std::vector<std::vector<char>>& grid) {
        int m = grid.size();
        int n = grid[0].size();
        
        int result = 0;
        
        for (int i = 0; i < m; i++) {
            for (int j = 0; j < n; j++) {
                if (grid[i][j] == '1') {
                    dfs(grid, i, j, m, n);
                    result++;
                }
            }
        }
        
        return result;
    }

private:
    void dfs(std::vector<std::vector<char>>& grid, int i, int j, int m, int n) {
        if (i < 0 || i >= m || j < 0 || j >= n || grid[i][j] == '0') {
            return;
        }

        // 訪れた点を'0'に書き換える
        grid[i][j] = '0';
        // 十字方向を再帰的に探索
        dfs(grid, i - 1, j, m, n);
        dfs(grid, i + 1, j, m, n);
        dfs(grid, i, j - 1, m, n);
        dfs(grid, i, j + 1, m, n);
    }
};

int main(void) {
    auto case_1 = std::vector{
        std::vector{'1', '1', '1', '1', '0'},
        std::vector{'1', '1', '0', '1', '0'},
        std::vector{'1', '1', '0', '0', '0'},
        std::vector{'0', '0', '0', '0', '0'},
    };
    // => 1
    auto case_2 = std::vector{
        std::vector{'1', '1', '0', '0', '0'},
        std::vector{'1', '1', '0', '0', '0'},
        std::vector{'0', '0', '1', '0', '0'},
        std::vector{'0', '0', '0', '1', '1'},
    };
    // => 3

    SolutionRustAns s_rust_ans;
    std::cout << s_rust_ans.numIslands(case_1) << std::endl;
    std::cout << s_rust_ans.numIslands(case_2) << std::endl;

    SolutionAns s_ans;
    std::cout << s_ans.numIslands(case_1) << std::endl;
    std::cout << s_ans.numIslands(case_2) << std::endl;
}
