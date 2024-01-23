#include <iostream>
#include <string>
#include <vector>
#include <unordered_map>

// 模範解答
class SolutionAns {
    public:
        bool checkInclusion(std::string s1, std::string s2) {
            int m = s1.size();
            int n = s2.size();
            if (m > n) {
                return false;
            }

            std::vector<int> count(26);
            for (int i = 0; i < m; i++) {
                count[s1[i] - 'a']++;
                count[s2[i] - 'a']--;
            }
            if (isPermutation(count)) {
                return true;
            }

            for (int i = m; i < n; i++) {
                count[s2[i] - 'a']--;
                count[s2[i - m] - 'a']++;
                if (isPermutation(count)) {
                    return true;
                }
            }

            return false;
        }
    private:
        bool isPermutation(std::vector<int>& count) {
            for (int i = 0; i < 26; i++) {
                if (count[i] != 0) {
                    return false;
                }
            }
            return true;
        }
};

// AC
// バケットの代わりにunordered_mapを使う
class SolutionMine {
    public:
        bool checkInclusion(std::string s1, std::string s2) {
            int m = s1.size();
            int n = s2.size();
            if(m > n) {
                return false;
            }

            std::unordered_map<char, int> count_map;
            // 0 ~ m - 1 文字目まで(n >= m)
            for(int i = 0; i < m; ++i) {
                ++count_map[s1[i]];
                --count_map[s2[i]];
            }
            if(isPermutation(count_map)) {
                return true;
            }

            // m ~ n - 1 文字目まで
            for(int i = m; i < n; ++i) {
                --count_map[s2[i]];
                // i - m: 0 ~ n - m 
                ++count_map[s2[i - m]];
                if(isPermutation(count_map)) {
                    return true;
                }
            }

            return false;
        }

    private:
        bool isPermutation(std::unordered_map<char, int>& count_map) {
        // int側が全て0ならtrue
            for(auto& v: count_map) {
                if(v.second != 0) {
                    return false;
                }
            }

            return true;
        }
};

int main(void) {
    auto case_1 = std::pair(std::string{"ab"}, std::string{"eidbaooo"});
    // => true
    auto case_2 = std::pair(std::string{"ab"}, std::string{"eidboaoo"});
    // => false

    SolutionAns s_ans;
    std::cout << s_ans.checkInclusion(case_1.first, case_1.second) << std::endl;
    std::cout << s_ans.checkInclusion(case_2.first, case_2.second) << std::endl;

    SolutionMine s_mine;
    std::cout << s_mine.checkInclusion(case_1.first, case_1.second) << std::endl;
    std::cout << s_mine.checkInclusion(case_2.first, case_2.second) << std::endl;
}
