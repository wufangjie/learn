use crate::dbgt;

#[derive(Debug)]
struct Foo(i32);

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(&'static str), // object: enum, struct..
    ChangeColor(i32, i32, i32),
}

#[test]
#[ignore]
fn test() {
    let lst: Vec<Foo> = vec![1, 2, 3].into_iter().map(Foo).collect();
    dbgt!(&lst);

    let lst: Vec<Message> = vec!["hello", "rust"]
        .into_iter()
        .map(Message::Write)
        .collect();
    dbgt!(&lst);
}

// funtion pointer
// closure traits (Fn, FnMut, FnOnce)
// how return a closure

fn returns_closure_impl() -> impl Fn(i32) -> i32 {
    // NOTE: the impl keyword is sugar for:
    // returns_closure_impl<T: Fn(i32) -> i32>() -> T
    |x| x + 1
}

fn returns_closure_trait_object() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}

fn returns_closure_2(a: i32) -> Box<dyn Fn(i32) -> i32> {
    // NOTE: impl version can not compile
    // no two closures, even if identical, have the same type
    if a > 0 {
        Box::new(move |x| x + a)
    } else {
        Box::new(move |x| x - a)
    }
}

#[test]
fn test_closure() {
    let c1 = returns_closure_impl();
    assert_eq!(6, c1(5));
    let c2 = returns_closure_trait_object();
    assert_eq!(6, c2(5));
    let c3 = returns_closure_2(2);
    assert_eq!(7, c3(5));
    let c4 = returns_closure_2(-2);
    assert_eq!(7, c4(5));
}
