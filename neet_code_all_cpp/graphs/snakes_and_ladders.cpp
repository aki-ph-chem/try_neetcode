#include <algorithm>
#include <cmath>
#include <iostream>
#include <vector>
#include <unordered_set>
#include <queue>

// Pythonの模範解答より
class SolutionAnsPython {
    public:
        // AC
        int snakesAndLadders(std::vector<std::vector<int>>& board) {
            auto n = (int)board.size();
            std::reverse(board.begin(), board.end());

            std::queue<std::pair<int,int>> q;
            q.push({1,0});
            std::unordered_set<int> visit;

            // BFS
            while(!q.empty()) {
                auto [square, moves] = std::pair(q.front().first, q.front().second);
                q.pop();

                for(int i = 1; i < 7; ++i) {
                    auto next_square = square + i;
                    auto [x, y] = square_to_position(next_square, n);

                    if(board[x][y] != -1) {
                        next_square = board[x][y];
                    }
                    if(next_square == std::pow(n,2)) {
                        return moves + 1;
                    }
                    if(!visit.count(next_square)) {
                        visit.insert(next_square);
                        q.push({next_square, moves + 1});
                    }
                }
            }

            return -1;
        }

    private:
        std::pair<int,int> square_to_position(int square, int n) {
            auto x = (square - 1)/ n;
            auto y = (square - 1) % n;

            if(x % 2) {
                y = n - 1 - y;
            }

            return {x, y};
        }
};

int main(void) {
    std::vector<std::vector<int>> case_1 = {
        {-1,-1,-1,-1,-1,-1},
        {-1,-1,-1,-1,-1,-1},
        {-1,-1,-1,-1,-1,-1},
        {-1,35,-1,-1,13,-1},
        {-1,-1,-1,-1,-1,-1},
        {-1,15,-1,-1,-1,-1}
    };
    // => 4
    std::vector<std::vector<int>> case_2 = {
        {-1, -1},
        {-1, 3}
    };
    // => 1

    SolutionAnsPython s_ans_py;

    std::cout << s_ans_py.snakesAndLadders(case_1) << std::endl;
    std::cout << s_ans_py.snakesAndLadders(case_2) << std::endl;
}
