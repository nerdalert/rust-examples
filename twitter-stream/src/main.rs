use serde::Deserialize;
use twitter_stream::rt::{self, Future, Stream};
use twitter_stream::{Token, TwitterStreamBuilder};

#[derive(Deserialize, Debug)]
struct Tweet {
    created_at: String,
    id: u64,
    text: String,
    user: User,
}

#[derive(Deserialize, Debug)]
struct User {
    id: u64,
    screen_name: String,
}

// Change this value to change what keyword to stream from Twitter
static TWITTER_KEYWORD: &str = "kittens";

fn main() {
    // replace the token values with from your Twitter account:
    // https://developer.twitter.com/en/apps
    let token = Token::new(
        "consumer_key",
        "consumer_secret",
        "access_key",
        "access_secret",
    );

    println!(
        "Streaming Twitter for Tweets containing the keyword [{}]:",
        TWITTER_KEYWORD
    );
    let future = TwitterStreamBuilder::filter(token)
        .track(Some(TWITTER_KEYWORD))
        .listen()
        .unwrap()
        .flatten_stream()
        .for_each(|json| {
            process_results(json.to_string());
            Ok(())
        })
        .map_err(|e| println!("error: {}", e));

    rt::run(future);
}

// process and deserialize the json results from the stream
fn process_results(json: String) {
    // deserialize the json against the Tweet struct
    let feed: Tweet = serde_json::from_str(&json).unwrap();
    println!("User: {} Tweeted: {}", feed.user.screen_name, feed.text);
}

/*
Example Output:
--------------
Streaming Twitter for Tweets containing the keyword [kittens]:
User: ftinftehah Tweeted: RT @mypaws: Kittens in hats https://t.co/HgUuNCk11q
User: BirchCatalyst Tweeted: RT @mypaws: Kittens in hats https://t.co/HgUuNCk11q
User: akuluthfi Tweeted: RT @landpsychology: Twin ginger kittens https://t.co/2o5581HXvJ
User: schofield_kid_ Tweeted: They're so sweet! “@landpsychology: Twin ginger kittens https://t.co/iQb8l1nm3n”
User: SaiyanPrince69 Tweeted: RT @SoyBoyManBun: Many different types of animals were displaced in Australia's bushfires.
User: BRING1DTONEWORL Tweeted: Kittens in a blanket are too cute to handle.
*/
