use std::fs::File;
use std::io::ErrorKind;
fn main() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("error create file{}", e),
            },
            oe => panic!("error openning the file{:?}", oe),
        },
    };
}
