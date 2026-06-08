use m2_rust_interpreter::{lexer::tokenize, parser::parse, convert::convert};

#[test]
fn parse_assignment_and_sequence() {
    let source = "x = 1; y := 2";
    let tokens = tokenize(source, "test.m2").expect("tokenize failed");
    let tree = parse(&tokens).expect("parse failed");
    let code = convert(tree);
    let debug = format!("{:#?}", code);
    assert!(debug.contains("x"));
    assert!(debug.contains("1"));
    assert!(debug.contains("y"));
    assert!(debug.contains(":="));
}
