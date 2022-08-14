use std::fs::{write, File};
use std::io::{Error, ErrorKind};

fn main() {
    let random_data = "Illum occaecati id sint explicabo. Ut officia sunt beatae esse. Eos quia animi ab quo reiciendis.";
    let greeting_file_result: Result<File, Error> = File::open("hello.txt");
    let _greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => {
                    let x = write("hello.txt", random_data);
                    match x {
                        Ok(_) => {}
                        Err(e) => {
                            panic!("Error Writing to file, {}", e)
                        }
                    }
                    fc
                }
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            _ => {
                panic!("Other Error");
            }
        },
    };
}
