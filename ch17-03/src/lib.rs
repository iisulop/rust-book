use std::cell::Cell;

macro_rules! proxy_to_state {
    ($($func_name:ident),+) => {
        $(
            pub fn $func_name(&mut self) {
                if let Some(s) = self.state.take() {
                    self.state = Some(s.$func_name())
                }
            }
         )+
    };
}

pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    pub fn new() -> Self {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }

    pub fn add_text(&mut self, text: &str) -> Result<(), ()> {
        if self.state.as_ref().unwrap().allow_modify() {
            self.content.push_str(text);
            Ok(())
        } else {
            Err(())
        }
    }

    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(self)
    }

    proxy_to_state!(request_review, approve, reject);
}

trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn content<'a>(&self, _post: &'a Post) -> &'a str {
        ""
    }
    fn reject(self: Box<Self>) -> Box<dyn State>;
    fn allow_modify(&self) -> bool {
        false
    }
}

struct Draft {}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {
            approval_count: Cell::new(0),
        })
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn allow_modify(&self) -> bool {
        true
    }
}

struct PendingReview {
    approval_count: Cell<usize>,
}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        match self.approval_count.get() {
            0 => {
                self.approval_count.set(1);
                self
            }
            _ => Box::new(Published {}),
        }
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        Box::new(Draft {})
    }
}

struct Published {}

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }
}
