use number::Number;
#[test]
fn serde_parses_integer() {
    let json = "10";
    let num: Number = serde_json::from_str(json).unwrap();
    assert_eq!(num, Number::from(10i32));
}
