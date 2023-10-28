#include <iostream>
#include <utility>
#include <vector>
#include <unordered_map>

// AC
class TimeMap {
    public:
        std::unordered_map<std::string, std::vector<std::pair<int, std::string>>> value_set;
        TimeMap() {}

        void set(std::string key, std::string value, int timestamp) {
            if(value_set.find(key) != value_set.end()) {
                value_set[key].push_back(std::make_pair(timestamp, value));
            } else {
                value_set.insert(std::make_pair(key, std::vector{std::make_pair(timestamp, value)}));
            }
        }

        std::string get(std::string key, int timestamp) {
            if(value_set.find(key) != value_set.end()) {
                int left = 0, right = value_set[key].size() - 1;
                int mid = (left + right) / 2;

                while(left <= right) {
                    if(value_set[key][mid].first <= timestamp) {
                        left = mid + 1;
                    } else {
                        right = mid - 1;
                    }
                    mid = (left + right) / 2;
                }

                std::string result;
                if(value_set[key][mid].first <= timestamp) {
                    result = value_set[key][mid].second;
                } else {
                    result = "";
                }

                std::cerr << result << std::endl;
                return result;

            } else {
                std::cerr << "" << std::endl;
                return "";
            }
        }
};


/*
    Design time-based key-value structure, multiple vals at diff times

    Hash map, since timestamps are naturally in order, binary search

    Time: O(log n)
    Space: O(n)
*/

class TimeMapAns {
    public:
        TimeMapAns() {

        }

        void set(std::string key, std::string value, int timestamp) {
            m[key].push_back({timestamp, value});
        }

        std::string get(std::string key, int timestamp) {
            if (m.find(key) == m.end()) {
                return "";
            }

            int low = 0;
            int high = m[key].size() - 1;

            while (low <= high) {
                int mid = low + (high - low) / 2;
                if (m[key][mid].first < timestamp) {
                    low = mid + 1;
                } else if (m[key][mid].first > timestamp) {
                    high = mid - 1;
                } else {
                    return m[key][mid].second;
                }
            }

            if (high >= 0) {
                return m[key][high].second;
            }
            return "";
        }
    private:
        std::unordered_map<std::string, std::vector<std::pair<int, std::string>>> m;
};


int main(void) {
    TimeMap m_1;
    m_1.set("foo", "bar", 1);

    m_1.get("foo", 1); // "bar"
    m_1.get("foo", 3); // "bar"

    m_1.set("foo", "bar2", 4);

    m_1.get("foo", 4); // "bar2"
    m_1.get("foo", 5); // "bar2"

    TimeMap m_2;

    m_2.set("love", "high", 10);
    m_2.set("love", "low", 20);

    m_2.get("love", 5);
    m_2.get("love", 10);
    m_2.get("love", 15);
    m_2.get("love", 20);
    m_2.get("love", 25);

    TimeMapAns m_2_ans;

    m_2_ans.set("love", "high", 10);
    m_2_ans.set("love", "low", 20);

    std::cout << m_2_ans.get("love", 5) << std::endl;
    std::cout << m_2_ans.get("love", 10) << std::endl;
    std::cout << m_2_ans.get("love", 15) << std::endl;
    std::cout << m_2_ans.get("love", 20) << std::endl;
    std::cout << m_2_ans.get("love", 25) << std::endl;
}
