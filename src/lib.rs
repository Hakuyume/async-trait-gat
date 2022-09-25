use std::future::{self, Future, Ready};
use std::pin::Pin;
use std::task::{Context, Poll};

// https://doc.rust-lang.org/book/ch10-02-traits.html#defining-a-trait
pub trait Summary {
    type SummarizeFuture<'a>: Future<Output = String>
    where
        Self: 'a;
    fn summarize<'a>(&'a self) -> Self::SummarizeFuture<'a>;
}

// https://doc.rust-lang.org/book/ch10-02-traits.html#implementing-a-trait-on-a-type
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    type SummarizeFuture<'a> = Ready<String>;
    fn summarize<'a>(&'a self) -> Self::SummarizeFuture<'a> {
        future::ready(format!(
            "{}, by {} ({})",
            self.headline, self.author, self.location
        ))
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    type SummarizeFuture<'a> = TweetSummarizeFuture<'a>;
    fn summarize<'a>(&'a self) -> Self::SummarizeFuture<'a> {
        TweetSummarizeFuture(self)
    }
}

pub struct TweetSummarizeFuture<'a>(&'a Tweet);

impl Future for TweetSummarizeFuture<'_> {
    type Output = String;
    fn poll(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Self::Output> {
        Poll::Ready(format!(
            "{}: {}",
            self.as_ref().0.username,
            self.as_ref().0.content
        ))
    }
}

// https://doc.rust-lang.org/book/ch10-02-traits.html#trait-bound-syntax
pub async fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize().await);
}
