#include <cmath>
#include <iostream>
#include <vector>

class Solution {
    public:
        // AC
        std::vector<std::vector<int>> generator(int numRows) {
            std::vector<std::vector<int>> result = {{1}};
            if(numRows == 1) {
                return result;
            }

            for(int n = 1; n < numRows; ++n) {
                std::vector<int> new_row(n + 1, 0);
                new_row[0] = 1;
                new_row[n] = 1;

                for(int i = 1; i < n; ++i) {
                    new_row[i] = result[n - 1][i - 1] + result[n - 1][i];
                }

                result.push_back(new_row);
            }

            return result;
        }
};

// 模範解答
class SolutinAns {
public:
    std::vector<std::vector<int>> generate(int numRows)
    {
        std::vector<std::vector<int>> ret;

        for(int i = 0; i < numRows ; i++){
            std::vector<int> row(i+1, 1);
            for(int j = 1; j < i ; j++){
                row[j] = ret[i-1][j] + ret[i-1][j-1];
            }
            ret.push_back(row);
        }

        return ret;
    }
};

void show_array_2d(std::vector<std::vector<int>>& result) {
    for(auto& v: result) {
        for(auto&w: v) {
            std::cout << w << " ";
        }
        std::cout << std::endl;
    }
}

int main(void) {
    Solution s_1;
    auto res_1 = s_1.generator(1);
    auto res_2 = s_1.generator(2);
    auto res_3 = s_1.generator(3);
    auto res_4 = s_1.generator(4);
    auto res_5 = s_1.generator(5);

    show_array_2d(res_1);
    show_array_2d(res_2);
    show_array_2d(res_3);
    show_array_2d(res_4);
    show_array_2d(res_5);

    SolutinAns s_ans;

    auto res_5_ans = s_ans.generate(5);
    show_array_2d(res_5_ans);
}
