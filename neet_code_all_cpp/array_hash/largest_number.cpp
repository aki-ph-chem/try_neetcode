#include <iostream>
#include <string>
#include <algorithm>
#include <vector>

class SolutionAns {
    public:
        std::string largestNumber(std::vector<int>& nums) {
            std::vector<std::string> vec_string;
            for(auto& n: nums) {
                vec_string.push_back(std::to_string(n));
            }

            std::sort(vec_string.begin(), vec_string.end(), [](auto& s_1, auto& s_2) 
                    {return s_1 + s_2 > s_2 + s_1;});


            if(vec_string[0] == "0") {
                return "0";
            }

            std::string result = "";
            for(auto& s: vec_string) {
                result += s;
            }
            return result;
        }
};

int main(void) {
    std::vector<int> case_1 = {10, 2};
    // => "210"
    std::vector<int> case_2 = {3, 30, 34, 5, 9};
    // => "9534330"

    SolutionAns s_ans;
    std::cout << s_ans.largestNumber(case_1) << std::endl;
    std::cout << s_ans.largestNumber(case_2) << std::endl;
}
