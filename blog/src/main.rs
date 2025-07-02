use blog::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());
    
    post.add_text(", and I ate a sandwich for dinner today");
    assert_eq!("I ate a salad for lunch today", post.content());
    
    post.request_review();
    assert_eq!("I ate a salad for lunch today", post.content());
    
    post.approve();
    assert_eq!("I ate a salad for lunch today, and I ate a sandwich for dinner today", post.content());
}