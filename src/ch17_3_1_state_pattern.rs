struct Post {
    state: Box<dyn State>,
    content: String,
}

impl Post {
    fn new() -> Post {
        Post {
            state: Box::new(Draft::new()),
            content: String::new(),
        }
    }

    fn add_text(&mut self, text: &str) {
        match self.state.add_text(text) {
            "" => (),
            s => self.content.push_str(s),
        }
    }

    fn change_state(&mut self, s: &'static str) {
        match s {
            "Draft" => self.state = Box::new(Draft::new()),
            "Pending" => self.state = Box::new(Pending::new()),
            "Published" => self.state = Box::new(Published::new()),
            _ => (),
        }
    }

    fn request_review(&mut self) {
        let s = self.state.request_review();
        self.change_state(s);
    }

    fn approve(&mut self) {
        let s = self.state.approve();
        self.change_state(s);
    }

    fn reject(&mut self) {
        let s = self.state.reject();
        self.change_state(s);
    }

    fn content(&self) -> &str {
        // NOTE: we can not use match here to do different actions
        self.state.content(&self.content[..])
    }

    fn print_current_state(&self) {
        self.state.print_current_state()
    }
}

trait State {
    fn print_current_state(&self);

    fn add_text<'a>(&mut self, _text: &'a str) -> &'a str {
        ""
    }

    fn request_review(&mut self) -> &'static str {
        ""
    }

    fn approve(&mut self) -> &'static str {
        ""
    }

    fn reject(&mut self) -> &'static str {
        ""
    }

    fn content<'a>(&self, _text: &'a str) -> &'a str {
        ""
    }
}

struct Draft;

impl Draft {
    fn new() -> Draft {
        Draft {} // NOTE: we can not use Draft(), but no {} is ok
    }
}

impl State for Draft {
    fn print_current_state(&self) {
        println!("Current State: Draft");
    }

    fn add_text<'a>(&mut self, text: &'a str) -> &'a str {
        text
    }

    fn request_review(&mut self) -> &'static str {
        println!("call request_review()!");
        "Pending"
    }
}

struct Pending {
    count: u8,
}

impl Pending {
    fn new() -> Pending {
        Pending { count: 0 }
    }
}

impl State for Pending {
    fn print_current_state(&self) {
        println!("Current State: Pending");
    }

    fn approve(&mut self) -> &'static str {
        println!("call approve()");
        self.count += 1;
        if self.count >= 2 {
            "Published"
        } else {
            ""
        }
    }

    fn reject(&mut self) -> &'static str {
        self.count = 0;
        "Draft"
    }
}

struct Published;

impl Published {
    fn new() -> Published {
        Published {}
    }
}

impl State for Published {
    fn print_current_state(&self) {
        println!("Current State: Published");
    }

    fn content<'a>(&self, text: &'a str) -> &'a str {
        text
    }
}

#[test]
fn test() {
    let mut post = Post::new();
    // post.add_text("hello world!");
    // fn is not first class, but closure is
    // let mut a = || post.request_review();
    // a();
    post.print_current_state();
    // TODO: call those functions in a loop
    post.approve();
    post.print_current_state();
    post.add_text("hello world!");
    post.print_current_state();
    post.request_review();
    post.print_current_state();
    post.approve();
    post.add_text("\thello rust!");
    post.print_current_state();
    post.reject();
    post.print_current_state();
    post.approve();
    post.print_current_state();
    post.request_review();
    post.print_current_state();
    post.approve();
    post.print_current_state();
    post.approve();
    post.print_current_state();
    println!("{:?}", post.content());
}
