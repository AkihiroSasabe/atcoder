use proconio::input;
use itertools::Itertools;
use std::cmp::{max, min};
use std::cmp::Ordering;
use std::collections::{VecDeque, HashSet};
use std::collections::BinaryHeap;
use std::collections::HashMap;
use proconio::marker::Chars;
use std::f64::consts::PI;
use std::mem::swap;
fn main() {
    // 2023-12-02 11:18-11:56 (38min)
    input! {
        n: usize,
        m: usize,
        a: isize,
        b: isize,
        c: isize,
        d: isize,
        e: isize,
        f: isize,
    }
    let mut ban = HashSet::new();
    for i in 0..m {
        input!{
            x_i: isize,
            y_i: isize,
        }
        ban.insert([x_i, y_i]);
    }
    let dir = [[a, b], [c, d], [e, f]];
    let modulus: usize = 998_244_353;

    // dp<現在到達可能な場所, 経路の個数>
    let mut dp = HashMap::new();
    dp.insert([0, 0], 1);

    for i in 0..n {
        // println!("dp = {:?}", dp);
        let mut next_dp = HashMap::new();
        for (p, num) in dp.iter() {
            for i in 0..3 {
                let mut np = p.clone();
                np[0] += dir[i][0];
                np[1] += dir[i][1];

                if ban.contains(&np) {
                    continue
                }

                *next_dp.entry(np).or_insert(0) += *num;
                *next_dp.get_mut(&np).unwrap() %= modulus;
            }
        }
        // swap(&mut dp, &mut next_dp);
        dp = next_dp;
    }

    let mut ans = 0;
    for (p, num) in dp.iter() {
        ans += *num;
        ans %= modulus;
    }
    println!("{}", ans);


    // DPの計算量の考察
    // dirが1方向なら、各回で許される状態数はNC0=1個だけ
    // 2方向なら、NC1=N個だけ
    // 3方向なら、NC2=N*(N-1)/2個だけ (∵state = n0 * dir[0] + n1 * dir[1] + n2 * dir[2], n0 + n1 + n2 = n の制約を満たす(n0,n1,n2)の組み合わせ数は、N+2C2に等しい)
    // 今回は3方向だから、
    // N*(N-1)/2 <= 300 * 299 / 2 = 44850 通り

}