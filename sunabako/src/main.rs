extern crate serde;
extern crate serde_json;
use std::collections::BTreeMap;

fn main() {
    println!("{}", hello());
    let s = r#"{"x":1,"y":2}"#;
    let deserialized_map: BTreeMap<String, f64> = serde_json::from_str(s).unwrap();
    println!("{:?}", deserialized_map);
    // => {"x": 1, "y": 2}
    println!("{:?}", deserialized_map.get("x").unwrap());
    // => 1
}

fn hello() -> String {
    return "hello".to_string();
}

#[cfg(test)]
mod tests {
    use super::*;
    // assertの練習
    #[test]
    fn test_assert() {
        assert_eq!(true, true);
        assert_ne!(true, false);
        assert_eq!("true", "true");
        assert_ne!("true", "false");
        //assert!(false, "always false"); // 必ず失敗する
        assert!(true, "always true"); // 必ず成功する
        let a = 3;
        let b = 3;
        assert!(a == b, "required {} == {}", a, b);
    }
}
