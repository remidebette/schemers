extern crate rustyline;
fn main() {
    let mut done = false;
    let mut reader = rustyline::Editor::<()>::new();
    while !done {
        match reader.readline(">> ") {
            Ok(line) =>
                if line == "(exit)" {
                    done = true;
                } else {
                    println!("{}",line)
                },
            Err(e) => println!("Couldn't readline. Error was: {}", e),
        }
    }
}
