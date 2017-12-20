extern crate egg_mode;
extern crate tokio_core;
extern crate futures;

use tokio_core::reactor::Core;
use egg_mode::tweet::DraftTweet;
use std::io::{self, Read, Write};

fn main() { 
    let base_url = "https://twitter.com";

    let mut tokio_core = Core::new().unwrap();
    let tokio_handle = tokio_core.handle();
    
    let consumer_key = include_str!("../keys/consumer_key").trim();
    let consumer_secret = include_str!("../keys/consumer_secret").trim();
    let access_token_key = include_str!("../keys/access_token_key").trim();
    let access_token_secret = include_str!("../keys/access_token_secret").trim();

    let consumer_keys = egg_mode::KeyPair::new(consumer_key, consumer_secret);
    let access_keys = egg_mode::KeyPair::new(access_token_key, access_token_secret);

    let token = egg_mode::Token::Access {
        consumer: consumer_keys,
        access: access_keys,
    };

    let botuser = tokio_core.run(egg_mode::user::show("t1ldebot", &token, &tokio_handle)).unwrap();

    let mut tweet_text = String::new();
    std::io::stdin().read_to_string(&mut tweet_text).expect("");

    let draft = DraftTweet::new(tweet_text.to_string());
    println!("Sending Tweet:\n{}", tweet_text);
    
    let tweet_result = tokio_core.run(draft.send(&token, &tokio_handle));

    match tweet_result {
        Ok(tweet) => if let Some(ref user) = tweet.user {
            println!("Tweet Sent:\n{}/{}/status/{}", base_url, user.screen_name, tweet.id);
        },
        Err(err) => println!("Error: {:?}", err),
    }
    
}
