#include <iostream>
#include <string>

class Solution {
    public:
        // AC
        std::string mergeAlternately(std::string word1, std::string word2) {
            std::string result;
            auto [w_1, w_2] = std::pair(0, 0);

            while(w_1 < word1.size() && w_2 < word2.size()) {
                result.push_back(word1[w_1]);
                result.push_back(word2[w_2]);
                ++w_1;
                ++w_2;
            }

            while(w_1 < word1.size()) {
                result.push_back(word1[w_1]);
                ++w_1;
            }
            while(w_2 < word2.size()) {
                result.push_back(word2[w_2]);
                ++w_2;
            }

            return result;
        }
};

// 模範解答
class SolutionAns {
    public:
        std::string mergeAlternately(std::string word1, std::string word2) {
            int i = 0;
            std::string result = "";

            while(i < word1.size() && i < word2.size()) {
                // += 演算子で書くとランタイムエラーが出るのだが...
                //result += word1[i] + word2[i++];
                result = result + word1[i] + word2[i++];
            }

            while(i < word1.size()) {
                result += word1[i++];
            }
            while(i < word2.size()) {
                result += word2[i++];
            }

            return result;
        }
};

int main(void) {
    using Case = std::pair<std::string, std::string>;

    Case case_1 = {"abc", "pqr"}; 
    // => apbqcr
    Case case_2 = {"ab", "pqrs"};
    // => apbqrs
    Case case_3 = {"abcd", "pq"};
    // => apbqcd 

    Solution s_1;
    std::cout << s_1.mergeAlternately(case_1.first, case_1.second) << std::endl;
    std::cout << s_1.mergeAlternately(case_2.first, case_2.second) << std::endl;
    std::cout << s_1.mergeAlternately(case_3.first, case_3.second) << std::endl;

    Solution s_ans;
    std::cout << s_ans.mergeAlternately(case_1.first, case_1.second) << std::endl;
    std::cout << s_ans.mergeAlternately(case_2.first, case_2.second) << std::endl;
    std::cout << s_ans.mergeAlternately(case_3.first, case_3.second) << std::endl;
}
