use proconio::input;
use itertools::Itertools;
use std::cmp::{max, min};
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use proconio::marker::Chars;
use std::f64::consts::PI;
use std::mem::swap;
fn main() {
    input! {
        n: usize,
        l: isize,
        r: isize,
        mut a: [isize; n]
    }
    let mut cum = cummulative_sum(&a);

    // 1. 置換なし
    let mut ans = cum[n-1];
    // 2. 全てRで置換
    ans = min(ans, n as isize * r);
    // 3. 全てLで置換
    ans = min(ans, n as isize * l);
    // 4. Lで一回も置換しない
    for y in 1..n {
        let sum = cum[y-1] + (n - y) as isize * r;
        ans = min(ans, sum);
    }
    // 5. Rで一回も置換しない
    for x in 1..n {
        let sum = cum[n-1] - cum[x] + (x + 1) as isize * l;
        ans = min(ans, sum);
    }
    // 6. x < yのとき
    let INF = std::isize::MAX; // 9_223_372_036_854_775_807 = 9.2... * 10^18
    // y_term[x] := x<yを満たすyのうち、項 cum[y-1] - r * y が最小になる値
    let mut y_term = vec![INF; n];
    let mut sum = INF;
    for yy in 1..n {
        let y = n - yy;
        sum = min(sum, cum[y-1] - r * y as isize);
        y_term[y-1] = sum;
    }
    for x in 0..n-1 {
        ans = min(ans, y_term[x] -cum[x] + x as isize * l + n as isize * r + l)
    }

    println!("{}", ans);

}

fn cummulative_sum(a: &Vec<isize>) -> Vec<isize> {
    let mut left_sum = vec![0; a.len()];
    left_sum[0] = a[0];
    for i in 1..a.len() {
        left_sum[i] = left_sum[i-1] + a[i];
    }
    return left_sum
}