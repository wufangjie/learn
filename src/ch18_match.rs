use crate::dbgt;

struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
enum Message {
    Hello { id: i32 },
}

// https://doc.rust-lang.org/book/ch18-03-pattern-syntax.html
// https://doc.rust-lang.org/rust-by-example/flow_control/match.html
// https://rust-lang.github.io/rfcs/2005-match-ergonomics.html

#[test]
fn test() {
    let Point { y, x } = Point {
        // NOTE: the order is not important
        x: 3,
        y: 4,
    };
    assert_eq!(x, 3);
    assert_eq!(y, 4);

    let x = 4;
    let y = false;

    let res = match x {
        4 | 5 | 6 if y => format!("yes"),
        // 4 | 5 | (6 if y) => format!("yes"), // syntax error
        _ => format!("no"),
    };
    assert_eq!("no", res);

    // @ bindings
    let res = match (Message::Hello { id: 5 }) {
        // NOTE: () is necessary
        // NOTE: without @, we cannot get `id` field in a range
        Message::Hello {
            id: id_variable @ 3..=7,
        } => format!("id = {:?} in [3, 7]", id_variable),
        Message::Hello { id: 10..=12 } => format!("id in [10, 12]"),
        Message::Hello { id } => format!("id = {}", id),
    };
    assert_eq!("id = 5 in [3, 7]", res);
}
