pub struct Post {
    content: String,
}

pub struct DraftPost {
    content: String,
}

pub struct PendingReviewPost {
    approve_count: usize,
    content: String,
}

pub enum PostType {
    Post(Post),
    DraftPost(DraftPost),
    PendingReviewPost(PendingReviewPost),
}

impl Post {
    pub fn new() -> DraftPost {
        DraftPost {
            content: String::new(),
        }
    }

    pub fn content(&self) -> &str {
        &self.content
    }
}

impl DraftPost {
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn request_review(self) -> PendingReviewPost {
        PendingReviewPost {
            content: self.content,
            approve_count: 0,
        }
    }
}

impl PendingReviewPost {
    pub fn approve(mut self) -> PostType {
        if self.approve_count > 0 {
            PostType::Post(Post {
                content: self.content,
            })
        } else {
            self.approve_count += 1;
            PostType::PendingReviewPost(self)
        }
    }

    pub fn reject(self) -> DraftPost {
        DraftPost {
            content: self.content,
        }
    }
}
