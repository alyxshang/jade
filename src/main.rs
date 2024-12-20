/*
Jade by Alyx Shang.
Licensed under the FSL v1.
*/

/// Importing this app's
/// tiny CLI function.
use jade::cli;

/// The main point of entry for the
/// Rust compiler.
fn main() {
    match cli(){
        Ok(feedback) => println!("{}", feedback),
        Err(e) => println!("{}", &e.to_string())
    }
}