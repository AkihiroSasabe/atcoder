use proconio::input;
use itertools::Itertools;
use std::cmp::{max, min};
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use proconio::marker::Chars;
use std::f64::consts::PI;
fn main() {
    input! {
        x: isize,
        a: isize,
        d: isize,
        n: isize
    }
    let s_max = max(a, a + d * (n-1));
    let s_min = min(a, a + d * (n-1));
    if s_max < x {
        println!("{}", x - s_max);
        return
    }
    if x < s_min {
        println!("{}", s_min - x);
        return
    }

    if d == 0 {
        println!("{}", min(x - s_min, s_max - x));
        return
    }

    let amari = (x - s_min) % (d.abs());
    let answer = min(amari, d.abs() - amari);
    println!("{}", answer);







    // // println!("aa");

    // // -555_555_555_555_555_555     x = -5.55 * 10^17
    // // -1_000_000_000_000_000_000   a = -1 * 10^18
    // // 1_000_000                    d = 10^6
    // // 1_000_000_000_000            n = 10^12

    // let mut s = vec![];
    // for i in 0..n {
    //     let added = a + d * i;
    //     // let added = a + d * (i as isize);
    //     s.push(added);
    //     if i > 1_0_000_000  {
    //         println!("{}", i);
    //     }
    // }
    // println!("bb");
    // if d < 0 {
    //     s.reverse();
    // }
    // // println!("{:?}", s);
    // let cand = s.upper_bound(&x);
    // // println!("cc");
    // if cand == n as usize {
    //     println!("{}", x - s[n as usize-1]);
    //     return 
    // }
    // if cand == 0 {
    //     println!("{}", s[cand] - x);
    //     return
    // }
    // println!("{}", min(s[cand] - x, x - s[cand-1]));
}
