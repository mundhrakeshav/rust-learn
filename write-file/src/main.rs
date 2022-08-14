use std::fs::{write, File};
use std::io::{Error, ErrorKind, Read};

fn main() {
    let random_data = "Illum occaecati id sint explicabo. Ut officia sunt beatae esse. Eos quia animi ab quo reiciendis.";
    let greeting_file_result: Result<File, Error> = File::open("hello.txt");
}

fn read_username_from_file() -> Result<String, Error> {
    let mut username_file: File = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}
