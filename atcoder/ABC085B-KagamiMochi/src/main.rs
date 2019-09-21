use std::collections::HashSet;

fn main() {
    let n: usize = read();
    let mut arr = Vec::with_capacity(n);
    for _ in 0..(n) {
        arr.push(read());
    }
    let ans = solve(arr);
    println!("{}", ans);
}

fn solve(values: Vec<u32>) -> usize {
    let uniq: HashSet<u32> = values.into_iter().collect();
    uniq.len()
}

fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(3, solve(vec! [10, 8, 8, 6]));
    }

    #[test]
    fn test2() {
        assert_eq!(1, solve(vec! [15, 15, 15]));
    }

    #[test]
    fn test3() {
        assert_eq!(4, solve(vec! [50, 30, 50, 100, 50, 80, 30]));
    }
}
