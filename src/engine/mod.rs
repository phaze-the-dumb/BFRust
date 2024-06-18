pub struct Engine{
  bytes: Vec<u8>,
  pub mem: [ u32; 20 ],
  pointer: usize,
  index: usize
}

impl Engine{
  pub fn new( code: String ) -> Self{
    Engine {
      bytes: code.as_bytes().to_vec(),
      mem: [ 0; 20 ],
      pointer: 0,
      index: 0
    }
  }

  pub fn run(&mut self){
    loop{
      // Loop through all the lines until you run out.
      if self.index == self.bytes.len(){ break; }

      self.run_index();
    }
  }

  fn run_loop(&mut self, loop_start: usize){
    loop{
      // "]" - If the current memory cell is equal to 0
      if self.bytes[self.index] == 93 {
        if self.mem[self.pointer] == 0{
          // then drop back to the parent loop
          self.index += 1;
          break;
        } else{
          // else go back to the start of this loop
          self.index = loop_start;
        }
      }

      self.run_index();
    }
  }

  fn run_index(&mut self){
    let byte = self.bytes[self.index];

    // Increment the character index
    self.index += 1;

    // Check which character is being processed
    match byte{
      43 => {
        // "+" - Add one to memory cell at the location of the pointer
        self.mem[self.pointer] += 1;
      }
      45 => {
        // "-" - Subtract one to memory cell at the location of the pointer
        self.mem[self.pointer] -= 1;
      }
      62 => {
        // ">" - Move the pointer up one
        self.pointer += 1;
      }
      60 => {
        // "<" - Move the pointer down one
        self.pointer -= 1;
      }
      46 => {
        // "." - Convert the currently selected memory cell into ascii and print it
        print!("{}", char::from_u32(self.mem[self.pointer]).unwrap());
      }
      91 => {
        // "[" - Pause this loop and process the sub-loop
        self.run_loop(self.index.clone());
      }
      93 => {
        // "]" - Panic because syntax is incorrect
        panic!("Char {}, Unexpected \"]\"", self.index);
      }
      _ => {}
    }
  }
}