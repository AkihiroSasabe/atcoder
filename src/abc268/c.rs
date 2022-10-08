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
        p: [usize; n]
    }

    // 各回転毎に、何人を喜ばせることが出来るかを格納
    let mut happy = vec![0; n];
    for i in 0..n {
        // 料理jを人[j-1, j, j+1]に届けるのに必要な回転
        let j = p[i];
        happy[(n + j-1 - i) % n] += 1;
        happy[(n + j   - i) % n] += 1;
        happy[(n + j+1 - i) % n] += 1;
    }

    let mut ans = 0;
    for i in 0..n {
        ans = max(ans, happy[i]);
    }
    println!("{}", ans);


    // let mut p0 = vec![0; n];
    // let mut p1 = vec![0; n];
    // let mut p2 = vec![0; n];

    // let mut ans = 0;
    // for i in 0..n {
    //     p0[i] = (p[i] + 2*n - p[0] - 1) % n;
    //     p1[i] = (p[i] + 2*n - p[0]) % n;
    //     p2[i] = (p[i] + 2*n - p[0] + 1) % n;
    //     if p0[i] == i || p1[i] == i || p2[i] == i {
    //         ans += 1;
    //     }
    // }
    // println!("{}", ans);

}