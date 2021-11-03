use crate::dbgt;
use std::fmt; // NOTE: the prelude Debug is a derive macro

trait NewTrait: fmt::Debug + fmt::Display {}
impl<T: fmt::Debug + fmt::Display> NewTrait for T {}

#[derive(Debug)]
struct TestTraitObject {
    data: Vec<Box<dyn NewTrait>>,
    // data: Vec<Box<dyn fmt::Debug + fmt::Display>>,
    // only auto traits can be used as additional traits in a trait object
    // uncomment it to see more information
}

impl TestTraitObject {
    fn test(&self) {
        dbgt!(&self);
        dbgt!(&self.data);

        for item in self.data.iter() {
            dbgt!(&item);
            println!("Debug: {:?}", item);
            println!("Display: {}", item);
        }
    }
}

#[test]
fn test() {
    let t = TestTraitObject {
        data: vec![Box::new(1), Box::new(3.14), Box::new(String::from("hello"))],
    };
    t.test();
}

// You can only make object-safe traits into trait objects.
// A trait is object safe if all the methods defined in the trait have the following properties:
//     The return type isn’t Self.
//     There are no generic type parameters.
// Clone trait
// https://doc.rust-lang.org/book/ch17-02-trait-objects.html
