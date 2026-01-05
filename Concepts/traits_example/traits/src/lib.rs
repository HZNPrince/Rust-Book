use std::fmt::Display;

pub trait Summary {
    fn summarize_author(&self) -> String;
    fn summarize(&self) -> String{
        format!("Read more from ...{}", self.summarize_author())
    }
}

pub struct NewsArticle{
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}
impl Summary for NewsArticle{
    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }
    fn summarize(&self) -> String {
        format!("{}, by {} {} /n {}", self.headline, self.author, self.location, self.content)
    }
}
pub struct SocialPost{
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub repost: bool,
}
impl Summary for SocialPost{
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

pub fn news<T>(item: &T) 
where T: Summary
{
    println!("Breaking new! {}", item.summarize())
}