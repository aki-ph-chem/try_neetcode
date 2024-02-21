use std::collections::{HashMap, HashSet};

// 解けなかった
struct Twitter {
    tweet: Vec<(i32, i32)>,
    map: HashMap<i32, HashSet<i32>>,
}

impl Twitter {
    fn new() -> Self {
        Self {
            tweet: vec![],
            map: HashMap::new(),
        }
    }

    fn post_tweet(&mut self, user_id: i32, tweet_id: i32) {
        self.tweet.push((user_id, tweet_id));
        if !self.map.contains_key(&user_id) {
            self.map.insert(user_id, HashSet::new());
        }
    }

    fn get_news_feed(&self, user_id: i32) -> Vec<i32> {
        let mut result = vec![];
        let mut counter = 0;

        for (id, tweet) in &self.tweet {
            if self.map.get(&user_id).unwrap().contains(id) {
                if counter > 10 {
                    return result;
                }

                result.push(*tweet);
                counter += 1;
            }
        }

        result
    }

    fn follow(&mut self, follower_id: i32, fllowee_id: i32) {
        if self.map.contains_key(&follower_id) {
            if let Some(follow_list) = self.map.get_mut(&follower_id) {
                follow_list.insert(fllowee_id);
            }
        } else {
            let mut set_tmp = HashSet::new();
            set_tmp.insert(fllowee_id);
            self.map.insert(follower_id, set_tmp);
        }
    }

    fn unfollow(&mut self, follower_id: i32, followee_id: i32) {
        if let Some(follow_list) = self.map.get_mut(&follower_id) {
            follow_list.take(&followee_id);
        }
    }
}

// AC
struct TwitterAnsCpp {
    posts: Vec<(i32, i32)>,
    follow_map: HashMap<i32, HashSet<i32>>,
}

impl TwitterAnsCpp {
    fn new() -> Self {
        Self {
            posts: vec![],
            follow_map: HashMap::new(),
        }
    }

    fn post_tweet(&mut self, user_id: i32, tweet_id: i32) {
        self.posts.push((user_id, tweet_id));
    }

    fn get_news_feed(&self, user_id: i32) -> Vec<i32> {
        let mut counter = 0;
        let mut result = vec![];

        for post in self.posts.iter().rev() {
            if counter == 10 {
                break;
            }

            let (following_id, tweet_id) = post;
            if let Some(following_list) = self.follow_map.get(&user_id) {
                if following_list.contains(following_id) || *following_id == user_id {
                    result.push(*tweet_id);
                    counter += 1;
                }
            } else if *following_id == user_id {
                result.push(*tweet_id);
                counter += 1;
            }
        }

        result
    }

    fn follow(&mut self, follower_id: i32, followee_id: i32) {
        if self.follow_map.contains_key(&follower_id) {
            if let Some(follow_list) = self.follow_map.get_mut(&follower_id) {
                follow_list.insert(followee_id);
            }
        } else {
            let mut set_tmp = HashSet::new();
            set_tmp.insert(followee_id);
            self.follow_map.insert(follower_id, set_tmp);
        }
    }

    fn unfollow(&mut self, follower_id: i32, followee_id: i32) {
        if let Some(follow_list) = self.follow_map.get_mut(&follower_id) {
            follow_list.take(&followee_id);
        }
    }
}

fn main() {
    /*
    let mut twitter_1 = Twitter::new();
    twitter_1.post_tweet(1, 5);
    println!("{:?}", twitter_1.get_news_feed(1));
    // => [5]
    twitter_1.follow(1, 2);
    twitter_1.post_tweet(2, 6);
    println!("{:?}", twitter_1.get_news_feed(1));
    // => [6, 5]
    twitter_1.unfollow(1, 2);
    println!("{:?}", twitter_1.get_news_feed(1));
    // => [5]
    */

    let mut twitter_2 = TwitterAnsCpp::new();
    twitter_2.post_tweet(1, 5);
    println!("{:?}", twitter_2.get_news_feed(1));
    // => [5]
    twitter_2.follow(1, 2);
    twitter_2.post_tweet(2, 6);
    println!("{:?}", twitter_2.get_news_feed(1));
    // => [6, 5]
    twitter_2.unfollow(1, 2);
    println!("{:?}", twitter_2.get_news_feed(1));
    // => [5]
}
