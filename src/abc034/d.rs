#![allow(dead_code, unused_imports)]
use proconio::{input, marker::{Usize1, Isize1}};
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
    // 2024-12-12 20:02-20:54 (52min)
    // 2024-12-13 18:34-19:12 (38min, 貪欲法が嘘解法だと解説を見て知ったうえで、解き直し。解説見る前にも、決め打ち二分探索でも解けることは気づいていたので、貪欲法でWAになったら試す予定だった。)
    // 公式の解説 https://img.atcoder.jp/abc034/editorial.pdf?_gl=1*1xo2etc*_ga*NTMzMTE0MDg3LjE2Nzg1MDYzNzk.*_ga_RC512FD18N*MTczNDAwNDQ2My40OTMuMS4xNzM0MDA0NjEyLjAuMC4w
    // けんちょんの解説 https://drken1215.hatenablog.com/entry/2023/10/21/021300
    // Total: 90min
    input! {
        n: usize,
        k: usize,
    }
    let mut w = vec![];
    let mut p = vec![];
    let mut pws = vec![];
    for i in 0..n {
        input!{
            wi: usize,
            pi: usize,
        }
        w.push(wi as f64);
        p.push(pi as f64 / 100.0);
        pws.push((pi as f64 / 100.0, wi as f64));
    }
    // pws.sort();
    pws.sort_by(|x1, x2| x1.partial_cmp(x2).unwrap());
    pws.reverse();

    // fake_solve(n, k, pws);
    educational_solve(n, k, pws);
    // 貪欲法は、以下のケースを落とすので、嘘解法。
    // 4 3
    // 16 51
    // 51 64
    // 61 57
    // 4 26
    // 答えは59.039062499999993くらい

    // 解く前の実験: 
    // let p0 = 0.99;
    // let mut w0 = 1.0;
    
    // // 濃いけど、重い
    // let p1 = 0.5;
    // let w1 = 1000.0;

    // // 薄いけど、軽い
    // let p2 = 0.1;
    // let w2 = 10.0;

    // for _ in 0..10 {
    //     let mix1 = mix(p0, w0, p1, w1);
    //     let mix2 = mix(p0, w0, p2, w2);
    //     println!("{:?}", mix1);
    //     println!("{:?}", mix2);
    //     println!("{}", mix1.0 < mix2.0);
    //     w0 *= 10.0
    // }

}

fn educational_solve(n: usize, k: usize, pws: Vec<(f64, f64)>) {
    // 決め打ち二分探索
    // 濃度が決めたやつより、高いやつは取っていっていい。
    // 必要になる塩の量...

    let mut ok: f64 = 0.0;
    let mut ng: f64 = 1.0;
    
    let mut is_all_100_percent = true;
    for i in 0..n {
        if pws[i].0 != 1.0 {
            is_all_100_percent = false;
        }
    }
    
    if is_all_100_percent {
        println!("100.0");
        return
    }

    // let delta = 0.000_000_000_001;
    // let delta = 0.000_000_001;
    // let delta = 0.001;
    // while (ng - ok).abs() > delta {
    
    // 小数の決め打ち二分探索の場合、(ok-ng).abs()で判断せずに、固定した回数分やるのが主流っぽい。
    // 100回やれば、2^100 >= 1.2.. * 10^30 なので、十分に良い精度が出る。
    let num_iteration = 100; 
    for _ in 0..num_iteration {
        let mid = (ok + ng) / 2.0;
        // println!("mid = {:?}", mid);
        if judge(mid, n, k, &pws) {
            ok = mid;
        }
        else {
            ng = mid;
        }
    }

    println!("{}", ok * 100.0);
}

fn judge(mid: f64, n: usize, k: usize, pws: &Vec<(f64, f64)>) -> bool {
    // mid の濃度を満たすことはできるか?

    // salt_needs[i] := 食塩水iの、濃度をmidにするのに、必要な塩の量
    let mut salt_needs = vec![];
    for i in 0..n {
        let pi = pws[i].0;
        let wi = pws[i].1;
        // 必要な塩をxとする。
        // mid * (x + wi) == x +  wi * pi;
        // <=> x = wi*(mid - pi) / (1-mid)
        let salt_need = wi * (mid - pi) / (1.0 - mid);
        salt_needs.push(salt_need);
    }

    // 追加で必要な塩が少ない順にソート
    // salt_needs.sort();
    salt_needs.sort_by(|x1, x2| x1.partial_cmp(x2).unwrap());
    
    let mut salt_r = 0.0;
    for i in 0..k {
        salt_r -= salt_needs[i];
    }

    return salt_r >= 0.0
}

fn fake_solve(n: usize, k: usize, pws: Vec<(f64, f64)>) {
    // 貪欲法の解法。実は嘘解法 (cf. 公式解説)。

    // 一番濃いやつを選ぶ
    // そのあと、そいつをなるべく薄めない方針で行きたい
    // 基本的には、量が少ないやつ or もっとも濃いやつ
    
    let mut concentration = pws[0].0;
    let mut total_mass =pws[0].1;
    let mut used = vec![false; n];
    used[0] = true;
    for _ in 0..k-1 {
        let mut min_npi = 0.0;
        let mut min_wpi = 0.0;
        let mut index = n;
        for i in 0..n {
            if used[i] {continue}
            let pi = pws[i].0;
            let wi = pws[i].1;
            let (npi, nwi) = mix(concentration, total_mass, pi, wi);
            if npi > min_npi {
                index = i;
                min_npi = npi;
                min_wpi = nwi;
            }
        }
        concentration = min_npi;
        total_mass = min_wpi;
        used[index] = true;
    }
    println!("{}", concentration * 100.0);


    // k,n <= 10^3
    // wi <= 10^9
    // dp[i][k] := k個選んだときの、最高濃度
    // 濃いやつ + 量が少なくて薄いやつ のペアが良い。
    // let mut dp = vec![vec![0.0; k + 1]; n + 1];
    // for i in 0..n {
    //     for j in 0..k {

    //     }
    // }

    

    // pws.sort();
    // pws.reverse();

    // let mut salts = 0.0;
    // let mut total = 0.0;
    // for i in 0..k {
    //     salts += pws[i].0 as f64 / 100.0 * pws[i].1 as f64;
    //     total += pws[i].1 as f64;
    // }
    // println!("salts = {:?}", salts);
    // println!("total = {:?}", total);
    // let ans = 100.0 * salts / total;
    // println!("{}", ans);
}

fn mix(p0: f64, w0: f64, p1: f64, w1: f64) -> (f64, f64) {
    let np = (p0 * w0 + p1 * w1) / (w0 + w1);
    let nw = w0 + w1;
    return (np, nw)
}