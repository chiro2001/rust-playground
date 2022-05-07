#[warn(unused_variables)]
use std::error::Error;
use std::io;
use std::fs;
use std::{fs::File, io::Write};

fn get_file_content(filename: &String) -> Result<String, io::Error> {
	fs::read_to_string(filename)
}
fn main() -> Result<(), Box<dyn Error>> {
    let filename = String::from("test-file.tmp");
	let content = get_file_content(&filename);
	let content = match content {
		Ok(content) => content,
		Err(_) => String::from("[Empty]")
	};
    let res = File::create(&filename)?.write((content + "\nHello, World!").as_bytes())?;
    print!("wrote {res} bytes to file {filename}");
    Ok(())
}
