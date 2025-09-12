#![allow(dead_code, unused_imports)]
use proconio::{input, marker::{Usize1, Isize1, Chars}};
use itertools::Itertools;
use std::cmp::{max, min, Ordering, Reverse};
use std::ops::Bound::{Excluded, Included, Unbounded};
use std::collections::{VecDeque, BinaryHeap, HashMap, BTreeMap, HashSet, BTreeSet};
use std::f64::consts::PI;
use std::mem::swap;
use superslice::*;
use rand::Rng;
use num::{BigUint, ToPrimitive, Integer};
use num_bigint::ToBigUint;
fn main() {
    // 2025-09-10 09:00-9:10 (10min, 電車)
    // 2025-09-10 12:21-13:01 (40min)
    // Total; 50min
    input! {
        n: usize,
        k: usize,
    }

    // 2^20 = 1_048_576

    // 基本的には、各項に同じ値を格納すればいい。
    // ただし、kが2^nで割り切れないときは、k%(2^n)個の項に1大きい値を格納する必要がある。
    // ではどの項目を1大きくするか？

    // 例えば、N=4 のときは、数列Bは長さ16であるが、以下の順序で値を格納していくと、最終的なアンバランスさが最小になる。
    // 0 1 2 3 4 5 6 7 8 9 A B C D E F  
    // ===============================
    // 0 _ _ _ _ _ _ _ 1 _ _ _ _ _ _ _　
    // 0 _ _ _ 2 _ _ _ 1 _ _ _ 3 _ _ _  
    // 0 _ 4 _ 2 _ 6 _ 1 _ 5 _ 3 _ 7 _  
    // 0 8 4 C 2 A 6 E 1 9 5 D 3 B 7 F  

    // これをもう少し言語化すると、
    // 閉区間(l,r)にrem個の値を渡す時、閉区間(l,r)を2等分したときに、
    // 左にrem/2個、右にrem-rem/2個を渡すようにすると、各操作でmax(A)-min(A)をなるべく最小にできて、最終的なアンバランスさが最小になる。
    // これを(l,r)=(0,2^n-1)から(l,r)=(i,i)になるまで繰り返す。
    //      0 1 2 3 4 5 6 7 8 9 A B C D E F  
    // =====================================
    // L=16 0 _ _ _ _ _ _ _ _ _ _ _ _ _ _ _  [(0,15)] の各区間にrem個ずつ配置する。
    // L=8  0 _ _ _ _ _ _ _ 1 _ _ _ _ _ _ _　[(0,7), (8,15)] の各区間にrem/2個ずつ配置する。
    // L=4  0 _ _ _ 2 _ _ _ 1 _ _ _ 3 _ _ _  [(0,3), (4,7), (8,11), (12,15)] の各区間にrem/4個ずつ配置する。
    // L=2  0 _ 4 _ 2 _ 6 _ 1 _ 5 _ 3 _ 7 _  [(0,1), (2,3), (4,5), (6,7), (8,9), (10,11), (12,13), (14,15)] の各区間にrem/8個ずつ配置する。
    // L=1  0 8 4 C 2 A 6 E 1 9 5 D 3 B 7 F  [(0,0), (1,1), (2,2), (3,3) (4,4), (5,5), (6,6), (7,7), (8,8), (9,9), (10,10), (11,11), (12,12), (13,13), (14,14), (15,15)] の各区間にrem/16個ずつ配置する。

    // これはDFSで実装できる。
    // 計算量は、O(2^n)

    let num = 1 << n;
    let rem: usize = k % num;
    let mut b = vec![k / num; num];
    fn dfs(b: &mut Vec<usize>, l: usize, r: usize, rem: usize, unbalance: &mut usize) {
        if l == r {
            b[l] += rem;
            return;
        }

        // remを区間0と区間1に分ける。
        let rem0 = rem / 2;
        let rem1 = rem - rem0;
        *unbalance = max(*unbalance, (rem0 as isize - rem1 as isize).abs() as usize);

        // 閉区間0と閉区間1を計算。
        let l0 = l;
        let r0 = l + (r - l) / 2;
        let l1 = r0 + 1;
        let r1 = r;
        // println!("l0, r0, l1, r1 = {:?}", (l0, r0, l1, r1));
        // println!("(rem0, rem1) = {:?}", (rem0, rem1));

        dfs(b, l0, r0, rem0, unbalance);
        dfs(b, l1, r1, rem1, unbalance);
    }
    let mut unbalance = 0;
    dfs(&mut b, 0, num - 1, rem, &mut unbalance);
    println!("{}", unbalance);
    println!("{}", b.iter().join(" "));

}