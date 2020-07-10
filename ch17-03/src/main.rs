use ch17_03::*;

fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    let post = post.request_review();

    let mut post = post.reject();
    post.add_text("\nNothing else");
    let post = post.request_review();

    let post = match post.approve() {
        PostType::PendingReviewPost(post) => match post.approve() {
            PostType::Post(post) => post,
            _ => unreachable!(),
        },
        _ => unreachable!(),
    };
    assert_eq!(
        "I ate a salad for lunch today\nNothing else",
        post.content()
    );
    println!("{}", post.content())
}
