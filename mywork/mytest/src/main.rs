
use std::fs::File;

fn main() {
    let res = File::open("foo");
    match res {
        Ok(f) => { println!("OK"); },
        Err(e) => { println!("ERR"); },
    }
}