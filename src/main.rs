extern crate rustyline;
use std::error;
use std::fmt;

/// Our main driver loop for our REPL
fn main() {
    let mut reader = rustyline::Editor::<()>::new();
    loop {
        match reader.readline(">> ") {
            Ok(line) => {
                let parsed = parse(&line.trim());
                match parsed {
                    Ok(result) => {
                        if result == "(exit)" {
                            break;
                        } else {
                            println!("{}",result);
                        }
                    },
                    Err(e) => println!("Couldn't evaluate expression. Error: {}", e),
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

//-----------------------------------------------------------------------------//
//                                                                             //
//                                  Parsing                                    //
//                                                                             //
//-----------------------------------------------------------------------------//

/// Parse the input and make sure it's valid Scheme.
fn parse(line: &str) -> Result<String, ParseError> {
    match syntax_check(line) {
        // We have it like this so that work can be done
        // in the future to Parse if things are alright
        Ok(x) => Ok(x),
        Err(e) => Err(e),
    }
}

//-----------------------------------------------------------------------------//
//                                                                             //
//                             Syntax Check                                    //
//                                                                             //
//-----------------------------------------------------------------------------//

/// Type alias that's more descriptive then a true or false
type SyntaxOkay = bool;

/// Check that the syntax is correct for the whole Scheme expression
fn syntax_check(line: &str) -> Result<String, ParseError> {
    if check_paren(line) {
        Ok(String::from(line))
    } else {
        Err(ParseError::MisMatchedParen)
    }
}

/// Checks to see if all of the parentheses have a matching one
fn check_paren(line: &str) -> SyntaxOkay {
    let mut count = 0;

    for i in line.chars() {

        // If we have less than 0 we had an unmatched )
        // Later on if count is greater than 0 after the loop
        // we had an unmatched (
        if count < 0 {
            break;
        }

        if i == '(' {
            count += 1;
        } else if i == ')' {
            count -= 1;
        }
    }

    count == 0
}



//-----------------------------------------------------------------------------//
//                                                                             //
//                                   Errors                                    //
//                                                                             //
//-----------------------------------------------------------------------------//
#[derive(Debug)]
enum ParseError {
    MisMatchedParen
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::ParseError::*;
        match *self {
            MisMatchedParen => write!(f, "Unmatched Parentheses"),
        }
    }
}

impl error::Error for ParseError {
    fn description(&self) -> &str {
        use self::ParseError::*;
        match *self {
            // We can make this positional later!
            MisMatchedParen => "Unbalanced number of parentheses"
        }
    }
}

//-----------------------------------------------------------------------------//
//                                                                             //
//                               Test Suite                                    //
//                                                                             //
//-----------------------------------------------------------------------------//
#[cfg(test)]
mod sytnax_tests {
    use super::check_paren;

    #[test]
    fn parentheses_checks() {
        assert_eq!(check_paren("(())"), true);
        assert_eq!(check_paren(")()"), false);
        assert_eq!(check_paren("(()"), false);
        assert_eq!(check_paren("(test)"), true);
        assert_eq!(check_paren("(test"), false);
    }

}
