#include <iostream>
#include <algorithm>
#include <vector>

template<typename T>
void show_result(std::vector<T>& result) {
    for(auto& v: result) {
        std::cout << v << " ";
    }
    std::cout << std::endl;
}

int main(void) {
    // 普通のsort
    auto nums = std::vector<int>{5, 18, 2, 10, 8};
    std::sort(nums.begin(), nums.end());
    show_result(nums);

    // 文字列の長さでsort
    auto strings = std::vector<std::string>{"ffoog", "ho", "a", "hogex_xhoge", "abcd", "xyz"};
    // 第三引数にはbool値を返す関数もしくはラムダ式を入れる
    std::sort(strings.begin(), strings.end(), [](auto& s_1, auto& s_2) { return s_1.size() < s_2.size();});
    show_result(strings);
}
