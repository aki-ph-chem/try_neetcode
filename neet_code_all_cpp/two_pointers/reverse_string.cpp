#include <iostream>
#include <vector>

class Solution {
    // AC
    // 模範解答も全く同じ
    public:
        void reverseString(std::vector<char>& s) {
            auto [l, r] = std::pair(0, s.size() - 1);
            while(l < r) {
                std::swap(s[l], s[r]);
                ++l;
                --r;
            }
        }
};

void show_result(std::vector<char>& result) {
    for(auto&c :result) {
        std::cout << c << " ";
    }
    std::cout << std::endl;
}

int main(void) {
    std::vector<char> case_1 = {'h', 'e', 'l', 'l', 'o'};
    std::vector<char> case_2 = {'H', 'a', 'n', 'n', 'a', 'h'};

    Solution s_1;
    auto res_1 = case_1;
    s_1.reverseString(res_1);
    auto res_2 = case_2;
    s_1.reverseString(res_2);

    show_result(res_1);
    show_result(res_2);
}
