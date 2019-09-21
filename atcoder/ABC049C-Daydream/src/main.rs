fn main() {
    let input = read();
    let ans = solve(input);
    println!("{}", ans);
}

fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

fn solve(s: String) -> String {
    // 俺の解
    //let result = s.split("eraser").collect::<String>()
    //    .split("erase").collect::<String>()
    //    .split("dreamer").collect::<String>()
    //    .split("dream").collect::<String>();
    //if result.len() == 0 {
    //    "YES".to_string()
    //} else {
    //    "NO".to_string()
    //}
    // 別解
    let patterns = [
        "eraser",
        "erase",
        "dreamer",
        "dream"
    ];
    let s = patterns.iter().fold(s, |s, x| s.replace(x, ""));
    if s.is_empty() {
        "YES"
    } else {
        "NO"
    }.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!("YES", solve("erasedream".to_string()));
    }

    #[test]
    fn test2() {
        assert_eq!("YES", solve("dreameraser".to_string()));
    }

    #[test]
    fn test3() {
        assert_eq!("NO", solve("dreamerer".to_string()));
    }
}
