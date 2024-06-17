use std::fs;

pub struct Compiler{
  bytes: Vec<u8>,
  file: fs::File
}

impl Compiler{
  pub fn new( code: String ) -> Self{
    Compiler{
      bytes: code.as_bytes().to_vec(),
      file: fs::File::open("out.rs").expect("Cannot open output file")
    }
  }

  pub fn run(){

  }
}