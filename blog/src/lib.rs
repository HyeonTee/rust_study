pub struct Post {
    state: Option<Box<dyn State>>,
    current_content: String,
    approved_content: String,
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            current_content: String::new(),
            approved_content: String::new(),
        }
    }

    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(self)
    }

    pub fn add_text(&mut self, text: &str) {
        self.current_content.push_str(text);
        if let Some(s) = self.state.take() {
            self.state = Some(s.add_text());
        }
    }

    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review());
        }
    }

    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve(self));
        }
    }

}

trait State {
    fn add_text<'a>(&self) -> Box<dyn State> {
        Box::new(Draft {})
    }
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>, post: &mut Post) -> Box<dyn State>;

    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.approved_content
    }
}

struct Draft {}

struct PendingReview {}

struct Published {}

impl State for Draft {    
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }

    fn approve(self: Box<Self>, post: &mut Post) -> Box<dyn State> {
        self
    }
}

impl State for PendingReview {    
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>, post: &mut Post) -> Box<dyn State> {
        post.approved_content = post.current_content.clone();
        Box::new(Published {})
    }
}

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>, post: &mut Post) -> Box<dyn State> {
        self
    }
}