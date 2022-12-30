use std::env; use std::fs;

struct Scanner {
  source: String
}

impl Scanner {
  fn scan(&self) -> Vec<&str> {
    self.source.split(" ").collect()
  }
}

fn run(source: String) {
  let scanner = Scanner { source };
  let tokens = scanner.scan();

  for token in tokens {
    println!("{}", token);
  }
}

fn error(line: u32, message: String) {
  println!("[line {}] Error: {}", line, message);
  std::process::exit(1);
}
 
fn main() {
  let _args: Vec<String> = env::args().collect();
  if _args.len() < 2 { 
    println!("invalid source path");
    std::process::exit(1); 
  }

  let contents = fs::read_to_string(&_args[1]).unwrap();

  run(contents);
}
