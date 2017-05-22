use super::{parse, ast};

#[test]
fn test_symbol() {
    let text = "foo";
    let result = parse(text).unwrap();
    let first_symbol = &result[0];
    match first_symbol {
        &ast::Statement::Symbol(ref s) => {
            assert_eq!(s, &Box::new(String::from("foo")));
        },
        _ => {
            panic!("symbol did not match.");
        }
    }
}
