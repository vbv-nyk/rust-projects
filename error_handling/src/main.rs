use std::{
    error::{self, Error},
    fs::File,
    io::{self, Read},
};
fn main() -> Result<(), Box<dyn Error>> {
    let mut content = String::new();
    File::open("hello.txt")?.read_to_string(&mut content)?;
    Ok(())
}
