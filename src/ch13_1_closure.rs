use crate::dbgt;
use std::collections::HashMap;
use std::hash::Hash;
use std::thread;
use std::time::Duration;

struct Cacher<T, K, V>
where
    T: Fn(K) -> V,
{
    calculation: T,
    value: HashMap<K, V>,
}

impl<T, K, V> Cacher<T, K, V>
where
    T: Fn(K) -> V,
    K: Hash + Eq + Clone,
    V: Clone,
{
    pub fn new(calculation: T) -> Cacher<T, K, V> {
        Cacher {
            calculation,
            value: HashMap::new(),
        }
    }

    pub fn value(&mut self, num: K) -> V {
        match self.value.get(&num) {
            Some(v) => (*v).clone(),
            None => {
                let v = (self.calculation)(num.clone());
                self.value.insert(num, v.clone());
                v
            }
        }
    }
}

#[test]
fn test() {
    let some_closure = |num| {
        println!("calculaing ...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    let mut cache = Cacher::new(some_closure);

    println!("{}", cache.value(String::from("hello")));
    println!("{}", cache.value(String::from("hello")));
    println!("{}", cache.value(String::from("rust")));
}

// when create a closure, rust will infer which trait to use:
// FnOnce: All closure implemented this
// FnMut: no move closure implemented this
// Fn: others
// NOTE: move closures may still implement Fn or FnMut
// what the closure does with captured values, not how it captures them
// TODO: how to known the closure type, dbgt! did not work

fn call_closure<T, U>(f: &impl Fn(T) -> U, arg: T) -> U {
    f(arg)
}

fn call_closure_mut<T, U>(f: &mut impl FnMut(T) -> U, arg: T) -> U {
    f(arg)
}

fn call_closure_once<F, T, U>(f: F, arg: T) -> U
where
    F: FnOnce(T) -> U, // since once, no reference
{
    f(arg)
}

#[test]
fn test_traits() {
    let mut a: Vec<i32> = vec![];
    let b: Vec<i32> = vec![1, 2, 3, 4];
    let c = 10;
    let mut d = 10;

    let mut fa = |x| {
        a.push(x);
        format!("{:?}", a)
    };
    let fb = |x| x == b.into_iter().sum(); // no move, but consume
    let mut fc = move |x| x == c; // move, but have Fn
    let mut fd = |x| {
        d += x;
        d.to_string()
    };

    assert_eq!("[1]", call_closure_mut(&mut fa, 1));
    assert_eq!("[1, 10]", call_closure_mut(&mut fa, 10));

    assert!(!call_closure_once(fb, 1));
    // assert!(call_closure_once(fb, 10));

    assert!(!call_closure(&fc, 1));
    assert!(call_closure(&fc, 10));
    assert!(!call_closure_mut(&mut fc, 100));

    assert_eq!("11", call_closure_mut(&mut fd, 1));
    assert_eq!("21", call_closure_mut(&mut fd, 10));
}

// NOTE: Each closure instance has its own unique anonymous type: that is, even if two closures have the same signature, their types are still considered different
