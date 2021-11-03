use crate::dbgt;
use std::cell::RefCell;
use std::fmt;
use std::rc::{Rc, Weak};

// memory leak, cycle reference, create memory that will never cleaned up

#[derive(Debug)]
enum List<T: fmt::Display> {
    Cons(T, RefCell<Rc<List<T>>>),
    Nil,
}

impl<T> List<T>
where
    T: fmt::Display,
{
    pub fn new() -> List<T> {
        Self::Nil
    }

    pub fn prepend(self, item: T) -> List<T> {
        Self::Cons(item, RefCell::new(Rc::new(self)))
    }
}

#[test]
fn make_cycle() {
    let a = Rc::new(List::new().prepend(1));
    let b = Rc::new(List::Cons(2, RefCell::new(Rc::clone(&a))));
    if let List::Cons(_, tail) = &*a {
        *tail.borrow_mut() = Rc::clone(&b);
    }
    assert_eq!(2, Rc::strong_count(&a));
    assert_eq!(2, Rc::strong_count(&b));
    drop(b);
    assert_eq!(2, Rc::strong_count(&a));
}

// use Weak to prevent reference cycle
// A Weak pointer is useful for keeping a temporary reference
// Weak::new()
// maybe weak is useful for safe double linked list
