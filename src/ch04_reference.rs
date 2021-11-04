use crate::dbgt;

#[test]
#[ignore]
fn test_ref_in_iter() {
    // let lst = vec![String::from("hello")];
    let lst = vec!["hello"];

    for &item in lst.iter() {
        // sugar for (&lst).into_iter()
        dbgt!(&item);
    }

    for item in lst.iter() {
        dbgt!(&item);
    }

    for item in (&&&lst).iter() {
        // seems silly, deref coercion?
        dbgt!(&item);
    }

    for item in (&lst).into_iter() {
        dbgt!(&item);
    }

    dbg!(&lst);

    for item in lst.into_iter() {
        // moved out
        dbgt!(&item);
    }
}

struct RGB(String, String, String);

// TODO: it seems reference's field and indexing are not reference
// but (&Iter).into_iter() will get &item
// https://rust-lang.github.io/rfcs/2005-match-ergonomics.html
// ref mut is a mutable reference
#[test]
fn test() {
    let a = (String::from("1"), String::from("2"), String::from("3"));
    let b = &a;
    let x = [String::from("1"), String::from("2"), String::from("3")];
    let y = &x;

    let m = RGB(String::from("1"), String::from("2"), String::from("3"));
    let n = &m;
    match n {
        RGB(r, _g, _b) => {
            dbgt!(&r);
        }
    }

    match n {
        &RGB(ref r, ref _g, ref _b) => {
            dbgt!(&r);
        } // desugar, without ref n will move
    }

    // reference access field, String
    dbgt!(&b.1);
    dbgt!(&y[1]);
    dbgt!(&m.1);

    let mut z = &5;
    z = &&6; // deref?
    dbgt!(&z);

    dbgt!(&(&1 + &2));
}

// NOTE: make reference on match
// https://doc.rust-lang.org/stable/rust-by-example/flow_control/match/destructuring/destructure_pointers.html
