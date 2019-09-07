fn main() {
    let a: u32 = read();
    let bc: Vec<u32> = read_vec();
    let s: String = read();
    let sum: u32 = a + bc.iter().fold(0, |sum, i| sum + i);

    println!("{} {}", sum, s);
}

fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

fn read_vec<T: std::str::FromStr>() -> Vec<T> {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().split_whitespace()
        .map(|e| e.parse().ok().unwrap()).collect()
}
