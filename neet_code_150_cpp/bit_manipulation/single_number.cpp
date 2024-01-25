#include <iostream>
#include <vector>
#include <unordered_map>

class Solution {
    public:
        // 縛り無視ならこう実装する
        // AC
        int singleNumberMap(std::vector<int>& nums) {
            std::unordered_map<int, int> map;

            for(auto &v: nums) {
                ++map[v];
            }

            int result = 0;
            for(auto &v: map) {
                if(v.second == 1) {
                    result = v.first;
                    break;
                }
            }

            return result;
        }
};

int main(void) {
    auto case_1 = std::vector{2, 2, 1};
    auto case_2 = std::vector{4, 1, 2, 2, 1};
    auto case_3 = std::vector{1};

    Solution s_1;
    std::cout << s_1.singleNumberMap(case_1) << std::endl;
    std::cout << s_1.singleNumberMap(case_2) << std::endl;
    std::cout << s_1.singleNumberMap(case_3) << std::endl;
}
