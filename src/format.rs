use crate::dbgt;
use std::collections::HashMap;
use std::fmt;
use std::fmt::Display;

// like python's str.format(), except ($, type specifier):
// println!("{number:>width$}", number=1, width=6);

// https://doc.rust-lang.org/rust-by-example/hello/print.html
// format!
// print!
// println!
// eprint!
// eprintln!

// write!
// dbg!

// https://doc.rust-lang.org/std/fmt/
// [[fill]align][sign]['#']['0'][width]['.' precision]type
// NOTE: `$` only for width and precision

#[test]
#[ignore]
fn test_basic() {
    println!(
        "{number:0>width$.precision$}",
        number = std::f64::consts::PI,
        width = 8,
        precision = 4
    ); // keyword arguments
    println!("{:?}", ());
    println!("{0:X}, {0:x}, {0:o}, {0:b}", 18);
    println!("{0:.2E}, {0:.2e}", 314159f64);
    println!("{:p}", &18);
    println!("{:p}", &19);
    println!("{:p}", &17);
    println!("{:p}", &18);
    println!("{:->20}", "");
}

// #[derive(Debug)] // will print struct's name
pub struct TupleStruct(f64, f64);

impl Display for TupleStruct {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Display: {:.1} + {:.1}i", self.0, self.1)
    }
}

impl fmt::Debug for TupleStruct {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Debug: Complex {{ real: {:.1}, imag: {:.1} }}",
            self.0, self.1
        )
    }
}

#[derive(Debug)]
pub struct Map<'a, T, U>(HashMap<&'a T, &'a U>)
// seems silly
where
    T: Display + ?Sized,
    U: Display + ?Sized;

impl<'a, T, U> Display for Map<'a, T, U>
where
    T: Display + ?Sized,
    U: Display + ?Sized,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{ ")?; // note this ?
        let mut is_first_time = true;
        for (&k, &v) in self.0.iter() {
            if is_first_time {
                is_first_time = false;
            } else {
                write!(f, ", ")?;
            }
            write!(f, "{}: {}", k, v)?;
        }
        write!(f, " }}")
    }
}

#[derive(Debug)]
pub struct Map2<T: Display, U: Display>(HashMap<T, U>);

impl<T, U> Display for Map2<T, U>
where
    T: Display,
    U: Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{ ")?; // note this ?
        let mut is_first_time = true;
        for (k, v) in self.0.iter() {
            if is_first_time {
                is_first_time = false;
            } else {
                write!(f, ", ")?;
            }
            write!(f, "{}: {}", k, v)?;
        }
        write!(f, " }}")
    }
}

#[test]
fn test() {
    let x = TupleStruct(3.33, 7.25);
    println!("{}", x);
    println!("{:?}", x);
    println!("{:#?}", x);

    let keys = vec!["hello", "world", "hash", "map"];
    let values: Vec<_> = vec![1, 2, 3, 4]
        .into_iter()
        .map(|x| x.to_string())
        .collect();
    let mut dct = HashMap::new();
    for i in 0..4 {
        dct.insert(keys[i], &values[i]);
    }

    dbgt!(&dct);
    // let mapt = Map(dct);
    let mapt = Map2(dct);
    dbgt!(&mapt);
    println!("{:?}", mapt);
    println!("{}", mapt);
    println!("{}", mapt);
}
