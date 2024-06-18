use std::{fs, io::Write, process::Command};

pub struct Compiler{
  bytes: Vec<u8>,
  file: fs::File,
  index: usize
}

impl Compiler{
  pub fn new( code: String ) -> Self{
    Compiler{
      bytes: code.as_bytes().to_vec(),
      file: fs::File::create("out.rs").expect("Cannot open output file"),
      index: 0
    }
  }

  pub fn run(&mut self){
    self.file.write(b"fn main(){let mut mem = [ 0; 100 ];let mut p = 0;").unwrap();

    loop{
      // Loop through all the lines until you run out.
      if self.index == self.bytes.len(){ break; }

      self.run_index();
    }

    self.file.write(b"}").unwrap();

    let mut cmd = Command::new("rustc");
    cmd.arg("out.rs");
    cmd.spawn().unwrap().wait().unwrap();

    fs::remove_file("out.rs").unwrap();
  }

  pub fn run_index(&mut self){
    let byte = self.bytes[self.index];

    // Increment the character index
    self.index += 1;

    // Check which character is being processed
    match byte{
      43 => {
        // "+" - Add one to memory cell at the location of the pointer
        self.file.write(b"mem[p] += 1;").unwrap();
      }
      45 => {
        // "-" - Subtract one to memory cell at the location of the pointer
        self.file.write(b"mem[p] -= 1;").unwrap();
      }
      62 => {
        // ">" - Move the pointer up one
        self.file.write(b"p += 1;").unwrap();
      }
      60 => {
        // "<" - Move the pointer down one
        self.file.write(b"p -= 1;").unwrap();
      }
      46 => {
        // "." - Convert the currently selected memory cell into ascii and print it
        self.file.write(b"print!(\"{}\", char::from_u32(mem[p]).unwrap());").unwrap();
      }
      44 => {
        // "," - Get 1 byte of input from the user
        self.file.write(b"let mut input_text = String::new();std::io::stdin().read_line(&mut input_text).expect(\"failed to read from stdin\");let input_text = input_text.replace(\"\r\n\", \"\");let input_bytes = input_text.as_bytes();let mut i = 0;for inpt in input_bytes{if i > 0{ break; }mem[p] = inpt.clone() as u32;i += 1;}").unwrap();
      }
      91 => {
        // "[" - Open loop
        self.file.write(b"loop{").unwrap();
      }
      93 => {
        // "]" - Panic because syntax is incorrect
        self.file.write(b"if mem[p] == 0 { break; }").unwrap();
        self.file.write(b"}").unwrap();
      }
      _ => {}
    }
  }
}