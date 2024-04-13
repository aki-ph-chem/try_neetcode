#include <iostream>
#include <string>
#include <vector>

void show_array(std::vector<int>& array) {
    for(auto& v: array){
        std::cout << v << " ";
    }
    std::cout << std::endl;
}

class SolutionAns{
    public:
        std::string pushDominoes(std::string dominoes) {
            std::string result = "";
            int n = dominoes.size(), count = 1;
            char prev = 0;

            std::vector<int> left(n, 0), right(n ,0);
            for(int i = 0; i < n; ++i) {
                if(dominoes[i] == 'R') {
                    count = 1;
                    prev = 'R';
                } else if (dominoes[i] != '.') {
                    prev = dominoes[i];
                }

                if(prev == 'R'&& dominoes[i] == '.') {
                    right[i] = count;
                    ++count;
                }
            }

            prev = '.';
            for(int i = n - 1; i >= 0; --i) {
                if(dominoes[i] == 'L') {
                    count = 1;
                    prev = 'L';
                } else if(dominoes[i] != '.') {
                    prev = dominoes[i];
                }

                if(prev == 'L' && dominoes[i] == '.') {
                    left[i] = count;
                    ++count;
                }
            }

            std::cout << "left" << std::endl;
            show_array(left);

            for(int i = 0; i < n; ++i) {
                if(!left[i] && !right[i]) {
                    result += dominoes[i];
                } else if (!left[i]) {
                    result += 'R';
                } else if (!right[i]) {
                    result += 'L';
                } else if (left[i] == right[i]) {
                    result += '.';
                } else if (left[i] < right[i]) {
                    result += 'L';
                } else {
                    result += 'R'; 
                }
            }

            return result;
        }
};

int main(void) {
    std::string case_1 = "RR.L";
    // => "RR.L"
    std::string case_2 =  ".L.R...LR..L..";
    // => "LL.RR.LLRRLL.."

    SolutionAns s_ans;

    //std::cout << s_ans.pushDominoes(case_1) << std::endl;
    std::cout << s_ans.pushDominoes(case_2) << std::endl;
}
