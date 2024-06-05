#include <iostream>
#include <vector>
#include <unordered_set>

// Kotlinの模範解答より
class SolutionAnsKotlin {
    public:
        // AC
        long long distinctNames(std::vector<std::string>& ideas) {
            std::vector<std::unordered_set<std::string>> fistToSuffix(26);
            for(auto&s: ideas) {
                fistToSuffix[s[0] - 'a'].insert(s.substr(1));
            }

            long long result = 0;
            for(int i = 0; i < 26; ++i) {
                for(int j = i; j < 26; ++j){
                    std::size_t common = 0;
                    for(auto& word_a: fistToSuffix[i]) {
                        if(fistToSuffix[j].count(word_a)) {
                            ++common;
                        }
                    }

                    auto map_i = fistToSuffix[i].size() - common;
                    auto map_j = fistToSuffix[j].size() - common;
                    result += (map_i * map_j) * 2;
                }
            }

            return result;
        }
};

int main(void){
    std::vector<std::string> case_1 = {"coffee", "donuts", "time", "toffee"};
    std::vector<std::string> case_2 = {"lack", "back"};

    SolutionAnsKotlin s_ans_kt;

    std::cout << s_ans_kt.distinctNames(case_1) << std::endl;
    std::cout << s_ans_kt.distinctNames(case_2) << std::endl;
}
