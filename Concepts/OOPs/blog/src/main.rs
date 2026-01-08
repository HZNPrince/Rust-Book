use blog;
pub mod blog2;

fn main() {
    let mut post = blog::Post::new();

    post.add_text("I am going to finish Rust Book before Q1");

    assert_eq!("", post.content());
}
