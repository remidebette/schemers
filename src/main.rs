extern crate rustyline;
#[macro_use]
extern crate nom;

mod parser;

/// Our main driver loop for our REPL
fn main() {
    let mut reader = rustyline::Editor::<()>::new();
    loop {
        match reader.readline(">> ") {
            Ok(line) => {
                if line == "(exit)" {
                    break;
                } else {
                    println!("{}", line);
                }
            },
            Err(e) => {
                use rustyline::error::ReadlineError::*;
                match e {
                    // Close the program on a Ctrl-C or Ctrl-D
                    Eof | Interrupted => break,
                    _ => println!("Couldn't readline. Error was: {}", e),
                }
            },
        }
    }
}
