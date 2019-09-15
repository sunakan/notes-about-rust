fn main() {
    let a = read();
    let b = read();
    let c = read();
    let x = read();
    let ans = solve(a, b, c, x);
    println!("{}", ans);
}

fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

// rust 1.15.1では for i in 0..=a {} みたいなのはできない
fn solve(a: u32, b: u32, c: u32, x: u32) -> u32 {
    let mut ans = 0;
    for i in 0..(a+1) {
        for j in 0..(b+1) {
            for k in 0..(c+1) {
                if 500*i + 100*j + 50*k == x {
                    ans += 1;
                }
            }
        }
    }
    ans
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(2, solve(2,2,2,100));
    }

    #[test]
    fn test2() {
        assert_eq!(0, solve(5,1,0,150));
    }

    #[test]
    fn test3() {
        assert_eq!(213, solve(30,40,50,6000));
    }
}
