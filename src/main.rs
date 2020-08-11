extern crate rustyline;
extern crate nom;

mod parser;

use rustyline::error::ReadlineError;

/// Our main driver loop for our REPL
fn main() {
    let mut reader = rustyline::Editor::<()>::new();
    loop {
        match reader.readline(">> ") {
            Ok(line) => {
                if line.trim() == "(exit)" {
                    break;
                } else {
                    println!("{}", line);
                }
            },
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            },
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            },
            Err(e) => println!("Couldn't readline. Error was: {}", e),
        }
    }
}
