use std::env; use std::fs; use std::io;

mod scanner;

fn run(source: String) {
  let scanner = scanner::Scanner::new(source);
  scanner.scan_tokens();
}

fn run_from_source(path: String) {
  run(fs::read_to_string(path).unwrap())
}

fn run_repl() {
  loop {
    println!("miau_script > ");    

    let mut input =  String::new();
    io::stdin().read_line(&mut input).unwrap();

    run(input.to_string());

    println!("\n");
  }
}

fn main() {
  let _args: Vec<String> = env::args().collect();
  if _args.len() < 2 { 
    println!("executing REPL:");
    run_repl();
  } else {
    println!("executing from source:");
    run_from_source(_args[1].to_string());
  }

}
