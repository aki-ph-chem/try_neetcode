#include <iostream>
#include <vector>
#include <set>

// 模範解答
class SolutionAns {
    public:
        int islandPerimeter(std::vector<std::vector<int>>& grid) {
            auto [m, n] = std::pair(grid.size(), grid[0].size());
            int result = 0;

            for(int i = 0; i < m; ++i) {
                for(int j = 0; j < n; ++j) {
                    if(grid[i][j] == 1) {
                        result += 4;

                        if(j > 0 && grid[i][j - 1] == 1) {
                            result -= 2;
                        }
                        if(i > 0 && grid[i - 1][j] == 1) {
                            result -= 2;
                        }
                    }
                }
            }

            return result;
        }
};

// Pythonの模範解答より
class SolutionAnsPython {
    public:
        // AC
        int islandPerimeter(std::vector<std::vector<int>>& grid) {
            std::set<std::pair<int,int>> visit;

            for(int i = 0; i < grid.size(); ++i) {
                for(int j = 0; j < grid[0].size(); ++j) {
                    if(grid[i][j]) {
                        return dfs(grid, visit, i, j);
                    }
                }
            }

            return 0;
        }

    private:
        int dfs(std::vector<std::vector<int>>& grid, std::set<std::pair<int,int>>& visit, int i, int j) {
            if(i >= grid.size() 
                    || j >= grid[0].size()
                    || i < 0 
                    || j < 0
                    || grid[i][j] == 0) 
            {
                return 1;
            }
            if(visit.find({i,j}) != visit.end()) {
                return 0;
            }

            visit.insert({i, j});
            int result = 0;
            std::vector<std::pair<int,int>> directions = {
                {1, 0},
                {0, 1},
                {-1, 0},
                {0, -1}
            };
            for(auto& [dx, dy]: directions) {
                result += dfs(grid, visit, i + dx, j + dy);
            }

            return result;
        }
};

int main(void) {
    std::vector<std::vector<int>> case_1 = {{0, 1, 0, 0},{1, 1, 1, 0},{0, 1, 0, 0},{1, 1, 0, 0}};
    // => 16
    std::vector<std::vector<int>> case_2 = {{1}};
    // => 4
    std::vector<std::vector<int>> case_3 = {{1, 0}};
    // => 4

    SolutionAns s_ans;

    std::cout << s_ans.islandPerimeter(case_1) << std::endl;
    std::cout << s_ans.islandPerimeter(case_2) << std::endl;
    std::cout << s_ans.islandPerimeter(case_3) << std::endl;

    SolutionAnsPython s_ans_py;

    std::cout << s_ans_py.islandPerimeter(case_1) << std::endl;
    std::cout << s_ans_py.islandPerimeter(case_2) << std::endl;
    std::cout << s_ans_py.islandPerimeter(case_3) << std::endl;
}
