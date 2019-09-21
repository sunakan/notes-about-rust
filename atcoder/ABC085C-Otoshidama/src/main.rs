fn main() {
    let input: Vec<u32> = read_vec();
    let n = input[0];
    let y = input[1];
    let ans = solve(n, y);
    println!("{}", ans.into_iter().map(|v| v.to_string() ).collect::<Vec<String>>().join(" "));
}

fn solve(n: u32, y: u32) -> Vec<i32> {
    for i in 0..(n+1) {
        for j in 0..(n-i+1) {
            let k = n - i - j;
            let sum = 10000*i + 5000*j + 1000*k;
            if sum == y {
                return vec! [i as i32, j as i32, k as i32]
            }
        }
    }
    vec! [-1, -1, -1]
}

fn read_vec<T: std::str::FromStr>() -> Vec<T> {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().split_whitespace()
        .map(|e| e.parse().ok().unwrap()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(vec! [4, 0, 5], solve(9, 45000));
    }

    #[test]
    fn test2() {
        assert_eq!(vec! [-1, -1, -1], solve(20, 196000));
    }

    #[test]
    fn test3() {
        assert_eq!(vec! [2000, 0, 0], solve(2000, 20000000));
    }
}
