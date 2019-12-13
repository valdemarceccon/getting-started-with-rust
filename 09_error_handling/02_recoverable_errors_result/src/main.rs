use std::fs::File;
use std::io::{ErrorKind, Write};

fn main() {
    treating_result();


}

fn treating_result() {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("PROBLEM {:?}", e),
            },
            other_error => panic!("PROBLEM: {:?}", other_error),
        },
    };
}


