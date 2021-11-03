use std::cell::Cell;

// Cell<T> enable compile-time interior mutability
// NOTE: Cell's interior need implement Copy trait for get(), #[derive(Debug)]

// #[derive(Debug)]
pub struct Immutable {
    a: Cell<i32>,
    b: Cell<String>,
    c: Cell<Vec<i32>>,
}

impl Immutable {
    pub fn new() -> Self {
        Immutable {
            a: Cell::new(0),
            b: Cell::new(String::from("hello")),
            c: Cell::new(vec![]),
        }
    }
}

#[test]
fn test() {
    let obj = Immutable::new();
    assert_eq!(0, obj.a.get());
    obj.a.set(5);
    assert_eq!(5, obj.a.get()); // get_mut(&mut self)

    let mut ct = obj.c.take();
    ct.push(1);
    ct.push(2);
    obj.c.set(ct);
    ct = obj.c.into_inner(); // the same as take()?
    assert_eq!(vec![1, 2], ct);

    assert_eq!(String::from("hello"), obj.b.replace(String::from("world")));
    let c2 = Cell::new(String::from("rust"));
    obj.b.swap(&c2);
    assert_eq!(String::from("world"), c2.take());
}
