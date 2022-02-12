use std::fmt::{self, Display};
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

trait PrintOutline: Display {
    // supertrait + blanket implementation

    fn print_outline(&self) {
        let output = self.to_string();
        let n = output.len();

        println!("{:*>width$}", "", width = n + 4);
        println!("*{: >width$}*", "", width = n + 2);
        println!("* {} *", output);
        println!("*{: >width$}*", "", width = n + 2);
        println!("{:*>width$}", "", width = n + 4);
    }
}

impl<T: Display> PrintOutline for T {
    // overwrite
    fn print_outline(&self) {
        let output = self.to_string();
        let n = output.len();

        println!("{:#>width$}", "", width = n + 4);
        println!("#{: >width$}#", "", width = n + 2);
        println!("# {} #", output);
        println!("#{: >width$}#", "", width = n + 2);
        println!("{:#>width$}", "", width = n + 4);
    }
}

#[test]
fn test_19_2_supertrait() {
    String::from("Hello, World").print_outline();
}

struct Wrapper(Vec<String>);

impl Deref for Wrapper {
    // NOTE: now, we can call Vec<String>'s method by wrapper object
    type Target = Vec<String>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Display for Wrapper {
    // using newtype pattern to implement Display trait for Vec<String>
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[ ")?;
        write!(f, "{}", self.0.join(", "))?;
        write!(f, " ]")
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
    assert_eq!(format!("[ {} ]", w.join(", ")), w.to_string());
}
