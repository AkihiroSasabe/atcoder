#![allow(dead_code, unused_imports)]
use proconio::input;
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
    // 2024-09-21 16:02-17:47 (1h45min, 降参)
    // 2024-09-21 17:47-18:02 (15min, 解説AC。公式の解説を参考)
    // Total: 2h
    input! {
        k: usize
    }
    // 出力の制約
    // 2 <= N <= 50
    // 0 <= ai <= 10^16 + 1000

    // 解説AC (公式解説参考)
    // N=50で固定して、a = [0,1,...,49] で終わることを考える。
    //      備考: 1回の操作で-1されるので、K回で-Kされる。
    // k = 0: a = [0,  1,   2,  3,  4, ..., 49]
    // k = 1: a = [50, 0,   1,  2,  3, ..., 48]
    // k = 2: a = [49, 50,  0,  1,  2, ..., 47]
    // k = 3: a = [48, 49, 50,  0,  1, ..., 46]
    // k = 4: a = [47, 48, 49, 50,  0, ..., 45]
    // ...
    // k =50: a = [ 1,  2,  3,  4,  5, ..., 50] <- K=50で、1周すると、全要素に+1される。
    // k =51: a = [51,  1,  2,  3,  4, ..., 49] <- K=1のときと、似た周期性が生まれる。
    // つまり、50周期の処理をうまく、実装に落とし込めばok。

    // 1回の操作で、全体的に-1減ることには気づいて利用していたが、
    // 0,1,2,...,49を初期状態にすることは思いつかなかった。(全部49で初期化することを考えてしまった。)


    // 終状態 (N=50で固定)
    let n = 50;
    let mut a: Vec<usize> = vec![];
    for i in 0..n {
        a.push(i);
    }

    let q = k / n;
    let r = k % n;
    for i in 0..n {
        a[i] += q;
    }
    for i in 0..r {
        a[n-1-i] += 1;
    }
    println!("{}", n);
    for i in 0..n {
        print!("{} ", a[i]);
    }

    return;

    // // 自分のオリジナル解法 (WA)
    // if k == 0 {
    //     println!("4");
    //     println!("3 3 3 3");
    //     return
    // }
    // if k == 1 {
    //     println!("3");
    //     println!("1 0 3");
    //     return
    // }

    // let max_n = 50;
    // // let max_n = 2; // debug
    // let mut n = min(max_n, k); // max_n が採用されたときに、WA になりがち。
    // // n = max(n, 2); // k = 0, 1 のときは、別の分岐で既に処理しているので、ここまででnが2未満になることはない。
    // println!("n = {:?}", n);

    // // 全操作の終了状態で初期化
    // let mut a = vec![n-1; n];
    // println!("end of a = {:?}", a);


    // // let d = k / n;
    // // let r = k % n;
    // // for i in 0..n {
    // //     a[i] += d;
    // // }
    // // for i in 0..r {
    // //     a[i] += 1;
    // // }
    // // できるかぎり、一点集中で攻めるべき。




    // a.sort();
    // // println!("a = {:?}", a);

    // println!("{}", n);
    // for i in 0..n {
    //     print!("{} ", a[i]);
    // }
    // println!("");

    // let count = ope(a);
    // println!("count = {:?}", count);

    // // 24_691_357_946
    // // 10_000_000_000_000_000 = 10^16



}

fn ope(mut a: Vec<usize>) -> usize {
    a.sort();
    println!("a = {:?}", a);

    let n = a.len();
    // 最大を見つける
    a.sort();
    let max_a = a[n-1];
    if max_a <= n - 1 {
        return 0
    }
    a[n-1] -= n;
    for i in 0..n-1 {
        a[i] += 1;
    }
    return ope(a) + 1
}