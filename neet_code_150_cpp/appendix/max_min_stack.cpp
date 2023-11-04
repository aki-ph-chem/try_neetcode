#include <vector>
#include <iostream>

// stackを使って最大値、最小値を求める
template<typename T>
class SearchMax{
    public:
        T search_max(const std::vector<T>& nums) {
            std::vector<T> stack;
            for(const auto& v: nums) {
                stack.push_back(v);
                if(stack.size() >= 2 && stack.back() <= stack[stack.size() - 2]) {
                    stack.pop_back();
                }
            }

            std::cout << stack.back() << std::endl;
            return stack.back();
        }

        T search_min(const std::vector<T>& nums) {
            std::vector<T> stack;
            for(const auto& v: nums) {
                stack.push_back(v);
                if(stack.size() >=2 && stack.back() >= stack[stack.size() - 2]) {
                    stack.pop_back();
                }
            }

            std::cout << stack.back() << std::endl;
            return stack.back();
        }
};

int main(void) {
    auto case_1 = std::vector{10, 8, 1, 20, 3, 5};
    auto case_2 = std::vector{10, 23, 3, -10, 51, 3, 1};

    SearchMax<int> s_1;
    s_1.search_max(case_1);
    s_1.search_min(case_1);

    s_1.search_max(case_2);
    s_1.search_min(case_2);
}
