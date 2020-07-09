use ch17_03::Post;

fn main() {
    let mut post = Post::new();

    assert_eq!(Ok(()), post.add_text("I ate a salad for lunch today"));
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    assert_eq!(Err(()), post.add_text("\nNothing else"));

    post.reject();
    assert_eq!("", post.content());

    assert_eq!(Ok(()), post.add_text("\nNothing else"));

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("", post.content());
    post.approve();
    assert_eq!(Err(()), post.add_text("\nNothing else"));
    assert_eq!(
        "I ate a salad for lunch today\nNothing else",
        post.content()
    );
    println!("{}", post.content())
}
