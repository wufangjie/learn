use std::rc::Rc;

struct TestDrop {
    data: String,
}

impl Drop for TestDrop {
    fn drop(&mut self) {
        println!("Calling drop: TestDrop {{ data: \"{}\" }}", self.data);
    }
}

fn take_ownership<T>(_: T) {
    // maybe it is what `std::mem::forget` do
    println!("Do nothing, just take ownership");
}

// NOTE:
// Variables are dropped in the reverse order of their creation
// Rust doesn’t let you call the Drop trait’s drop method manually (double free)

#[test]
fn test() {
    {
        println!("{:-^20}", " dropping ");
        let a = TestDrop {
            data: String::from("hello"),
        };
        let _b = TestDrop {
            data: String::from("world"),
        };
        let _c = TestDrop {
            data: String::from("drop"),
        };
        take_ownership(a);
        println!("{:->20}", "");
    }

    {
        println!("{:-^20}", " dropping rc ");
        let a = Rc::new(TestDrop {
            data: String::from("rc"),
        });
        let b = Rc::clone(&a);
        let _c = Rc::clone(&a);
        take_ownership(a);
        println!("Does drop() just take ownership?");
        drop(b);
        println!("{:->20}", "");
    }
}
