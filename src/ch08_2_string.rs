use crate::dbgt;

#[test]
fn test() {
    let s1 = "hello";
    let mut s2 = String::from("hello");

    // convert
    assert_eq!(s1, s2.as_str());
    assert_eq!(s1.to_owned(), s2);
    assert_eq!(s1.to_string(), s2);
    assert_eq!(String::from(s1), s2);
    dbgt!(&&s2[..]); // &str
    dbgt!(&&s2); // &String

    // update
    s2.push_str(" world"); // &str
    assert_eq!("hello world", s2);
    s2.push('!'); // char
    assert_eq!("hello world!", s2);

    // concatenate
    s2 = s2 + " " + s1 + " rust!"; // move, String + &str or deref to &str
                                   // s2 += is not allowed
    assert_eq!("hello world! hello rust!", s2);

    assert_eq!(format!("{:->20}", ""), "--------------------");

    // String don't support indexing
    // UTF-8 String O(1) indexing is impossible

    // iterator
    assert_eq!("world", s2.chars().skip(6).take(5).collect::<String>());
    // s2.bytes()

    assert!("5" > "42"); // compare
    assert_eq!(24, s2.len()); // length
    assert_eq!(
        vec!["hello world", "hello rust!"],
        s2.split("! ").collect::<Vec<_>>()
    ); // split
    assert_eq!("halo world! halo rust!", s2.replace("hello", "halo")); // replace
}
