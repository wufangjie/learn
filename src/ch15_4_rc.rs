use std::fmt;
use std::rc::Rc;

#[derive(Debug)]
pub enum List<T>
where
    T: fmt::Display + fmt::Debug,
{
    Cons(T, Rc<List<T>>), // NOTE: Rc
    Nil,
}

impl<T> List<T>
where
    T: fmt::Display + fmt::Debug,
{
    pub fn new() -> List<T> {
        Self::Nil
    }

    pub fn prepend(self, item: T) -> List<T> {
        Self::Cons(item, Rc::new(self))
    }

    pub fn stringify(&self) -> String {
        match self {
            Self::Cons(head, tail) => format!("({}, {})", head, tail.stringify()),
            Self::Nil => format!("()"),
        }
    }
}

#[test]
fn test_basic() {
    let a = Rc::new(List::new().prepend(1).prepend(2).prepend(3));
    let b = List::Cons(4, Rc::clone(&a));
    let c = List::Cons(5, Rc::clone(&a));
    assert_eq!("(3, (2, (1, ())))", a.stringify());
    assert_eq!("(4, (3, (2, (1, ()))))", b.stringify());
    assert_eq!("(5, (3, (2, (1, ()))))", c.stringify());
    assert_eq!(3, Rc::strong_count(&a));
}

// NOTE: Rc<T> only make immutable references
