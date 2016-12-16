use nom::*;
use std::str;

// Parsers
named!(procedure<&[u8], Procedure>,
    do_parse!(
        tag!("(")   >>
        op_type: op >>
        arguments: ws!(many0!(string)) >>
        tag!(")")   >>
        (Procedure { op: op_type, args: arguments  })
    )
);

named!(op<&[u8], Op>,
    alt!(
        ws!(specialform) |
        ws!(primitive)   |
        ws!(user)
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

#[derive(Debug, PartialEq, Eq)]
enum Op {
    Primitive(Prim),
    SpecialForm(SForm),
    User(String),
}

#[derive(Debug, PartialEq, Eq)]
struct Procedure {
    op: Op,
    args: Vec<String>
}

#[derive(Debug, PartialEq, Eq)]
enum Prim {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug, PartialEq, Eq)]
enum SForm {
    If,
    Begin,
    Define,
    Lambda,
    Let,
}

#[test]
fn string_parser() {
    match string(b"hi") {
        IResult::Done(_,s) => assert_eq!(String::from("hi"), s),
        _ => panic!("Failed to parse string")
    }

    match string(b"   hi    ") {
        IResult::Done(_,s) => assert_eq!(String::from("hi"), s),
        _ => panic!("Failed to parse string")
    }

    match string(b"hi      ") {
        IResult::Done(_,s) => assert_eq!(String::from("hi"), s),
        _ => panic!("Failed to parse string")
    }

    match string(b"        hi") {
        IResult::Done(_,s) => assert_eq!(String::from("hi"), s),
        _ => panic!("Failed to parse string")
    }
}

#[test]
fn user_op_parser() {
    match user(b"userop") {
        IResult::Done(_,s) => assert_eq!(Op::User(String::from("userop")), s),
        _ => panic!("Failed to parse userop")
    }
}

#[test]
fn specialform_op_parser() {
    match specialform(b"if") {
        IResult::Done(_,a) => assert_eq!(Op::SpecialForm(SForm::If), a),
        _ => panic!("Failed to parse special form")
    }
    match specialform(b"begin") {
        IResult::Done(_,a) => assert_eq!(Op::SpecialForm(SForm::Begin), a),
        _ => panic!("Failed to parse special form")
    }
    match specialform(b"define") {
        IResult::Done(_,a) => assert_eq!(Op::SpecialForm(SForm::Define), a),
        _ => panic!("Failed to parse special form")
    }
    match specialform(b"lambda") {
        IResult::Done(_,a) => assert_eq!(Op::SpecialForm(SForm::Lambda), a),
        _ => panic!("Failed to parse special form")
    }
    match specialform(b"let") {
        IResult::Done(_,a) => assert_eq!(Op::SpecialForm(SForm::Let), a),
        _ => panic!("Failed to parse special form")
    }
}

#[test]
fn primitive_op_parser() {
    match primitive(b"+") {
        IResult::Done(_,a) => assert_eq!(Op::Primitive(Prim::Add), a),
        _ => panic!("Failed to parse primitive")
    }
    match primitive(b"*") {
        IResult::Done(_,a) => assert_eq!(Op::Primitive(Prim::Mul), a),
        _ => panic!("Failed to parse primitive")
    }
    match primitive(b"-") {
        IResult::Done(_,a) => assert_eq!(Op::Primitive(Prim::Sub), a),
        _ => panic!("Failed to parse primitive")
    }
    match primitive(b"/") {
        IResult::Done(_,a) => assert_eq!(Op::Primitive(Prim::Div), a),
        _ => panic!("Failed to parse primitive")
    }
}

#[test]
fn op_parser() {
    match op(b"  +   ") {
        IResult::Done(_,a) => assert_eq!(Op::Primitive(Prim::Add), a),
        _ => panic!("Failed to parse primitive")
    }
    match op(b"   let   ") {
        IResult::Done(_,a) => assert_eq!(Op::SpecialForm(SForm::Let), a),
        _ => panic!("Failed to parse primitive")
    }
    match op(b"   myprocedure   ") {
        IResult::Done(_,a) => assert_eq!(Op::User(String::from("myprocedure")), a),
        _ => panic!("Failed to parse primitive")
    }
}

#[test]
fn procedure_test() {

    let procedure_num = Procedure {
        op: Op::Primitive(Prim::Add),
        args: vec!["1".to_string(), "2".to_string(), "3".to_string(), "4".to_string()]
    };

    let procedure_user = Procedure {
        op: Op::User(String::from("myprocedure")),
        args: Vec::new()
    };

    match procedure(b"(+ 1 2 3 4)") {
        IResult::Done(_,a) => assert_eq!(procedure_num, a),
        _ => panic!("Failed to parse primitive")
    }

    match procedure(b"(myprocedure)") {
        IResult::Done(_,a) => assert_eq!(procedure_user, a),
        _ => panic!("Failed to parse primitive")
    }
}
