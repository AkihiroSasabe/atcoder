#![allow(dead_code, unused_imports)]
use proconio::{input, marker::Usize1};
use itertools::Itertools;
use std::cmp::{max, min};
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::BTreeMap;
use std::ops::Bound::{Excluded, Included, Unbounded};
use std::collections::{HashSet, BTreeSet};
use proconio::marker::Chars;
use std::f64::consts::PI;
use std::mem::swap;
use superslice::*;
use rand::Rng;
fn main() {
    // 2024-11-21 19:52-20:31
    // Total: 39min 
    input! {
        n: usize,
    }
    let mut t = vec![];
    let mut a = vec![];
    for i in 0..n {
        input!{
            ti: usize,
            ai: usize,
        }
        t.push(ti);
        a.push(ai);
    }

    let mut ti = t[0];
    let mut ai = a[0];
    let mut res = vec![(ti, ai)];

    for i in 1..n {
        let tp = res[i-1].0;
        let ap = res[i-1].1;

        let mut ti = t[i];
        let mut ai = a[i];

        let mut ri = 1;


        // 10 / 3 = 3..1 -> 4
        // 9 / 3 = 3..0 -> 3
        // 余りが出たら+1したい。
        // 余りが0なら、そのまま
        // (N - 1) / D + 1

        let rt = (tp - 1) / t[i] + 1;
        let ra = (ap - 1) / a[i] + 1;

        let ri = max(rt, ra);
        let ti = t[i] * ri;
        let ai = a[i] * ri;
        res.push((ti, ai));


        // if ti < tp {
        //     let mut rt = tp / t[i];
        //     if tp % t[i] != 0 {
        //         rt += 1;
        //     }
        //     ri = rt;
        //     ti = t[i] * ri;
        //     ai = a[i] * ri;
        // }
        // if ai < ap {
        //     let mut ra = ap / a[i];
        //     if ap % a[i] != 0 {
        //         ra += 1;
        //     }
        //     ti = t[i] * ra;
        //     ai = a[i] * ra;
        // }
        // res.push((ti, ai));



        // // LCMを求める
        // let sump = tp + ap;
        // let sum = t[i] + a[i];
        // // GCD
        // let g = gcd(sump, sum);
        // let lcm = sump * sum / g;
        // println!("lcm = {:?}", lcm);

        // let min_t = lcm * t[i] / (t[i] + a[i]);
        // let min_a = lcm * a[i] / (t[i] + a[i]);

        // let mut rt = tp / min_t;
        // let mut ra = ap / min_a;
        // if tp % min_t != 0 {
        //     rt += 1;
        // }
        // if ap % min_a != 0 {
        //     ra += 1;
        // }

        // let ri = max(rt, ra);

        // let ti = min_t * ri;
        // let ai = min_a * ri;

        // res.push((ti, ai));
    }

    // for i in 0..n {
    //     println!("res[{i}] = {:?}", res[i]);
    // }

    let ans = res[n-1].0 + res[n-1].1;
    println!("{}", ans);


}


// ユークリッドの互除法で最大公約数を求める (Euclidean Algorithm)
// ユークリッドの互除法とは、x < y のとき、gcd(x, y)=gcd(x, y % x)
fn gcd(x: usize, y:usize) -> usize {
    if y == 0 {
        // 任意の整数xは0の約数と言える(∵0 % x == 0)ので、0とxの最大公約数はx
        return x;
    }
    else {
        return gcd(y, x % y);
    }
}