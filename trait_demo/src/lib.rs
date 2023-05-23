use std::fmt::{Debug, Display};

pub trait Summary {
    fn summarize_auther(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_auther())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize_auther(&self) -> String {
        format!("@{}", self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }

    fn summarize_auther(&self) -> String {
        format!("@{}", self.username)
    }
}

// pub fn notify1(item: impl Summary + Display) {
//     println!("Breaking news! {}", item.summarize());
// }

// pub fn notify<T: Summary + Display>(item: T) {
//     println!("Breaking news! {}", item.summarize());
// }

// pub fn notify<T: Summary + Display, U:Clone + Debug> (a:T, b:U) -> String {
//     format!("Breaking news! {}", a.summarize())
// }

pub fn notify<T, U>(a: T, b: U) -> String
where
    T: Summary + Display,
    U: Clone + Debug,
{
    format!("Breaking news! {}", a.summarize())
}

pub fn notify1(s: &str) -> impl Summary {
    NewsArticle {
        headline: String::from("Breaking news!"),
        location: String::from("London"),
        author: String::from("John Smith"),
        content: String::from(s),
    }
}
