#include <iostream>
#include <string>
#include <vector>

void show_array(std::vector<std::string>& arr) {
    for(auto &v: arr) {
        std::cout << v << ",";
    }
    std::cout << std::endl;
}

// AC
class Solution {
    public:
        std::vector<std::pair<int, int>> map;

        std::string encode(std::vector<std::string>& strs) {
            auto result = std::string{""};

            int a = 0;
            for(int i = 0; i < strs.size(); ++i) {
                result += strs[i];

                map.push_back({a, strs[i].size()});
                a += strs[i].size();
            }

            return result;
        }

        std::vector<std::string> decode(std::string s) {
            std::vector<std::string> result;
            for(auto& m: map) {
                result.push_back(s.substr(m.first, m.second));
            }

            return result;
        }

        void show_map() {
            for(auto& v: map) {
                std::cout << v.first << "," << v.second << std::endl;
            }
        }
};

// 模範解答
class SolutionAns {
public:

    std::string encode(std::vector<std::string>& strs) {
        std::string result = "";
        
        for (int i = 0; i < strs.size(); i++) {
            std::string str = strs[i];
            result += std::to_string(str.size()) + "#" + str;
        }
        
        return result;
    }

    std::vector<std::string> decode(std::string s) {
        std::vector<std::string> result;
        
        int i = 0;
        while (i < s.size()) {
            int j = i;
            while (s[j] != '#') {
                j++;
            }
            int length = stoi(s.substr(i, j - i));
            std::string str = s.substr(j + 1, length);
            result.push_back(str);
            i = j + 1 + length;
        }
        
        return result;
    }
};


int main(void) {
    auto case_1 = std::vector{std::string{"neet"}, std::string{"code"}, std::string{"love"}, std::string{"your"}};
    auto case_2 = std::vector{std::string{"we"}, std::string{"say"}, std::string{":"}, std::string{"yes"}};

    Solution s_1;

    auto encoded_1 = s_1.encode(case_1);
    auto decoded_1 = s_1.decode(encoded_1);

    std::cout << encoded_1 << std::endl;
    s_1.show_map();
    show_array(decoded_1);

    Solution s_2;

    auto encoded_2 = s_2.encode(case_2);
    auto decoded_2 = s_2.decode(encoded_2);
    show_array(decoded_2);

    Solution s_ans;

    auto encoded_1_ans = s_ans.encode(case_1);
    auto decoded_1_ans = s_ans.decode(encoded_1_ans);
    show_array(decoded_1_ans);
}
