use std::fs::File;
use std::io;
use std::io::{Read, ErrorKind};


fn unwrap_or_else() {
    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
}

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s), // amazing!!!
        Err(e) => Err(e),
    }
}

fn read_username_from_file_sugar() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

// unwrap: unwrap will call the panic! macro for us
// expect: we can choose the panic! error message
// unwarp_or_else: use closure to deal err
// ? operator

// panic! marco,
// example of panic error: index out of bound

// Read::by_ref(&mut f): how to specify method when there are multiple implements
