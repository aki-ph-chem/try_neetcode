#include <iostream>
#include <vector>
#include <stack>


// 解けなかった
class Solutoin {
    public:
        std::vector<int> asteroidCollision(std::vector<int>& asteroids) {
            std::stack<int> stk;
            for(auto& a: asteroids) {
                if(stk.empty()) {
                    stk.push(a);
                } else  {
                    if(stk.top() < 0 ||(stk.top() > 0 && a > 0)) {
                        stk.push(a);
                    } else if((stk.top() * a < 0) && stk.top() == std::abs(a)) {
                        stk.pop();
                    } else if(stk.top() < std::abs(a)) {
                        while(!stk.empty() && stk.top() < std::abs(a)) {
                            stk.pop();
                        }
                        if(stk.empty()) {
                            stk.push(a);
                        }
                    }
                }
            }

            std::vector<int> result(stk.size());
            int i = stk.size() - 1;
            while(!stk.empty()) {
                result[i] = stk.top();
                stk.pop();
                --i;
            }
            return result;
        }
};

void show(std::vector<int> result) {
    for(auto& a: result) {
        std::cout << a << " ";
    }
    std::cout << std::endl;
}

// 模範解答
class SolutionAns {
    public:
        std::vector<int> asteroidCollision(std::vector<int>& asteroids) {
            std::vector<int> stk;
            for(auto ast: asteroids) {
                while(!stk.empty() && stk.back() > 0 && ast < 0) {
                    int diff = ast + stk.back();
                    if(diff > 0) {
                        ast = 0;
                    } else if (diff < 0) {
                        stk.pop_back();
                    } else {
                        ast = 0;
                        stk.pop_back();
                    }
                }
                if(ast != 0) {
                    stk.push_back(ast);
                }
            }

            return stk;
        }
};

int main(void) {
    std::vector<int> case_1 = {5, 10, -5};
    // => [5, 10}
    std::vector<int> case_2 = {8, -8};
    // => [}
    std::vector<int> case_3 = {10, 2, -5};
    // => [10}
    std::vector<int> case_4 = {-2, -1, 1, 2};
    // => [-2,-1,1,2}
    
    /*
    Solutoin s_1;
    auto res_1 =s_1.asteroidCollision(case_1);
    auto res_2 =s_1.asteroidCollision(case_2);
    auto res_3 =s_1.asteroidCollision(case_3);
    auto res_4 =s_1.asteroidCollision(case_4);
    show(res_1);
    show(res_2);
    show(res_3);
    show(res_4);
    */

    SolutionAns s_ans;
    auto res_1 = s_ans.asteroidCollision(case_1);
    auto res_2 = s_ans.asteroidCollision(case_2);
    auto res_3 = s_ans.asteroidCollision(case_3);
    auto res_4 = s_ans.asteroidCollision(case_4);
    show(res_1);
    show(res_2);
    show(res_3);
    show(res_4);
}
