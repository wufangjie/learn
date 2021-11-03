use crate::dbgt;
use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;

pub trait SendMessage {
    fn send(&self, msg: String); // we can use &'static str
}

pub struct LimitChecker<'a, T: SendMessage> {
    agent: &'a T,
    value: usize,
    limit: usize,
}

impl<'a, T> LimitChecker<'a, T>
where
    T: SendMessage,
{
    pub fn new(agent: &'a T, limit: usize) -> LimitChecker<'a, T> {
        LimitChecker {
            agent,
            value: 0,
            limit,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;
        let p = value as f64 / self.limit as f64;
        if p > 1.0 {
            self.agent.send(format!("{:.4} > 1", p));
        } else if p > 0.9 {
            self.agent.send(format!("{:.4} > 0.9", p));
        } else if p > 0.75 {
            self.agent.send(format!("{:.4} > 0.75", p));
        }
    }
}

//#[derive(Debug)]
struct Messenger {
    message_store: RefCell<Vec<String>>,
    //message_store: Cell<Vec<String>>,
}

impl Messenger {
    pub fn new() -> Messenger {
        Messenger {
	    message_store: RefCell::new(Vec::new()),
            //message_store: Cell::new(Vec::new()),
        }
    }
}

impl SendMessage for Messenger {
    fn send(&self, msg: String) {
        self.message_store.borrow_mut().push(msg);
	//self.message_store.get_mut().push(msg);
    }
}

#[test]
fn test_mock() {
    let mock = Messenger::new(); // NOTE: immutable
    let mut limit_checker = LimitChecker::new(&mock, 100);
    for v in [60, 85, 95, 195] {
        limit_checker.set_value(v);
    }
    assert_eq!(
        mock.message_store,
        RefCell::new(vec![
            String::from("0.8500 > 0.75"),
            String::from("0.9500 > 0.9"),
            String::from("1.9500 > 1")
        ])
    );
}

#[test]
fn test_basic() {
    let a = RefCell::new(5);
    dbgt!(&a.borrow());
    dbgt!(&a.borrow_mut());
    *a.borrow_mut() += 1;
    dbgt!(&a.borrow());

    // RefCell enforces borrowing rules at *runtime*
    let x = Rc::new(a);
    let y = Rc::clone(&x);
    {
        let mut z = y.borrow_mut(); // sugar(deref?) for (*y).borrow_mut()
        assert_eq!(2, Rc::strong_count(&x));
        *z += 1;
        println!("x = {:?}, y = {:?}", x, y); // RefCell { value: <borrowed> }
    }
    println!("x = {:?}, y = {:?}", x, y); // RefCell { value: 7 }
}

#[derive(Debug)]
enum List<T> {
    Cons(RefCell<T>, Rc<List<T>>),
    Nil,
}

impl<T> List<T>
where
    T: fmt::Display + fmt::Debug,
{
    pub fn stringify(&self) -> String {
        match self {
            Self::Cons(head, tail) => format!("({}, {})", head.borrow(), tail.stringify()),
            Self::Nil => format!("()"),
        }
    }
}

#[test]
fn test_rm_rc() {
    let a = Rc::new(List::Cons(RefCell::new(5), Rc::new(List::Nil)));
    let b = List::Cons(RefCell::new(4), Rc::clone(&a));
    let c = List::Cons(RefCell::new(3), Rc::clone(&a));

    if let List::Cons(v, _) = &*a {
        // NOTE: a is a smart pointer, while &*a is a regular reference
        *v.borrow_mut() *= 2;
    }

    assert_eq!("(10, ())", a.stringify());
    assert_eq!("(4, (10, ()))", b.stringify());
    assert_eq!("(3, (10, ()))", c.stringify());

    assert_eq!(&1, &1);
}

// NOTE: I think it is possible to check refcell's borrowing rules at compile time, but it is too heavy for rust?
// https://users.rust-lang.org/t/refcell-why-borrowing-is-checked-only-in-runtime/52721/20
