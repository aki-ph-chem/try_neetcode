#include <climits>
#include <iostream>
#include <vector>

class Solution {
    public:
        // AC
        std::vector<int> replaceElements(std::vector<int>& arr) {
            auto n = (int)arr.size();
            std::vector<int> result(n);
            result[n - 1] = -1;

            int max_tmp = INT_MIN;
            for(int i = n - 1; i >=1; --i) {
                max_tmp = std::max(max_tmp, arr[i]);
                result[i - 1] = max_tmp;
            }

            return result;
        }
};

// 模範解答
class SolutionAns {
    public:
        std::vector<int> replaceElements(std::vector<int> arr) {
            auto n = (int)arr.size();
            auto maxSoFar = arr[n - 1];
            arr[n - 1] = -1;

            for(int i = n - 2; i >=0; --i) {
                auto temp = maxSoFar;
                if(maxSoFar < arr[i]) maxSoFar = arr[i];
                arr[i] = temp;
            }

            return arr;
        }
};

void show_result(std::vector<int>& result) {
    for(auto& v: result) {
        std::cout << v << " ";
    }
    std::cout << std::endl;
}

int main(void) {
    auto case_1 = std::vector{17, 18, 5, 4, 6, 1};
    // => [18,6,6,6,1,-1]
    auto case_2 = std::vector{400};
    // => [-1]

    Solution s_1;
    auto res_1 = s_1.replaceElements(case_1);
    auto res_2 = s_1.replaceElements(case_2);
    show_result(res_1);
    show_result(res_2);

    SolutionAns s_ans;
    auto res_1_ans = s_ans.replaceElements(case_1);
    auto res_2_ans = s_ans.replaceElements(case_2);
    show_result(res_1_ans);
    show_result(res_2_ans);
}
