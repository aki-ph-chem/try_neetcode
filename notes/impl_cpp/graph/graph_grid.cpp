#include <iostream>
#include <vector>
#include <queue>

template<typename T>
using Grid = std::vector<std::vector<T>>;

class DfsGridCross {
    public:
        void search(const Grid<int>& grid) {
            auto [m, n] = std::pair(grid.size(), grid[0].size());
            Grid<bool> seen(m, std::vector<bool>(n, false));

            for(int i = 0; i < m; ++i) {
                for(int j = 0; j < n; ++j) {
                    dfs(grid, seen, i, j);
                }
            }

            std::cout << "end" << std::endl;
        }

    private:
        void dfs(const Grid<int>& grid, Grid<bool>& seen, int x, int y) {
            auto [m, n] = std::pair(grid.size(), grid[0].size());

            if(x < 0
                    || y < 0
                    || x >= m
                    || y >= n
                    || seen[x][y]
              ) 
            {
                return;
            }

            seen[x][y] = true;

            std::cout << grid[x][y] << " -> ";

            std::vector<std::pair<int, int>> directions 
                = {{0, 1}, {1, 0}, {0, -1}, {-1, 0}};
            for(auto& s: directions) {
                dfs(grid, seen, x + s.first, y + s.second);
            }
        }
};

class BfsGridCross {
    public:
        void search(const Grid<int>& grid, int x, int y) {
            auto [m, n] = std::pair(grid.size(), grid[0].size());
            Grid<bool> seen(m, std::vector<bool>(n, false));
            std::queue<std::pair<int, int>> todo;

            seen[x][y] = true;
            todo.push({x, y});

            while(!todo.empty()) {
                auto [v_x, v_y] = todo.front();
                todo.pop();

                std::cout << grid[v_x][v_y] << " -> ";

                std::vector<std::pair<int, int>> directions 
                    = {{1, 0}, {0, 1}, {-1, 0}, {0, -1}};
                for(auto& s: directions) {
                    auto [new_x, new_y] = std::pair(v_x + s.first, v_y + s.second);

                    if(new_x < 0
                            || new_y < 0
                            || new_x >= m
                            || new_y >= n
                            || seen[new_x][new_y]
                            ) {
                        continue;
                    }

                    seen[new_x][new_y] = true;
                    todo.push({new_x, new_y});
                }
            }
            std::cout << std::endl;
        }
};

int main(void) {
    Grid<int> case_1 = {
        {1, 2, 3},
        {4, 5, 6},
        {7, 8, 9}
    };

    DfsGridCross dfs_grid;
    std::cout << "DFS" << std::endl; 
    dfs_grid.search(case_1);

    BfsGridCross bfs_grid;
    std::cout << "BFS" << std::endl;
    bfs_grid.search(case_1, 0, 0);
}
