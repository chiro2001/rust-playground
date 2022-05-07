#[warn(unused_variables)]
use std::error::Error;
use std::{fs::File, io::Write};
fn main() -> Result<(), Box<dyn Error>> {
    let filename = String::from("test-file.tmp");
    let res = File::create(&filename)?.write("Hello, World!".as_bytes())?;
    print!("wrote {res} bytes to file {filename}");
    Ok(())
}
