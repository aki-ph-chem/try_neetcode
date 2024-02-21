#include <iostream>
#include <vector>
#include <unordered_map>
#include <unordered_set>

// 模範解答
class TwitterAns {
    private:
        std::vector<std::pair<int, int>> posts;
        std::unordered_map<int, std::unordered_set<int>> followMap;

    public:
        TwitterAns() {}

        void postTweet(int userId, int tweetId) {
            posts.push_back({userId, tweetId});
        }

        std::vector<int> getNewsFeed(int userId) {
            int count = 10;
            std::vector<int> result;

            for(int i = posts.size() - 1; i >= 0; --i){
                if(count == 0) {
                    break;
                }
                auto [followingId, tweetId] = posts[i];
                auto following = followMap[userId];

                if(following.find(followingId) != following.end() 
                        || followingId == userId) {
                    result.push_back(tweetId);
                    --count;
                }

            }

            return result;
        }

        void follow(int followerId, int followeeId) {
            followMap[followerId].insert(followeeId);
        }

        void unfollow(int followerId, int followeeId) {
            followMap[followerId].erase(followeeId);
        }
};

void show_result(std::vector<int>& result) {
    for(auto& v: result) {
        std::cout << v << " ";
    }

    std::cout << '\n';
}

int main(void) {
    TwitterAns twitter_1;

    twitter_1.postTweet(1, 5);
    auto news_feed_1 = twitter_1.getNewsFeed(1);
    show_result(news_feed_1);
    // => [5]
    twitter_1.follow(1, 2);
    twitter_1.postTweet(2, 6);
    auto news_feed_2 = twitter_1.getNewsFeed(1);
    show_result(news_feed_2);
    // => [6,5]
    twitter_1.unfollow(1, 2);
    auto news_feed_3 = twitter_1.getNewsFeed(1);
    show_result(news_feed_3);
    // => [5]
}
