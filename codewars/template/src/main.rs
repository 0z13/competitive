
#![allow(dead_code)]
use std::collections::HashMap;

use std::collections::hash_map::DefaultHasher;
use std::hash::Hasher;

struct UrlShortener {
    shortToLong: HashMap<String, String>,
    longToShort: HashMap<String, String>,
    hasher: DefaultHasher,
}

impl UrlShortener {
    fn new() -> Self {
        Self {
            shortToLong: HashMap::new(),
            longToShort: HashMap::new(),
            hasher: DefaultHasher::new(),
         }
    }

    fn shorten(&self, long_url: &str) -> String {
        self.hasher.write(&str);
        let hash = self.hasher.finish();
        let hash: u8 = hash.into();
        let short_url = format!("{}{}", "short.ly/", hash.to_string());
        self.shortToLong.insert(short_url.clone(),long_url.to_string());
        self.longToShort.insert(long_url.to_string(), short_url.clone());
        short_url
    }

    fn redirect(&self, short_url: &str) -> String {
        let long_url = self.shortToLong.get(&short_url.to_string()).unwrap();
        long_url.clone()
    }
}

fn main() {
}

fn max_profit(quotes: &[u32]) -> u32{
    let mut v =  vec![];
    let last = 0;
    for i in quotes {
        if i 
    }
}
