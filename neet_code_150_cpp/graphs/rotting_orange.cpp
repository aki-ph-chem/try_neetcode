#include <iostream>
#include <vector>
#include <queue>

// 模範解答(所々書き方を変えてはいるが...)
class SolutionAns {
    public:
        int orangeRotting(std::vector<std::vector<int>>& grid) {
            auto [m, n] = std::pair((int)grid.size(), (int)grid[0].size());

            // q: 腐ったオレンジのある場所のキュー
            // fresh: 新鮮なオレンジの数
            std::queue<std::pair<int, int>> q;
            int fresh = 0;
            for(int i = 0; i < m; ++i) {
                for(int j = 0; j < n; ++j) {
                    if(grid[i][j] == 2) {
                        q.push({i, j});
                    } else if (grid[i][j] == 1) {
                        ++fresh;
                    }
                }
            }

            // (-1, -1)よりスタート
            q.push({-1, -1});
            int result = -1;

            while(!q.empty()) {
                auto [row, col] = std::pair(q.front().first, q.front().second);
                q.pop();

                if(row == -1) {
                    ++result;
                    if(!q.empty()) {
                        q.push({-1, -1});
                    }
                } else {
                    std::vector<std::pair<int ,int>> directions = {{-1, 0}, {1, 0}, {0, -1}, {0, 1}}; 

                    for(auto& v: directions) {
                        auto [x, y] = std::pair(row + v.first, col + v.second);

                        if (x < 0 || x >= m || y < 0 || y >= n) {
                            continue;
                        }

                        if(grid[x][y] == 1) {
                            grid[x][y] = 2;
                            --fresh;
                            q.push({x, y});
                        }
                    }
                }
            }

            if(!fresh) {
                return result;
            } 

            return -1;
        }
};


int main(void) {
    auto case_1 = std::vector{std::vector{2, 1, 1}, std::vector{1, 1, 0}, std::vector{0, 1, 1}};
    // => 4
    auto case_2 = std::vector{std::vector{2, 1, 1}, std::vector{0, 1, 1}, std::vector{1, 0, 1}};
    // => -1
    
    SolutionAns s_ans;
    std::cout << s_ans.orangeRotting(case_1) << std::endl;
    std::cout << s_ans.orangeRotting(case_2) << std::endl;
}
