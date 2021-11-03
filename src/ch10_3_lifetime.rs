#[test]
pub fn test_longest() {
    let a = "xyz";
    let r = "";
    {
        let b = String::from("abcd");
        let r = longest(a, b.as_str());
        assert_eq!("abcd", r);
    }
    assert_eq!("", r);
}

fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() >= s2.len() {
        s1
    } else {
        s2
    }
}

// NOTE: all parameters with lifetime 'a, does not means they have the same lifetime, but declare they live at least as long as lifetime 'a, which is the smallest

// Lifetime Elision
// The first rule is that each parameter that is a reference gets its own lifetime parameter.
// The second rule is if there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters.
// The third rule is if there are multiple input lifetime parameters, but one of them is &self or &mut self because this is a method, the lifetime of self is assigned to all output lifetime parameters.
