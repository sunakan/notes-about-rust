fn main() {
    let nab = read_vec();
    let n = nab[0];
    let a = nab[1];
    let b = nab[2];
    let ans = solve1(n, a, b);
    println!("{}", ans);
}

fn read_vec<T: std::str::FromStr>() -> Vec<T> {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().split_whitespace()
        .map(|e| e.parse().ok().unwrap()).collect()
}

// 自分で解いたとき
fn solve1(n: u32, a: u32, b: u32) -> u32 {
    let mut ans = 0;
    for i in 1..(n+1) {
        let sum = i/10000 + ((i%10000)/1000) + ((i%1000)/100) + ((i%100)/10) + (i%10);
        if a<=sum && sum<=b {
            ans += i;
        }
    }
    ans
}

// webを見ての別解
fn solve2(n: u32, a: u32, b: u32) -> u32 {
    (1..n+1).filter( |i| {
        let sum = i.to_string()
            .chars()
            .map(|c| c as u32 - '0' as u32)
            .sum::<u32>();
        a<=sum && sum <= b
    }).sum::<u32>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(84, solve1(20,2,5));
        assert_eq!(84, solve2(20,2,5));
    }

    #[test]
    fn test2() {
        assert_eq!(13, solve1(10,1,2));
        assert_eq!(13, solve2(10,1,2));
    }

    #[test]
    fn test3() {
        assert_eq!(4554, solve1(100, 4, 16));
        assert_eq!(4554, solve2(100, 4, 16));
    }
}
