use nom::*;
use std::str;

// Parsers
named!(procedure<&[u8], Procedure>,
    do_parse!(
        tag!("(")   >>
        op_type: op >>
        arguments: many1!(string) >>
        tag!(")")   >>
        (Procedure { op: op_type, args: arguments  })
    )
);

named!(op<&[u8], Op>,
    alt!(
        specialform |
        primitive   |
        user
    )
);

named!(specialform<&[u8], Op>,
    alt!(
        map!(tag!("if"),     |_| Op::SpecialForm(SForm::If))     |
        map!(tag!("begin"),  |_| Op::SpecialForm(SForm::Begin))  |
        map!(tag!("define"), |_| Op::SpecialForm(SForm::Define)) |
        map!(tag!("lambda"), |_| Op::SpecialForm(SForm::Lambda)) |
        map!(tag!("let"),    |_| Op::SpecialForm(SForm::Let))
    )
);

named!(primitive<&[u8], Op>,
    alt!(
        map!(tag!("*"), |_| Op::Primitive(Prim::Mul)) |
        map!(tag!("+"), |_| Op::Primitive(Prim::Add)) |
        map!(tag!("-"), |_| Op::Primitive(Prim::Sub)) |
        map!(tag!("/"), |_| Op::Primitive(Prim::Div))
    )
);

named!(user<&[u8], Op>,
    do_parse!(
        user_proc: string >>
        (Op::User(user_proc))
    )
);

named!(string<&[u8], String>,
    do_parse!(
        word: ws!(alphanumeric) >>
        (String::from_utf8(word.to_vec()).unwrap())
    )
);

#[derive(Debug)]
enum Op {
    Primitive(Prim),
    SpecialForm(SForm),
    User(String),
}

#[derive(Debug)]
struct Procedure {
    op: Op,
    args: Vec<String>
}

#[derive(Debug)]
enum Prim {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug)]
enum SForm {
    If,
    Begin,
    Define,
    Lambda,
    Let,
}

#[test]
fn parens_test() {
    let parsed = procedure(b"(+ hi hello what is up)");
    println!("{:#?}", parsed);
}
