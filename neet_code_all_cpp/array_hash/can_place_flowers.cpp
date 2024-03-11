#include <cinttypes>
#include <iostream>
#include <vector>

// 模範解答
class SolutionAns {
    public:
        bool canPlaceFlowers(std::vector<int>& flowerbed, int n) {
            if(n == 0) {
                return true;
            }

            // 頭とケツに0を余計に追加
            flowerbed.insert(flowerbed.begin(), 0);
            flowerbed.push_back(0);

            for(int i = 1; i < flowerbed.size() - 1; ++i) {
                // 隣り合った3個がゼロなら1に変更
                if(flowerbed[i - 1] == 0 && flowerbed[i] == 0 && flowerbed[i + 1] == 0) {
                    flowerbed[i] = 1;
                    --n;
                }

                if(n == 0) {
                    return true;
                }
            }

            return false;
        }

};

// Rustの模範解答より
class SolutionAnsRust {
    public:
        // time O(N), space: O(1)
        bool canPlaceFlowers(std::vector<int>& flowerbed, int n) {
            int empty = 0;
            if(!flowerbed[0]) {
                empty = 1;
            }

            for(auto& f: flowerbed) {
                if(f) {
                    n -= (empty - 1) / 2;
                    empty = 0;
                } else {
                    empty += 1;
                }
            }

            n -=  empty / 2;

            return n <= 0;
        }

        // time O(N), space: O(1)
        bool canPlaceFlowers2(std::vector<int> flowerbed, int n) {
            for(int i = 0; i < flowerbed.size(); ++i) {
                if(n == 0) {
                    return true;
                }

                if((i == 0 || flowerbed[i - 1] == 0)
                        && flowerbed[i] == 0
                        && (i == flowerbed.size() - 1 || flowerbed[i + 1] == 0)
                        ) 
                {
                    flowerbed[i] = 1;
                    --n;
                }
            }

            return n == 0;
        }
};

int main(void) {
    using Case = std::pair<std::vector<int>, int>;

    Case case_1 = {{1, 0, 0, 0, 1}, 1};
    // => true
    Case case_2 = {{1, 0, 0, 0, 1}, 2};
    // => false
    Case case_3 = {{0, 0, 1, 0, 1}, 1};
    // => true
    Case case_4 = {{0, 0, 1, 0, 0}, 1};
    // => true

    SolutionAnsRust s_ans_rs;

    std::cout << "case_1: " << s_ans_rs.canPlaceFlowers(case_1.first, case_1.second) << std::endl;
    std::cout << "case_2: " << s_ans_rs.canPlaceFlowers(case_2.first, case_2.second) << std::endl;
    std::cout << "case_3: " << s_ans_rs.canPlaceFlowers(case_3.first, case_3.second) << std::endl;
    std::cout << "case_4: " << s_ans_rs.canPlaceFlowers(case_4.first, case_4.second) << std::endl;

    std::cout << "case_1: " << s_ans_rs.canPlaceFlowers2(case_1.first, case_1.second) << std::endl;
    std::cout << "case_2: " << s_ans_rs.canPlaceFlowers2(case_2.first, case_2.second) << std::endl;
    std::cout << "case_3: " << s_ans_rs.canPlaceFlowers2(case_3.first, case_3.second) << std::endl;
    std::cout << "case_4: " << s_ans_rs.canPlaceFlowers2(case_4.first, case_4.second) << std::endl;

    SolutionAns s_ans;

    std::cout << "case_1: " << s_ans.canPlaceFlowers(case_1.first, case_1.second) << std::endl;
    std::cout << "case_2: " << s_ans.canPlaceFlowers(case_2.first, case_2.second) << std::endl;
    std::cout << "case_3: " << s_ans.canPlaceFlowers(case_3.first, case_3.second) << std::endl;
    std::cout << "case_4: " << s_ans.canPlaceFlowers(case_4.first, case_4.second) << std::endl;
}
