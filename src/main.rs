mod engine;
mod compiler;

use std::{ env, fs };
use regex::Regex;

use engine::Engine;
use compiler::Compiler;

fn main() {
  let file_name = env::args().nth(1);

  if file_name.is_none() {
    println!("Usage: bfrust <path>");
  }

  let content = fs::read_to_string(file_name.unwrap()).expect("Cannot read file");

  // filter out characters that aren't <>-+,.[]

  let re = Regex::new(r"(?m)[^<>+,.\-\[\]]").unwrap();
  let content = re.replace_all(&content, "").to_string();

  let is_compiled = env::args().find(|x| x == "-c").is_some();

  if is_compiled{
    // Compile the code
    let mut compiler = Compiler::new(content);
  } else{
    // Run the code interpreter
    let mut engine = Engine::new(content);
    engine.run();
  }
}