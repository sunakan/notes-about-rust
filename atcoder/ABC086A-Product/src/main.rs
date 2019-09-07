fn main() {
    let ab: Vec<u32> =read_vec();
    let product: u32 = ab[0] * ab[1];
    if product%2 == 0 {
       println!("Even");
    } else {
       println!("Odd");
    }
}

fn read_vec<T: std::str::FromStr>() -> Vec<T> {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().split_whitespace()
        .map(|e| e.parse().ok().unwrap()).collect()
}
