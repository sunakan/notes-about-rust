fn main() {
    // 1行捨てる
    let mut s = String::new();
    std::io::stdin().read_line(&mut s);
    // 本番
    let arr = read_vec();
    // 再帰用
    //let ans = solv(arr, 0);
    let ans = solv(arr);
    println!("{}", ans);
}

fn read_vec<T: std::str::FromStr>() -> Vec<T> {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().split_whitespace()
        .map(|e| e.parse().ok().unwrap()).collect()
}

// 再帰
//fn solv(arr: Vec<u32>, count: u32) -> u32 {
//    let even_only_arr: Vec<u32> = arr.clone().into_iter().filter(|x| x%2 == 0).collect();
//    if arr.iter().count() == even_only_arr.iter().count() {
//        let next: Vec<u32> = even_only_arr.into_iter().map(|x| x / 2).collect();
//        solv(next, count + 1)
//    } else {
//        count
//    }
//}

// 別解(こっちのほうがスマート)
// 2進数にしたとき1の位から何回1が連続しているか
// minはOptionを返すが、今回は1以上であることを問題が保証しているため、unwrapしてOKとのこと
// メソッドチェインしてもよかったが、今回は上の解説コメントを入れるために分けた
fn solv(arr: Vec<u32>) -> u32 {
    let ans = arr.into_iter().map(|v| v.trailing_zeros()).min();
    ans.unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        //再帰用
        //assert_eq!(2, solv(vec! [8, 12, 40], 0));
        assert_eq!(2, solv(vec! [8, 12, 40]));
    }
    #[test]
    fn test2() {
        //再帰用
        //assert_eq!(0, solv(vec! [5, 6, 8, 10], 0));
        assert_eq!(0, solv(vec! [5, 6, 8, 10]));
    }
    #[test]
    fn test3() {
        let arr = vec! [382253568, 723152896, 37802240, 379425024, 404894720, 471526144];
        //再帰用
        //assert_eq!(8, solv(arr, 0));
        assert_eq!(8, solv(arr));
    }
}
