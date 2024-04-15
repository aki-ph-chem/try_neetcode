#include <iostream>
#include <string>
#include <unordered_set>
#include <vector>

class Solution {
    public:
        // AC
        int partitionString(std::string s) {
            std::vector<std::pair<int,int>> result;
            std::unordered_set<char> set;
            int i = 0;
            while(i < s.size()) {
                int j = i;
                while(j < s.size() && !set.count(s[j])) {
                    set.insert(s[j]);
                    ++j;
                }
                result.push_back({i, j - 1});
                set.clear();
                i = j;
            }

            return (int)result.size();
        }

        // AC
        int partitionString2(std::string s) {
            int result = 0;
            std::unordered_set<char> set;
            int i = 0;
            while(i < s.size()) {
                int j = i;
                while(j < s.size() && !set.count(s[j])) {
                    set.insert(s[j]);
                    ++j;
                }
                ++result;
                set.clear();
                i = j;
            }

            return result;
        }
};

// Kotlinの模範解答より
class SolutionAnsKotlin {
    public:
        // AC
        int partitionString(std::string s) {
            int result = 0;
            std::unordered_set<char> set;

            for(auto& c: s){
                if(set.count(c)) {
                    ++result;
                    set.clear();
                }
                set.insert(c);
            }

            if(set.size() == 0) {
                return result;
            }
            return result + 1;
        }
};

int main(void) {
    std::string case_1 = "abacaba";
    // => 4
    std::string case_2 = "ssssss";
    // => 6

    Solution s_1;
    std::cout << s_1.partitionString(case_1) << std::endl;
    std::cout << s_1.partitionString(case_2) << std::endl;

    std::cout << s_1.partitionString2(case_1) << std::endl;
    std::cout << s_1.partitionString2(case_2) << std::endl;

    SolutionAnsKotlin s_ans_kt;
    std::cout << s_ans_kt.partitionString(case_1) << std::endl;
    std::cout << s_ans_kt.partitionString(case_2) << std::endl;
}
