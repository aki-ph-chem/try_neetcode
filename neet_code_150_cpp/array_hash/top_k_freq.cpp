#include <iostream>
#include <algorithm>
#include <vector>
#include <unordered_map>
//#define DEBUG_ANS

class Solution {
    public:
        std::vector<int> top_k_freq(const std::vector<int>& nums, int k) {
            std::unordered_map<int,int> map;

            for(const auto &n : nums) {
                if(map.find(n) != map.end()) {
                    map.insert(std::make_pair(n, 0));
                }
                ++map[n];
            }

            /*
            for(const auto &v: map) {
                std::cout << "(" << v.first << "," << v.second << ")" << std::endl;
            }
            */

            return std::vector{1,2};
        }
};


class SolutionAns {
    public:
        std::vector<int> topKFrequent(std::vector<int>& nums, int k) {
            int n = nums.size();

            std::unordered_map<int, int> m;
            for (int i = 0; i < n; i++) {
                m[nums[i]]++;
            }
#ifdef DEBUG_ANS
            std::cerr << "m" << std::endl;
            for(const auto &v: m) {
                std::cout << "(" << v.first << "," << v.second << ")" << std::endl;
            }
#endif
            std::vector<std::vector<int>> buckets(n + 1);
            for (auto it = m.begin(); it != m.end(); it++) {
                buckets[it->second].push_back(it->first);
            }
#ifdef DEBUG_ANS
            std::cerr << "bukets" << std::endl;
            for(const auto &v: buckets) {
                for(const auto &w: v) {
                    std::cerr << w << std::endl;
                }
            }
#endif
            std::vector<int> result;

            for (int i = n; i >= 0; i--) {
                if (static_cast<int>(result.size()) >= k) {
                    break;
                }
                if (!buckets[i].empty()) {
                    // bukets[i]の要素をresultに後ろから挿入
                    result.insert(result.end(), buckets[i].begin(), buckets[i].end());
                }
            }

            return result;
        }
};

// 時間を置いて解いた別解
class SolutionLatter {
    public:
        std::vector<int> topKFrequent(std::vector<int>& nums, int k) {
            std::unordered_map<int,int> map;
            for(auto&v: nums) {
                ++map[v];
            }

            std::vector<std::pair<int,int>> map_2;
            for(auto& [key, v]: map) {
                map_2.push_back({v, key});
            }
            std::sort(map_2.begin(), map_2.end(),[](auto& a, auto& b) {return a.first > b.first;});

            std::vector<int> result;
            for(auto& [key, v]: map_2) {
                if(k > 0) {
                    result.push_back(v);
                    --k;
                } else {
                    break;
                }
            }

            return result;
        }
};

int main(void) {
    auto case_1_array = std::vector{1,1,1,2,2,3};
    auto case_1_k = 2;

    SolutionAns s_2;
    auto res_case_1_ans = s_2.topKFrequent(case_1_array, case_1_k);
    for(const auto &v: res_case_1_ans) {
        std::cout << v << " ";
    }
    std::cout << std::endl;
}
