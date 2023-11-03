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
use superslice::*;
fn main() {
    // 2023-11-02 20:58-21:30 降参. 強プロフレンズの半分全列挙
    // 2023-11-02 21:30-22:00 提出するもTLE (1h02min)
    // 2023-11-03 09:34-10:22 (48min)
    // total: 1h50min = 110min
    input! {
        n: usize,
        a: [[usize; n]; n]
    }
    // for i in 0..n {
    //     for j in 0..n {
    //         println!("{} =  {:03b}", a[i][j], a[i][j]);
    //     }
    // }
    // 20^30 =   1_073_741_824
    // 20^29 =     536_870_912
    // 40C20 = 137_846_528_820
    // 38C19 =  35_345_263_800

    // 半分全列挙の計算量
    // ∑[i=0,19] 19-1_C_i = 524_288
    // 19C0 + 19C1 + ... 19C19 = (1+1)^19 = 2^19 = 524_288

    // 半分全列挙
    // 原点からスラッシュ（左上領域）
    let mut pre = vec![vec![]; n];
    for i in 0..n {
        for comb in (0..n-1).combinations(i) {
            let mut hash = HashSet::new();
            for ind in comb {
                hash.insert(ind);
            }
            let mut xor_sum = a[0][0];
            let mut y = 0;
            let mut x = 0;
            for j in 0..n-1 {
                if hash.contains(&j) {
                    y += 1;
                }
                else {
                    x += 1;
                }
                xor_sum ^= a[y][x];
            }
            pre[i].push(xor_sum);
        }
    }

    // スラッシュからゴール（右下領域）
    let mut pos = vec![HashMap::new(); n];
    // let mut pos = vec![vec![]; n];
    for i in 0..n {
        for comb in (0..n-1).combinations(i) {
            let mut hash = HashSet::new();
            for ind in comb {
                hash.insert(ind);
            }
            let mut xor_sum = 0;
            let mut y = i;
            let mut x = n- 1 - i;
            for j in 0..n-1 {
                if hash.contains(&j) {
                    x += 1;
                }
                else {
                    y += 1;
                }
                xor_sum ^= a[y][x];
            }
            // pos[i].push(xor_sum);
            *pos[i].entry(xor_sum).or_insert(0) += 1;
        }
    }
    let mut ans: usize = 0;
    // let mut count = 0;
    for i in 0..n {
        // 排他的論理和が0になる組み合わせを全探索すると間に合わない
        // count += pre[i].len() * pos[i].len();
        // for j in 0..pre[i].len() {
        //     for k in 0..pos[i].len() {
        //         if pre[i][j] ^ pos[i][k] == 0 {
        //             ans += 1;
        //         }
        //     }
        // }

        // 実は数xとの排他的論理和が0になる組み合わせは、xだけ。
        // 例
        // x=10
        // 00, 01, 10, 11
        // --, --, **, --

        // x=11
        // 00, 01, 10, 11
        // --, --, --, **

        // x=100
        // 000, 001, 010, 011, 100, 101, 110, 111
        // ---, ---, ---, ---, ***, ---, ---, ---
        for j in 0..pre[i].len() {
            if pos[i].contains_key(&pre[i][j]) {
                ans += pos[i][&pre[i][j]];
            }
        }
    }
    // 35_345_263_800
    // println!("count = {:?}", count);
    println!("{}", ans);

}