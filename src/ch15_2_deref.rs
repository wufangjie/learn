use crate::dbgt;
use std::fmt;
use std::ops::Deref;
use std::rc::Rc;
// use std::cell::RefCell;

#[derive(Debug)]
struct SP<T> {
    data: T,
}

impl<T> SP<T> {
    fn new(data: T) -> SP<T> {
        SP { data }
    }
}

impl<T> Deref for SP<T> {
    type Target = T; // missing `Target` in implementation
                     // so called associated type in ch19

    fn deref(&self) -> &T {
        // &Self::Target
        // we can use *&T to find T and won't transfer ownership
        &self.data
    }
}

#[derive(Debug)]
struct SP2<T: fmt::Debug> {
    data: SP<T>,
}

impl<T> SP2<T>
where
    T: fmt::Debug,
{
    fn new(data: SP<T>) -> SP2<T> {
        SP2 { data }
    }

    fn coercion_method(&self) {
        // type of `self` must be `Self` or a type that dereferences to it
        dbgt!(&self);
    }
}

impl<T> Deref for SP2<T>
where
    T: fmt::Debug,
{
    type Target = SP<T>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

// behind the scenes Rust actually ran *y as *(y.deref())

#[test]
fn test() {
    let p = SP2::new(SP::new(5));
    dbgt!(&*p);
    dbgt!(&(&3).deref()); // regular reference implemented deref (return self)
    deref_coercion(&p);
    // & is needed, we must keep an & so that we can call deref(&self)
    Box::new(Rc::new(p)).coercion_method(); // Rust add & automatically
                                            // RefCell dit not implement Deref, we must use borrow or borrow_mut
}

fn deref_coercion(v: &i32) {
    dbgt!(v);
}
