use number::Number;
#[test]
fn serde_parses_float() {
    let json = "10.5";
    let num: Number = serde_json::from_str(json).unwrap();
    assert_eq!(num, Number::from(21) / 2);
}
