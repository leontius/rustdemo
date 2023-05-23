use trait_demo::NewsArticle;
use trait_demo::Summary;
use trait_demo::Tweet;

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Preston, North Carolina, United States"),
        author: String::from("Iceburgh"),
        content: String::from("The Penguins once again are the best hockey team in the NHL."),
    };

    println!("1 new tweet: {}", article.summarize());
}
