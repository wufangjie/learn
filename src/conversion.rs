use crate::dbgt;
// use std::convert::{From, Into}; // already in prelude

#[derive(Debug, PartialEq)]
struct A {
    value: i32,
}

impl From<i32> for A {
    fn from(value: i32) -> Self {
        A { value }
    }
}

#[test]
fn test_from() {
    let _b: A = 5.into(); // if we impl from(A) -> B, we can use b.into()
    assert_eq!(A::from(5), 5.into());
    dbgt!(&Into::<A>::into(5));
    // 5.into() must be used in the palce rust can infer T
}

impl Into<String> for A {
    fn into(self) -> String {
        format!("{}", self.value)
    }
}

#[test]
fn test_into() {
    // let a = A::from("42".to_owned()); // error
    // but we cannot use A::from(b), even if we impl into<B> for A
    let a = A { value: 42 };
    let sa: String = a.into();
    dbg!(sa);
}
