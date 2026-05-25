use number::Number;
use serde::Deserialize;

fn main() {
    let json = r#"123"#;
    let num: Number = serde_json::from_str(json).unwrap();
    println!("{:?}", num);
}
