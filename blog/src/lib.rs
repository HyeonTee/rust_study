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
    }
    
    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review());
        }
    }

    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            // approve()를 호출해서 상태를 전환
            let new_state = s.approve();

            // Published 상태인지 확인
            if new_state.as_ref().is_published() {
                self.approved_content = self.current_content.clone();
            }

            self.state = Some(new_state);
        }
    }

}

trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.approved_content
    }

    fn is_published(&self) -> bool {
        false
    }
}

struct Draft {}

struct PendingReview {}

struct Published {}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }
    
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
    
    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})
    }
}

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
    
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn is_published(&self) -> bool {
        true
    }
}