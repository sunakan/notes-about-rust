struct Point {
    x: i64,
    y: i64,
}
struct Plan {
    time: i64,
    point: Point,
}

fn main() {
    let n = read();
    let mut plans = Vec::with_capacity(n);
    for _ in 0..n {
        let txy = read_vec();
        let plan = Plan {
            time: txy[0],
            point: Point {
                x: txy[1],
                y: txy[2],
            },
        };
        plans.push(plan);
    }
    let ans = solve(plans);
    println!("{}", ans);
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

fn solve(mut plans: Vec<Plan>) -> String {
    plans.sort_by_key(|p| p.time);
    let mut current_time: i64 = 0;
    let mut current_point: Point = Point { x: 0, y: 0 };
    for plan in plans.into_iter() {
        let time_diff = plan.time - current_time;
        let dist = (plan.point.x - current_point.x).abs() + (plan.point.y - current_point.y).abs();
        if time_diff < dist {
            return "No".to_string()
        }
        if time_diff%2 != dist%2 {
            return "No".to_string()
        }
        current_time = plan.time;
        current_point = plan.point;
    }
    "Yes".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        let mut plans = Vec::with_capacity(2);
        plans.push(Plan { time: 3, point: Point { x: 1, y: 2 } });
        plans.push(Plan { time: 6, point: Point { x: 1, y: 1 } });
        assert_eq!("Yes", solve(plans));
    }

    #[test]
    fn test2() {
        let mut plans = Vec::with_capacity(1);
        plans.push(Plan { time: 2, point: Point { x: 100, y: 100 } });
        assert_eq!("No", solve(plans));
    }

    #[test]
    fn test3() {
        let mut plans = Vec::with_capacity(2);
        plans.push(Plan { time: 5,   point: Point { x: 1, y: 1 } });
        plans.push(Plan { time: 100, point: Point { x: 1, y: 1 } });
        assert_eq!("No", solve(plans));
    }
}
