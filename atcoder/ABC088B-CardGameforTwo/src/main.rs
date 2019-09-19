fn main() {
    // 1行捨てる
    let mut s = String::new();
    std::io::stdin().read_line(&mut s);
    // 本番
    let arr = read_vec();
    let ans = solve(arr);
    println!("{}", ans);
}

//fn read<T: std::str::FromStr>() -> T {
//    let mut s = String::new();
//    std::io::stdin().read_line(&mut s).ok();
//    s.trim().parse().ok().unwrap()
//}

fn read_vec<T: std::str::FromStr>() -> Vec<T> {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().split_whitespace()
        .map(|e| e.parse().ok().unwrap()).collect()
}

fn solve(mut arr: Vec<u32>) -> u32 {
    arr.sort_by(|a, b| b.cmp(a));
    let mut a = 0;
    let mut b = 0;
    for (i, val) in arr.iter().enumerate() {
        if (i%2 == 0) {
            a += *val;
        } else {
            b += *val;
        }
    }
    a - b
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(2, solve(vec! [3, 1]));
    }

    #[test]
    fn test2() {
        assert_eq!(5, solve(vec! [2, 7, 4]));
    }

    #[test]
    fn test3() {
        assert_eq!(18, solve(vec! [20, 18, 2, 18]));
    }
}
