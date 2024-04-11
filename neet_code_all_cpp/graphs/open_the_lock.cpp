#include <iostream>
#include <unordered_set>
#include <vector>
#include <string>
#include <queue>
#include <set>
#include <climits>
#include <cmath>

class Solution {
    public:
        // AC
        int openLock(std::vector<std::string>& deadends, std::string target) {
            std::set<std::vector<int>> deadend_set;
            for(auto& end: deadends) {
                deadend_set.insert(string_to_vec(end));
            }
            if(deadend_set.count(std::vector<int>{0,0,0,0})) {
                return -1;
            }
            auto target_vec = string_to_vec(target);

            std::set<std::vector<int>> seen;
            std::queue<std::vector<int>> q;

            seen.insert(std::vector<int>{0,0,0,0});
            q.push(std::vector<int>{0,0,0,0});

            std::vector<int> dist(10000, INT_MAX);
            dist[0] = 0;

            while(!q.empty()) {
                auto q_front = q.front();
                q.pop();
                auto idx_v = state_to_num(q_front);

                for(int i = 0; i < 4; ++i){
                    auto direction = std::vector{1, -1};
                    for(auto& d: direction) {
                        auto q_front_next = q_front;
                        q_front_next[i] += d;

                        if(q_front_next[i] == -1) {
                            q_front_next[i] = 9;
                        }
                        if(q_front_next[i] == 10) {
                            q_front_next[i] = 0;
                        }

                        if(q_front_next[i] < -1
                                || q_front_next[i] > 10
                                || seen.count(q_front_next)
                                || deadend_set.count(q_front_next)) {
                            continue;
                        }

                        auto idx_next_v = state_to_num(q_front_next);
                        dist[idx_next_v] = std::min(dist[idx_next_v], dist[idx_v] + 1);

                        seen.insert(q_front_next);
                        q.push(q_front_next);
                    }
                }
            }

            auto idx_target = state_to_num(target_vec);
            if(dist[idx_target] == INT_MAX) {
                return -1;
            }

            return dist[idx_target];
        }

    private:
        std::vector<int> string_to_vec(std::string& s) {
            std::vector<int> result;
            for(auto&c: s) {
                result.push_back(c - '0');
            }

            return result;
        }

        int state_to_num(std::vector<int>& state) {
            int num = 0;
            for(int i = 3; i >= 0; --i) {
                num += state[i] * std::pow(10, 3 - i);
            }

            return num;
        }
};

// C#の模範解答より: 合わない...
class SolutionAnsCs {
    public:
        int openLock(std::vector<std::string>& deadends, std::string target) {
            std::unordered_set<std::string> deadend_set;
            for(auto& end: deadends) {
                deadend_set.insert(end);
            }
            if(deadend_set.count("0000") || deadend_set.count(target)) {
                return -1;
            }

            std::queue<std::string> q;
            std::unordered_set<std::string> visit;
            q.push("0000");
            visit.insert("0000");
            int result = 0;
            while(!q.empty()) {
                for(int i = 0; i < (int)q.size(); ++i) {
                    auto current = q.front();
                    q.pop();
                    if(current == target) {
                        return result;
                    }

                    for(auto& s: get_neighbors(current)) {
                        if(!visit.count(s) && !deadend_set.count(s)) {
                            q.push(s);
                            visit.insert(s);
                        }
                    }
                }
                ++result;
            }

            return -1;
        }

        std::vector<std::string> get_neighbors(std::string s) {
            std::vector<std::string> result;
            for(int i = 0; i < (int)s.size(); ++i) {
                auto s_1 = s;
                auto s_2 = s;

                if(s_1[i] == '9') {
                    s_1[i] = '0';
                } else {
                    ++s_1[i];
                }
                result.push_back(s_1);

                if(s_2[i] == '0') {
                    s_2[i] = '9';
                } else {
                    --s_2[i];
                }
                result.push_back(s_2);
            }

            return result;
        }
};

int main(void) {
    using Case = std::pair<std::vector<std::string>, std::string>;
    Case case_1 = {{"0201","0101","0102","1212","2002"}, "0202"};
    // => 6
    Case case_2 = {{"8888"}, "0009"};
    // => 1
    Case case_3 = {{"8887","8889","8878","8898","8788","8988","7888","9888"}, "8888"};
    // => -1

    Solution s_1;
    std::cout << s_1.openLock(case_1.first, case_1.second) << std::endl;
    std::cout << s_1.openLock(case_2.first, case_2.second) << std::endl;
    std::cout << s_1.openLock(case_3.first, case_3.second) << std::endl;

    /*
    SolutionAnsCs s_ans_cs;
    std::cout << s_ans_cs.openLock(case_1.first, case_1.second) << std::endl;
    std::cout << s_ans_cs.openLock(case_2.first, case_2.second) << std::endl;
    std::cout << s_ans_cs.openLock(case_3.first, case_3.second) << std::endl;
    */
}
