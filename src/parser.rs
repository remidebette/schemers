use nom::{
    IResult,
    branch::alt,
    combinator::map,
    bytes::complete::tag,
    character::complete::{alphanumeric1, char},
    multi::many0,
    sequence::delimited,
};

fn procedure(i: &str) -> IResult<&str, Procedure> {
    delimited(
        char('('),
        procedure_body,
        char(')')
    )(i)
}

fn procedure_body(i: &str) -> IResult<&str, Procedure> {
    let (i, op_type) = op(i)?;
    let (i, arguments) = many0(string)(i)?;

    Ok((i, Procedure {op: op_type, args: arguments}))
}

fn op(i: &str) -> IResult<&str, Op> {
    let i = i.trim();
    alt((
        specialform,
        primitive,
        user,
    ))(i)
}

fn specialform(i: &str) -> IResult<&str, Op> {
    alt((
        map(tag("if"), |_| Op::SpecialForm(SForm::If)),
        map(tag("begin"), |_| Op::SpecialForm(SForm::Begin)),
        map(tag("define"), |_| Op::SpecialForm(SForm::Define)),
        map(tag("lambda"), |_| Op::SpecialForm(SForm::Lambda)),
        map(tag("let"), |_| Op::SpecialForm(SForm::Let)),
    ))(i)
}

fn primitive(i: &str) -> IResult<&str, Op> {
    alt((
        map(tag("*"), |_| Op::Primitive(Prim::Mul)),
        map(tag("+"), |_| Op::Primitive(Prim::Add)),
        map(tag("-"), |_| Op::Primitive(Prim::Sub)),
        map(tag("/"), |_| Op::Primitive(Prim::Div)),
    ))(i)
}

fn user(i: &str) -> IResult<&str, Op> {
    let (i, user_proc) = string(i)?;
    Ok((i, Op::User(user_proc)))
}

fn string(i: &str) -> nom::IResult<&str, String> {
    let (i, word) = alphanumeric1(i.trim())?;
    Ok((i, String::from(word)))
}

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
    let comp_string = String::from("hi");

    match string("hi") {
        Ok((_, s)) => assert_eq!(comp_string, s),
        _ => panic!("Failed to parse string")
    }

    match string("   hi    ") {
        Ok((_, s)) => assert_eq!(comp_string, s),
        _ => panic!("Failed to parse string")
    }

    match string("hi      ") {
        Ok((_, s)) => assert_eq!(comp_string, s),
        _ => panic!("Failed to parse string")
    }

    match string("        hi") {
        Ok((_, s)) => assert_eq!(comp_string, s),
        _ => panic!("Failed to parse string")
    }
}

#[test]
fn user_op_parser() {
    match user("userop") {
        Ok((_, s)) => assert_eq!(Op::User(String::from("userop")), s),
        _ => panic!("Failed to parse userop")
    }
}

#[test]
fn specialform_op_parser() {
    match specialform("if") {
        Ok((_, a)) => assert_eq!(Op::SpecialForm(SForm::If), a),
        _ => panic!("Failed to parse special form")
    }
    match specialform("begin") {
        Ok((_, a)) => assert_eq!(Op::SpecialForm(SForm::Begin), a),
        _ => panic!("Failed to parse special form")
    }
    match specialform("define") {
        Ok((_, a)) => assert_eq!(Op::SpecialForm(SForm::Define), a),
        _ => panic!("Failed to parse special form")
    }
    match specialform("lambda") {
        Ok((_, a)) => assert_eq!(Op::SpecialForm(SForm::Lambda), a),
        _ => panic!("Failed to parse special form")
    }
    match specialform("let") {
        Ok((_, a)) => assert_eq!(Op::SpecialForm(SForm::Let), a),
        _ => panic!("Failed to parse special form")
    }
}

#[test]
fn primitive_op_parser() {
    match primitive("+") {
        Ok((_, a)) => assert_eq!(Op::Primitive(Prim::Add), a),
        _ => panic!("Failed to parse primitive")
    }
    match primitive("*") {
        Ok((_, a)) => assert_eq!(Op::Primitive(Prim::Mul), a),
        _ => panic!("Failed to parse primitive")
    }
    match primitive("-") {
        Ok((_, a)) => assert_eq!(Op::Primitive(Prim::Sub), a),
        _ => panic!("Failed to parse primitive")
    }
    match primitive("/") {
        Ok((_, a)) => assert_eq!(Op::Primitive(Prim::Div), a),
        _ => panic!("Failed to parse primitive")
    }
}

#[test]
fn op_parser() {
    match op("  +   ") {
        Ok((_, a)) => assert_eq!(Op::Primitive(Prim::Add), a),
        _ => panic!("Failed to parse primitive")
    }
    match op("   let   ") {
        Ok((_, a)) => assert_eq!(Op::SpecialForm(SForm::Let), a),
        _ => panic!("Failed to parse special form")
    }
    match op("   myprocedure   ") {
        Ok((_, a)) => assert_eq!(Op::User(String::from("myprocedure")), a),
        _ => panic!("Failed to parse user op")
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
    let proc = procedure("(+ 1    2    3  4)");

    match proc {
        Ok((_, a)) => assert_eq!(procedure_num, a),
        _ => panic!("Failed to parse primitive")
    }

    let proc = procedure("(myprocedure)");

    match proc {
        Ok((_, a)) => assert_eq!(procedure_user, a),
        _ => panic!("Failed to parse primitive")
    }
}
