#include <iostream>
#include <tuple>
#include <vector>

// Pythonの模範解答より
class SolutionAnsPython {
    public:
        // AC
        std::vector<int> findClosestElements(std::vector<int>& arr, int k, int x) {
            auto [left, right] = std::pair(0, (int)arr.size() - 1);
            auto [val, idx] = std::pair(arr[0], 0);

            while(left <= right) {
                int mid = left + (right - left) / 2;
                auto [current_diff, res_diff] = std::pair(std::abs(arr[mid] - x), std::abs(val - x));
                if(current_diff < res_diff || (current_diff == res_diff && arr[mid] < val)) {
                    val = arr[mid];
                    idx = mid;
                }

                if(arr[mid] < x) {
                    left = mid + 1;
                } else if (arr[mid] > x) {
                    right = mid - 1;
                } else {
                    break;
                }

            }

            left = idx;
            right = idx;

            for(int i = 0; i < k - 1; ++i) {
                if(left == 0) {
                    ++right;
                } else if(right == arr.size() - 1 || (x - arr[left - 1] <= arr[right + 1] - x)) {
                    --left;
                } else {
                    ++right;
                }
            }

            return std::vector(arr.begin() + left, arr.begin() + right + 1);
        }
};

void show_result(std::vector<int>& result) {
    for(auto&v: result) {
        std::cout << v << " ";
    }
    std::cout << std::endl;
}

int main(void) {
    using Case = std::tuple<std::vector<int>,int ,int >;
    Case case_1 = {{1, 2, 3, 4, 5}, 4, 3};
    // => [1, 2, 3, 4}
    Case case_2 = {{1, 2, 3, 4, 5}, 4, -1};
    // => [1, 2, 3, 4}
    Case case_3 = {{1, 1, 1, 10, 10, 10}, 1, 9};
    // => [10}

    SolutionAnsPython s_ans_Py;

    auto res_1 =s_ans_Py.findClosestElements(std::get<0>(case_1), std::get<1>(case_1), std::get<2>(case_1));
    show_result(res_1);

    auto res_2 =s_ans_Py.findClosestElements(std::get<0>(case_2), std::get<1>(case_2), std::get<2>(case_2));
    show_result(res_2);

    auto res_3 =s_ans_Py.findClosestElements(std::get<0>(case_3), std::get<1>(case_3), std::get<2>(case_3));
    show_result(res_3);
}
