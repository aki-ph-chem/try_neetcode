#include <iostream>
#include <utility>
#include <vector>
#include <unordered_set>
#include <set>

// 模範解答
class SolutionAns {
    public:
        int countSubIsland(std::vector<std::vector<int>>& grid1, std::vector<std::vector<int>>& grid2) {
            auto [n, m] = std::pair(grid1.size(), grid1[0].size());
            std::unordered_set<int> visit;

            int result = 0;
            for(int x = 0; x < n; ++x) {
                for(int y = 0; y < m; ++y){
                    if(grid2[x][y] && !visit.count(x * m + y) && dfs(grid1, grid2, visit,x, y)) {
                        ++result;
                    }
                }
            }

            return result;
        }

    using Grid = std::vector<std::vector<int>>;
    private:
        bool dfs(Grid& grid1, Grid& grid2, std::unordered_set<int>& visit, int x, int y) {
            auto [n, m] = std::pair(grid1.size(), grid1[0].size());

            if(x < 0 
                    || y < 0
                    || x == n 
                    || y == m 
                    || grid2[x][y] == 0
                    || visit.count(x * m + y)) 
            {
                return true;
            }

            visit.insert(x * m + y);
            auto res = true;
            if(grid1[x][y] == 0) {
                res = false;
            }

            std::vector<std::pair<int,int>> directions = {
                {1, 0},
                {0, 1},
                {-1, 0},
                {0, -1}
            };

            for(auto& [dx, dy]: directions) {
                auto [x_new, y_new] = std::pair(x + dx, y + dy);
                res = dfs(grid1, grid2, visit, x_new, y_new) && res;
            }

            return res;
        }
};

// AC(std::set<std::pair<int,int>>で実装)
class SolutionAns2 {
    public:
        int countSubIsland(std::vector<std::vector<int>>& grid1, std::vector<std::vector<int>>& grid2) {
            auto [n, m] = std::pair(grid1.size(), grid1[0].size());
            std::set<std::pair<int,int>> visit;

            int result = 0;
            for(int x = 0; x < n; ++x) {
                for(int y = 0; y < m; ++y){
                    if(grid2[x][y] && !visit.count({x, y}) && dfs(grid1, grid2, visit,x, y)) {
                        ++result;
                    }
                }
            }

            return result;
        }

    using Grid = std::vector<std::vector<int>>;
    private:
        bool dfs(Grid& grid1, Grid& grid2, std::set<std::pair<int,int>>& visit, int x, int y) {
            auto [n, m] = std::pair(grid1.size(), grid1[0].size());

            if(x < 0 
                    || y < 0
                    || x == n 
                    || y == m 
                    || grid2[x][y] == 0
                    || visit.count({x, y})) 
            {
                return true;
            }

            visit.insert({x, y});
            auto res = true;
            if(grid1[x][y] == 0) {
                res = false;
            }

            std::vector<std::pair<int,int>> directions = {
                {1, 0},
                {0, 1},
                {-1, 0},
                {0, -1}
            };

            for(auto& [dx, dy]: directions) {
                auto [x_new, y_new] = std::pair(x + dx, y + dy);
                res = dfs(grid1, grid2, visit, x_new, y_new) && res;
            }

            return res;
        }
};

int main(void) {
    using Grid = std::vector<std::vector<int>>;
    using Case = std::pair<Grid, Grid>;

    Case case_1 = {
        {{1,1,1,0,0},{0,1,1,1,1},{0,0,0,0,0},{1,0,0,0,0},{1,1,0,1,1}},
        {{1,1,1,0,0},{0,0,1,1,1},{0,1,0,0,0},{1,0,1,1,0},{0,1,0,1,0}}
    };
    // => 3
    Case case_2 = {
        {{1,0,1,0,1},{1,1,1,1,1},{0,0,0,0,0},{1,1,1,1,1},{1,0,1,0,1}},
        {{0,0,0,0,0},{1,1,1,1,1},{0,1,0,1,0},{0,1,0,1,0},{1,0,0,0,1}}
    };
    // => 2

    SolutionAns s_ans;
    std::cout << s_ans.countSubIsland(case_1.first, case_1.second) 
        << std::endl;
    std::cout << s_ans.countSubIsland(case_2.first, case_2.second) 
        << std::endl;

    SolutionAns2 s_ans_2;
    std::cout << s_ans_2.countSubIsland(case_1.first, case_1.second) 
        << std::endl;
    std::cout << s_ans_2.countSubIsland(case_2.first, case_2.second) 
        << std::endl;
}
