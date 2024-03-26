#include <iostream>
#include <vector>
#include <unordered_map>

// AC
// Rustの模範解答より
class SolutionAnsRust {
    public:
        int totaolFruit(std::vector<int>& fruits){
            std::unordered_map<int, int> map;
            int left = 0, total = 0;
            int result = 0;

            for(auto& f: fruits) {
                ++map[f];
                ++total;

                while(map.size() > 2) {
                    auto f_left = fruits[left];
                    auto v = map[f_left];
                    map.erase(f_left);

                    if(v > 1) {
                        map.insert({f_left, v - 1});
                    }

                    --total;
                    ++left;
                }

                result = std::max(result, total);
            }

            return result;
        }
};

// AC
// Pythonの模範解答より
class SolutionAnsPython {
    public:
        int totaolFruit(std::vector<int>& fruits){
            std::unordered_map<int, int> map;
            int left = 0, total = 0;
            int result = 0;

            for(auto& f: fruits) {
                ++map[f];
                ++total;

                while(map.size() > 2) {
                    auto f_left = fruits[left];

                    --map[f_left];
                    --total;
                    ++left;
                    if(!map[f_left]) {
                        map.erase(f_left);
                    }
                }

                result = std::max(result, total);
            }

            return result;
        }
};

int main(void) {
    std::vector<int> case_1 = {1, 2, 1};
    // => 3
    std::vector<int> case_2 = {0, 1, 2, 2};
    // => 3
    std::vector<int> case_3 = {1, 2, 3, 2, 2};
    // => 4
    std::vector<int> case_4 = {3, 3, 3, 1, 2, 1, 1, 2, 3, 3, 4};
    // => 5

    SolutionAnsRust s_ans_rs;
    std::cout << s_ans_rs.totaolFruit(case_1) << std::endl;
    std::cout << s_ans_rs.totaolFruit(case_2) << std::endl;
    std::cout << s_ans_rs.totaolFruit(case_3) << std::endl;
    std::cout << s_ans_rs.totaolFruit(case_4) << std::endl;

    SolutionAnsPython s_ans_py;
    std::cout << s_ans_py.totaolFruit(case_1) << std::endl;
    std::cout << s_ans_py.totaolFruit(case_2) << std::endl;
    std::cout << s_ans_py.totaolFruit(case_3) << std::endl;
    std::cout << s_ans_py.totaolFruit(case_4) << std::endl;
}
