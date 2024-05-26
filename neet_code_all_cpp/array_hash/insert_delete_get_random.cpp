#include <iostream>
#include <unordered_set>
#include <unordered_map>
#include <random>
#include <vector>

// 解けなかった
class RandomizedSet {
    private:
        std::unordered_set<int> set;
        std::mt19937_64 mt64;

    public:
        RandomizedSet() 
        : mt64(0){}

        bool insert(int val) {
            if(set.count(val)) {
                return false;
            }

            set.insert(val);
            return true;
        }

        bool remove(int val) {
            if(!set.count(val)) {
                return false;
            }

            set.erase(val);
            return true;
        }

        int getRandom() {
            std::uniform_int_distribution<int> rand_int(0, (int)set.size());
            int idx_rand = rand_int(mt64);

            return 1;
        }
};

// 模範解答
class RandomizedSetAns {
    public:
        std::unordered_map<int, int> indices;
        std::vector<int> values;

        RandomizedSetAns() {
        }

        bool insert(int val) {
            if(!indices.count(val)) {
                values.push_back(val);
                indices[val] = values.size() - 1;  
                return true;
            }

            return false;
        }

        bool remove(int val) {
            if(!indices.count(val)) {
                return false;
            }

            int idx = indices[val];
            indices[values[values.size() - 1]] = idx;
            values[idx] = values[values.size() - 1];

            values.pop_back();
            indices.erase(val);

            return true;
        }

        int getRandom() {
            return values[rand() % values.size()];
        }

};

int main(void) {
    RandomizedSetAns r_ans;

    std::cout << r_ans.insert(1) << std::endl;
    std::cout << r_ans.remove(2) << std::endl;
    std::cout << r_ans.insert(2) << std::endl;
    std::cout << r_ans.getRandom() << std::endl;
    std::cout << r_ans.remove(1) << std::endl;
    std::cout << r_ans.insert(2) << std::endl;
    std::cout << r_ans.getRandom() << std::endl;
}
