use std::collections::HashMap;

// できなかった
#[derive(Debug)]
struct Codec {
    data: Vec<Vec<char>>,
}

impl Codec {
    pub fn new() -> Self {
        Self { data: vec![] }
    }

    pub fn encode(&mut self, longURL: String) -> String {
        let mut part = vec![];
        for c in longURL.chars() {
            if c == '/' {
                self.data.push(part.clone());
                part.clear();
            } else {
                part.push(c);
            }
        }
        self.data.push(part);
        let mut result = longURL[0..7].to_string();
        for i in 2..self.data.len() {
            result.push('/');
            result.push_str(&i.to_string());
        }

        result
    }

    pub fn decode(&self, shortURL: String) -> String {
        //for c in shortURL[8..].chars() {}
        let mut result = shortURL[0..7].to_string();
        for i in 2..self.data.len() {
            result.push('/');
            result.push_str(&self.data[i].iter().collect::<String>());
        }

        result
    }
}

// 模範解答
struct CodecAns {
    url_map: HashMap<String, String>,
    base: String,
}

impl CodecAns {
    pub fn new() -> Self {
        Self {
            url_map: HashMap::new(),
            base: "http://tinyurl.com".to_string(),
        }
    }

    pub fn encode(&mut self, longURL: String) -> String {
        let short_url = format!("{}{}", self.base, Self::generate_hash(&longURL));
        self.url_map.insert(short_url.clone(), longURL.clone());
        short_url
    }

    pub fn decode(&self, shortURL: String) -> String {
        self.url_map[&shortURL].clone()
    }

    fn generate_hash(url: &str) -> i64 {
        let mut hash = 5831;

        // お手本通りだとオーバーフローするのですが..(hash << 5)
        for c in url.chars() {
            //hash = ((hash << 5) + hash) + c as i64;
            hash += c as i64;
        }

        hash
    }
}

// C++の実装より
struct SolutionAnsCpp {
    encode_map: HashMap<String, String>,
    docode_map: HashMap<String, String>,
    base: String,
}

impl SolutionAnsCpp {
    pub fn new() -> Self {
        Self {
            encode_map: HashMap::new(),
            docode_map: HashMap::new(),
            base: "http://tinyurl.com".to_string(),
        }
    }

    pub fn encode(&mut self, longURL: String) -> String {
        if !self.encode_map.contains_key(&longURL) {
            let short_url = self.base.clone() + &(self.encode_map.len() + 1).to_string();
            self.encode_map.insert(longURL.clone(), short_url.clone());
            self.docode_map.insert(short_url, longURL.clone());
        }

        self.encode_map[&longURL].clone()
    }

    pub fn decode(&self, shortURL: String) -> String {
        self.docode_map[&shortURL].clone()
    }
}

fn main() {
    let case_1 = "https://leetcode.com/problems/design-tinyurl".to_string();
    /*
    let mut codec_0 = Codec::new();
    let encoded_1 = codec_0.encode(case_1.clone());
    println!("case_1: {}", encoded_1);
    let decoded_1 = codec_0.decode(case_1.clone());
    println!("case_1: {}", decoded_1);
    */

    let mut codec_ans = CodecAns::new();
    let encoded_1 = codec_ans.encode(case_1.clone());
    println!("case_1: {}", encoded_1);
    let decoded_1 = codec_ans.decode(encoded_1);
    println!("case_1: {}", decoded_1);

    let mut codec_ans_cpp = SolutionAnsCpp::new();
    let encoded_1 = codec_ans_cpp.encode(case_1.clone());
    println!("case_1: {}", encoded_1);
    let decoded_1 = codec_ans_cpp.decode(encoded_1);
    println!("case_1: {}", decoded_1);
}
