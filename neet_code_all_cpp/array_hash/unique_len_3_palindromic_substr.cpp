#include <iostream>
#include <vector>
#include <unordered_set>
#include <string>

// 模範解答
class SolutionAns {
    public:
        int countPalindromicSubsequence(std::string s) {
            std::vector<std::pair<int, int>> map(26, {-1, -1});
            for(int i = 0; i < s.size(); ++i) {
                if(map[s[i] - 'a'].first == -1) {
                    map[s[i] - 'a'].first = i;
                } else {
                    map[s[i] - 'a'].second = i;
                }
            }

            int result = 0;
            for(int i = 0; i < 26; ++i) {
                if(map[i].second != -1) {
                    std::unordered_set<char> tmp;
                    for(int j = map[i].first + 1; j < map[i].second; ++j) {
                        tmp.insert(s[j]);
                    }

                    result += tmp.size();
                }
            }

            return result;
        }
};

int main(void) {
    SolutionAns s_ans;

    std::string case_1 = "aabca";
    // => 3
    std::string case_2 = "adc";
    // => 0
    std::string case_3 = "bbcbaba";
    // => 4

    std::cout << s_ans.countPalindromicSubsequence(case_1) << std::endl;
    std::cout << s_ans.countPalindromicSubsequence(case_2) << std::endl;
    std::cout << s_ans.countPalindromicSubsequence(case_3) << std::endl;
}
