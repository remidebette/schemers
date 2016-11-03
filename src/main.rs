extern crate rustyline;
fn main() {
    let mut reader = rustyline::Editor::<()>::new();
    let mut done = false;
    while !done {
        let readline = reader.readline(">> ");
        match readline {
            Ok(line) =>
                if line == "(exit)" {
                    done = true;
                } else {
                    println!("{}",line)
                },
            Err(_) => panic!("Couldn't readline"),
        }
    }
}
