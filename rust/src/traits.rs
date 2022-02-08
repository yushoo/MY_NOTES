// Traits

pub struct NewsArticle {
    pub author: String,
    pub headline: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {}", self.headline, self.author)
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
        format!("{}, by {}", self.username, self.content)
    }
}

pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

pub trait Summary_with_default {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}

// This function can only take any item that implements Summary traits
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

pub fn run() {
    let tweet = Tweet {
        username: String::from("@johndoe"),
        content: String::from("Hello world!"),
        reply: false,
        retweet: false,
    };

    let article = NewsArticle {
        author: String::from("John doe"),
        headline: String::from("The sky is falling"),
        content: String::from("They sky is not actually falling"),
    };

    println!("Tweet summary: {}", tweet.summarize());
    println!("Article summary: {}", article.summarize());
}
