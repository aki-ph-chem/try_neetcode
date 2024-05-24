#include <cwchar>
#include <iostream>
#include <string>
#include <unordered_map>

class Solution {
private:
    std::unordered_map<std::string, std::string> encodeMap;
    std::unordered_map<std::string, std::string> decodeMap;
    std::string base = "http://tinyurl.com/";

public:
    // Encodes a URL to a shortened URL.
    std::string encode(std::string longUrl) {
        if(!encodeMap.count(longUrl)) {
            std::string shortUrl = base + std::to_string(encodeMap.size() + 1);
            encodeMap[longUrl] = shortUrl;
            decodeMap[shortUrl] = longUrl;
        }
        return encodeMap[longUrl];
    }

    // Decodes a shortened URL to its original URL.
    std::string decode(std::string shortUrl) {
        return decodeMap[shortUrl];
    }
};

int main(void) {
    std::string case_1 = "https://leetcode.com/problems/design-tinyurl";
    Solution s_1;

    auto encorded_1 = s_1.encode(case_1);
    std::cout << encorded_1 << std::endl;
    auto decorded_1 = s_1.decode(encorded_1);
    std::cout << decorded_1 << std::endl;
}
