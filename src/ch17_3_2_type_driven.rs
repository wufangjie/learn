use learn::dbgt;

struct Post {
    content: String,
}

impl Post {
    fn new() -> DraftPost {
        DraftPost {
            content: String::from(""),
        }
    }

    fn content(&self) -> &str {
        &self.content[..]
    }
}

struct DraftPost {
    content: String,
}

impl DraftPost {
    fn request_review(self) -> PendingPost {
        PendingPost {
            count: 0,
            content: self.content,
        }
    }

    fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }
}

struct PendingPost {
    count: i32,
    content: String,
}

impl PendingPost {
    fn approve(self) -> Post {
        // FIXME: I can not implement twice approve
        // https://users.rust-lang.org/t/chapter-17-3-multiple-approvals-with-type-system/62290
        Post {
            content: self.content,
        }
    }

    fn reject(self) -> DraftPost {
        DraftPost {
            content: self.content,
        }
    }
}

#[test]
fn test() {
    let mut post = Post::new();
    post.add_text("hello world");
    let post = post.request_review();
    let post = post.approve();
    println!("{}", post.content());
}

#[derive(Debug)]
enum State {
    Draft,
    Pending,
    Published,
}

struct EnumPost {
    state: State,
    content: String,
    approve_count: u32,
}

impl EnumPost {
    fn new() -> EnumPost {
        EnumPost {
            state: State::Draft,
            content: String::new(),
            approve_count: 0,
        }
    }

    fn print_state(&self) {
        println!("Current Post State: {:?}", self.state);
    }

    fn add_text(&mut self, text: &str) {
        if let State::Draft = self.state {
            self.content.push_str(text);
        }
    }

    fn request_review(&mut self) {
        if let State::Draft = self.state {
            self.state = State::Pending;
        }
    }

    fn approve(&mut self) {
        if let State::Pending = self.state {
            if self.approve_count == 1 {
                self.state = State::Published;
            } else {
                self.approve_count += 1;
            }
        }
    }

    fn reject(&mut self) {
        if let State::Pending = self.state {
            self.state = State::Draft;
            self.approve_count = 0;
        }
    }

    fn content(&mut self) -> &str {
        if let State::Published = self.state {
            &self.content[..]
        } else {
            ""
        }
    }
}

#[test]
fn test_enum() {
    let mut post = EnumPost::new();
    post.add_text("hello enum post");
    post.print_state();
    post.request_review();
    post.print_state();
    post.approve();
    post.print_state();
    post.approve();
    post.print_state();
    println!("{}", post.content());
}
