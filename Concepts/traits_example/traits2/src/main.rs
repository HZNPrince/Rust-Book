use traits::{SocialPost, NewsArticle, Summary};
fn main() {
    let post = SocialPost{
        username:String::from("Prince Mehta"),
        content: String::from("of course, as you people might already know
                                how is world is being played."),
        reply: false,
        repost: false,
    };
    println!("{}", post.summarize());

    let article = NewsArticle{
        author: "Prince Mehta".to_string(),
        headline: "Biggest fall of all time".to_string(),
        location: "New Jersey".to_string(),
        content: "Many suicides have been reported since the fall of crypto markets due to Trump deals with China".to_string(),
    };
    println!("The article is now summarize {}", article.summarize());
    
    traits::news(&article);

    
}
