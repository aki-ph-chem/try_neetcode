#include <iostream>
#include <vector>
#include <memory>

class Solution {
    public:
        // AC
        std::vector<std::vector<int>> shiftGrid(std::vector<std::vector<int>> grid, int k) {
            auto [m, n] = std::pair(grid.size(), grid[0].size());
            auto prev = grid; 
            auto result = prev;

            for(int iter = 0; iter < k ; ++iter) {
                for(int i = 0; i < m; ++i) {
                    for(int j = 0; j < n; ++j) {
                        if(j + 1 < n) {
                            result[i][j + 1] = prev[i][j];
                        }
                    }
                }
                for(int i = 0; i < m; ++i) {
                    if(i + 1 < m) {
                        result[i + 1][0] = prev[i][n - 1];
                    }
                }
                result[0][0] = prev[m - 1][n - 1];
                prev = result;
            }

            return result;
        }
};

// 模範解答
class SolutionAns {
    public:
        std::vector<std::vector<int>> shiftGrid(std::vector<std::vector<int>>& grid, int k) {
            int m = grid.size(), n = grid[0].size();
            auto posToVal = [&](int r, int c) {
                return r * n + c;
            };
            // 生のポインタをnewしてもstd::vector<T>でもunique_ptr<T[]>でもok
            // 速度は 生 > unique_ptr<T []> > std::vector<T> 
            auto valToPos = [&](int v) {
                //return std::vector{v/n, v % n};
                //return new int[]{v/n, v % n};
                auto ptr = std::make_unique<int[]>(2);
                ptr[0] = v / n;
                ptr[1] = v % n;

                return ptr;
            };

            std::vector<std::vector<int>> result;
            for(int r = 0; r < m; ++r) {
                std::vector<int> row;
                for(int c = 0; c < n; ++c) {
                    row.push_back(0);
                }
                result.push_back(row);
            }
            for(int r = 0; r < m; ++r) {
                for(int c= 0; c < n; ++c) {
                    int newVal = (posToVal(r, c) + k) % (m * n);
                    auto newRc = valToPos(newVal);
                    result[newRc[0]][newRc[1]] = grid[r][c];
                }
            }

            return result;
        }

        // AC
        std::vector<std::vector<int>> shiftGrid2(std::vector<std::vector<int>>& grid, int k) {
            int m = grid.size(), n = grid[0].size();
            auto posToVal = [&](int r, int c) {
                return r * n + c;
            };
            // std::pair<U,V>を返す
            auto valToPos = [&](int v) {
                return std::pair(v / n,  v % n);
            };

            std::vector<std::vector<int>> result;
            for(int r = 0; r < m; ++r) {
                std::vector<int> row;
                for(int c = 0; c < n; ++c) {
                    row.push_back(0);
                }
                result.push_back(row);
            }
            for(int r = 0; r < m; ++r) {
                for(int c= 0; c < n; ++c) {
                    int newVal = (posToVal(r, c) + k) % (m * n);
                    auto [idx_r, idx_c] = valToPos(newVal);
                    result[idx_r][idx_c] = grid[r][c];
                }
            }

            return result;
        }
};

void show_result(const std::vector<std::vector<int>>& result) {
    for(auto &v: result) {
        for(auto &w: v) {
            std::cout << w << " ";
        }
        std::cout << '\n';
    }
}

int main(void) {
    using Case = std::pair<std::vector<std::vector<int>>, int>;

    Case case_1 = {{{1,2,3},{4,5,6},{7,8,9}}, 1};
    Case case_2 = {{{3,8,1,9},{19,7,2,5},{4,6,11,10},{12,0,21,13}}, 4};
    Case case_3 = {{{1,2,3},{4,5,6},{7,8,9}}, 9};

    Solution s_1;
    auto res_1 = s_1.shiftGrid(case_1.first, case_1.second);
    auto res_2 = s_1.shiftGrid(case_2.first, case_2.second);
    auto res_3 = s_1.shiftGrid(case_3.first, case_3.second);
    show_result(res_1);
    show_result(res_2);
    show_result(res_3);

    SolutionAns s_ans;
    auto res_1_ans = s_ans.shiftGrid(case_1.first, case_1.second);
    auto res_2_ans = s_ans.shiftGrid(case_2.first, case_2.second);
    auto res_3_ans = s_ans.shiftGrid(case_3.first, case_3.second);
    show_result(res_1_ans);
    show_result(res_2_ans);
    show_result(res_3_ans);
}
