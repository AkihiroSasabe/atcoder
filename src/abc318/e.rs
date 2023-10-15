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
fn main() {
    input! {
        n: usize,
        a: [usize; n]
    }
    // 2023-09-18 18:09-19:38 (1.5h)

    // 値が等しい項をグループ化する
    let mut btree = BTreeMap::new();
    for i in 0..n {
        btree.entry(a[i]).or_insert(vec![]).push(i);
    }

    // 値が同じ項について、(i,k)の候補は、値が同じ項の個数をnumとすると、num-1 + num-2 + ... + 0 = num*(num-1)/2個の候補がある。
    // 例えば以下のように。
    // 数列Aのindex:                0, 1, 2, 3, 4, 5, 6, 7, 8, 9
    // 数列A:                       9, 1, 9, 2, 3, 9, 4, 5, 6, 9
    // 値が同じ項i:                 0     1        2           3
    // i,i+1の間にある項の個数num:     1個     2個       3個
    
    // これで全探索するとTLEしてしまう。
    // 全探索をやめて、解析的に解くことを考える。
    // [0,1]と[1,2]と[2,3]の各区間は、それぞれ
    // (1, 2, 3)個のjの候補があり、
    // (i,k)の組み合わせ次第で、(3, 4, 3)回足し算される。
    // よって、(i,j,k)の組の総数は、1*3 + 2*4 + 3*3 = 3+6+9=18
    
    // 次に、これをうまく定式化する方法を考える
    // v := 同じ値の項のindexが格納されたvector
    // v.len()-1: 各区間の足される回数
    // 1:   (1)
    // 2:   (2,2)
    // 3:   (3,4,3)
    // 4:   (4,6,6,4)
    // 5:   (5,8,9,8,5)
    // 6:   (6,10,12,12,10,6)
    // 7:   (7,12,15,16,15,12,7)
    // 階差数列になっていることに気づく。
    // 初項はv.len()-1となっていて、各項目の項差は等しい。
    // 例えば7個目は、次の項 - 前の項 を見てみると
    // +5, +3, +1, -1, -3, -5
    // となっている。
    // これは、公差-2の等差数列である。
    // 階差数列の公差は、いかなるv.len()においても、-2である。
    // これで定式化できた。
    
    let mut ans = 0;
    for (k, v) in &btree {
        if v.len() >= 2 {
            // 全部同じだったとき、N!でTLEするO(N^2)
            // for i in 0..v.len() {
            //     for j in i+1..v.len() {
            //         ans += (v[j] - v[i] - (j-i));
            //     }
            // }
            
            // [i, i+1]の区間を何回足すか? O(N)
            let mut num: isize = v.len() as isize - 1;
            // 次に足す回数 と 今回足す回数 の差分
            let mut diff: isize = v.len() as isize - 3;
            for i in 0..(v.len()-1) {
                // iとkの間にあるjの個数
                let sukima = v[i+1] - v[i] - 1;
                ans += (sukima as isize * num) as usize;
                num += diff;
                diff -= 2;
            }
        }
    }
    println!("{}", ans);

    
}