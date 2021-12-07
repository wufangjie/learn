use std::ops::Deref;

struct A;

impl A {
    fn associate_function_call() -> &'static str {
        "function A"
    }

    fn method_call(&self) -> &str {
        "method A"
    }
}

trait B {
    fn associate_function_call() -> &'static str {
        "function B"
    }

    fn method_call(&self) -> &str {
        "method B"
    }
}

impl B for A {}

#[test]
fn test_19_2_ambiguous() {
    let a = A {};
    assert_eq!("function A", A::associate_function_call());
    assert_eq!("method A", a.method_call());
    assert_eq!("function B", <A as B>::associate_function_call());
    assert_eq!("method B", B::method_call(&a));
}

struct Wrapper(Vec<String>);

impl Deref for Wrapper {
    type Target = Vec<String>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[test]
fn test_19_2_newtype() {
    let w = Wrapper(vec![
        "hello".to_owned(),
        "world".to_owned(),
        "newtype".to_owned(),
        "deref".to_owned(),
    ]);
    println!("{:?}", w.join(", "));
}
