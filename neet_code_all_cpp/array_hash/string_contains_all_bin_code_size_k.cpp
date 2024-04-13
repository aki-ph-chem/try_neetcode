#include <iostream>
#include <set>
#include <unordered_map>
#include <unordered_set>
#include <string>

class Solution {
    public:
        // AC
        bool hasAllCodes(std::string s, int k) {
            int n = s.size();
            if(n < k) {
                return false;
            }

            auto str_to_bit = [](auto& bit_string) {
                int n = bit_string.size();
                int bit = 0;
                for(int i = n - 1; i >= 0; --i) {
                    if(bit_string[i] == '1') {
                        bit += 1 << (n - i - 1);
                    }
                }

                return bit;
            };

            std::unordered_map<int, int> bit_list;
            for(int bit = 0; bit < 1 <<k; ++bit) {
                bit_list[bit] = 0;
            }

            for(int i = 0; i <= (n - k); ++i) {
                auto sub_str = s.substr(i, k);
                ++bit_list[str_to_bit(sub_str)];
            }

            for(auto& [bit, n]: bit_list) {
                if(n == 0) {
                    return false;
                }
            }

            return true;
        }
};

// 模範解答
class SolutionAns {
    public:
        // AC
        bool hasAllCodes(std::string s, int k) {
            std::set<std::string> all_strings;
            int total = 1 << k;

            for(int i = 0; i + k <= s.size(); ++i) {
                all_strings.insert(s.substr(i, k));
                if(all_strings.size() == total) {
                    return true;
                }
            }

            return false;
        }
};

int main(void) {
    using Case = std::pair<std::string, int>;
    Case case_1 = {"00110110", 2};
    // => true
    Case case_2 = {"0110", 1};
    // => true
    Case case_3 = {"0110", 2};
    // => false

    Solution s_1;
    std::cout << s_1.hasAllCodes(case_1.first, case_1.second) << std::endl;
    std::cout << s_1.hasAllCodes(case_2.first, case_2.second) << std::endl;
    std::cout << s_1.hasAllCodes(case_3.first, case_3.second) << std::endl;

    SolutionAns s_ans;
    std::cout << s_ans.hasAllCodes(case_1.first, case_1.second) << std::endl;
    std::cout << s_ans.hasAllCodes(case_2.first, case_2.second) << std::endl;
    std::cout << s_ans.hasAllCodes(case_3.first, case_3.second) << std::endl;
}
