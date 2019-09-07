fn main() {
    let line: String = read();
    // Matchでやる方法
    // https://qiita.com/aflc/items/f2be832f9612064b12c6
    //let count: usize = line.match_indices("1").count();
    // 別解
    let count: usize = line.chars().filter(|&c| c == '1').count();
    println!("{}", count);
}

fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}
