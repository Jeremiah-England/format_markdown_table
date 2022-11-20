mod table;

use std::env;
use std::fs::File;
use std::io;
use std::io::Read;
use std::io::Write;

fn get_raw() -> String {
    let args: Vec<String> = env::args().collect();

    let mut buffer = String::new();
    match args.len() {
        1 => {
            let mut stdin = io::stdin();
            match stdin.read_to_string(&mut buffer) {
                Ok(_) => {}
                Err(error) => panic!("Trouble reading from stdin: {error}"),
            }
        }
        2 => {
            let path = &args[1];
            let mut buf: Vec<u8> = Vec::new();
            match File::open(path) {
                Ok(mut file) => {
                    match file.read_to_end(&mut buf) {
                        Ok(_) => match String::from_utf8(buf) {
                            Ok(string) => buffer = string,
                            Err(error) => panic!("File is not in UTF8! {error}"),
                        },
                        Err(error) => {
                            panic!("Error reading file! {error}");
                        }
                    };
                }
                Err(error) => {
                    panic!("Trouble opening file at {path}: {error}")
                }
            }
        }
        _ => {
            panic!("Please submit one or zero arguments.")
        }
    }
    return buffer;
}

fn main() {
    let raw = get_raw();
    let table = table::Table::parse(&raw);
    let formatted = table.format();
    match io::stdout().write_all(formatted.as_bytes()) {
        Ok(_) => {}
        Err(error) => {
            println!("Bummer! {error}")
        }
    }
}
