use crate::dbgt;
use std::future::Future;


async fn foo() -> usize {
    println!("hello async!");
    0
}

#[test]
fn test() {
    let fut = foo();
    println!("lazy");
    futures::block_on();
}
