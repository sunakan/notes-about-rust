fn main() {
    println!("{}", hello());
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
