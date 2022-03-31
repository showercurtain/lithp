use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use super::Error;

pub fn load_file(filename: &String) -> Result<String,Error> {
  // Create a path to the desired file
  let path = Path::new(filename);

  // Open the path in read-only mode, returns `io::Result<File>`
  let mut file = match File::open(&path) {
      Err(_) => {return Err(Error::FileNotFound(filename.clone()))},
      Ok(file) => file,
  };

  // Read the file contents into a string, returns `io::Result<usize>`
  let mut s = String::new();
  match file.read_to_string(&mut s) {
    Ok(_) => Ok(s),
    Err(_) => Err(Error::FileNotFound(filename.clone()))
  }
}