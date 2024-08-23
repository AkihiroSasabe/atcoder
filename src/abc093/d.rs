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
    // 2024-08-21 20:36-21:00 (24min)
    // 2024-08-22 19:43-21:00 (1h17min)
    // 1h41min
    input! {
        q: usize
    }
    let mut a = vec![];
    let mut b = vec![];
    for i in 0..q {
        input!{
            ai: usize,
            bi: usize,
        }
        a.push(ai);
        b.push(bi);
    }

    // ai位, bi位

    // (x位, y位)
    // x*y
    // (x+a, y-a)
    // (x+a)*(y-a) = xy - ax + ay - a^2
    // x*y - (x+a)*(y-a) = ax - ay + a^2 > 0
    // ax - ay + a^2 > 0 の条件は?
    // a(x-y) + a^2 > 0
    // (x-y) + a > 0
    // x>y, a>0 より、いかなる a でも成立

    // (x-a, y+a)
    // (x-a) * (y+a) = xy + a(x-y) - a^2
    // - a(x-y) + a^2 > 0 
    // a^2 > a(x-y)
    // a > (x-y)
    

    for i in 0..q {
        let mut ans: usize = 0;
        let ai = a[i];
        let bi = b[i];

        let x = max(ai, bi);
        let y = min(ai, bi);

        // [1] (x+a, y-a) のパターン
        // いかなる a でもok
        let cont1 = y - 1;
        ans += cont1;

        // [2] (x-a, y+a) のパターン
        // a > (x-y) となる a から許される(a=6,7,8,...)
        // これって結局、(x+a, y-a)と同じパターン
        let cont2 = y - 1;
        ans += cont2;

        // [3] (x-a, y+b) のパターン (x-a > y + b かつ a <= x-y)
        // めぐる式二分探索
        // x-a > y+b を満たす、最大の y+b を知りたい。(下の具体例の考察参考。) 
        let mut ng = x;
        let mut ok = y;
        if judge(ng, x, y) {
            ok = ng;
        }
        while (ng as i128 - ok as i128).abs() > 1 {
            let mid = (ng + ok) / 2;
            let is_ok = judge(mid, x, y);
            if is_ok {
                ok = mid;
            }
            else {
                ng = mid;
            }
        }

        let cont3 = 1 + ok - y;
        let opp = if x*y % ok != 0 {
            x*y / ok
        } else {
            x*y / ok - 1
        };

        if x != y {
            if opp != ok {
                ans += cont3 * 2;
            }
            else {
                if cont3 != 0 {
                    ans += cont3 * 2 - 1;
                }
            }
            // 同じ順位のやつは使えない
            ans -= 1;
        }


        // println!("---------- i = {:?}", i);
        // println!("x, y = {}, {:?}", x, y);
        // println!("ok = {:?}", ok);
        // println!("opp = {:?}", opp);
        // println!("cont1 = {:?}", cont1);
        // println!("cont3 = {:?}", cont3);
        println!("{}", ans);
    }

    // 10, 9 = 90 (diff = 1)
    // 11, 8 = 88
    // 12, 7 = 84
    // 13, 6 = 78

    // ★サンプル i = 2 (ai, bi) = (10, 5)
    // [1](x+a, y-a) のパターン
    // 4通り
    // (10, 5) = 50
    // ~~~~~~~~~~~~~~~
    // (11, 4) = 44
    // (12, 3) = 36
    // (13, 2) = 26
    // (14, 1) = 28 y - 4

    // [2](x-a, y+a) のパターン a > x - y で、確実に xy > (x-a)(y-b)
    // 4 + 通り
    // (10, 5) = 50, a=0
    // ~~~~~~~~~~~~~~~~~~~~~~~
    // (9, 6) = 54, a=1
    // (8, 7) = 56, a=2
    // (7, 8) = 56, a=3
    // (6, 9) = 54, a=4 < x - y
    // (5, 10) = 50, a=5 == x-y -----------
    // (4, 11) = 44, a=6 > x-y
    // (3, 12) = 36
    // (2, 13) = 26
    // (1, 14) = 14

    // ==== ==== ====
    // (9, 5) <= 同じ順位のやつは使えない (yが5位)
    // (8, 6)
    // (7, 7) ~~~~
    // (6, 8)
    // (5, 9)


    // ★サンプル i = 3 (ai, bi) = (3, 3)
    // [1](x+a, y-a) のパターン
    // (3, 3) = 9
    // ~~~~~~~~~~~~
    // (3+1, 3-1) = (4,2)=8
    // (3+2, 3-2) = (5,1) = 5

    // [2](x-a, y+a) のパターン a > x - y で、確実に xy > (x-a)(y-b)
    // (3, 3) = 9
    // ~~~~~~~~~~~~
    // (3-1, 3+1) = (2,4)=8
    // (3-2, 3+2) = (1,5) = 5

    // ★サンプル (自分オリジナル) (ai, bi) = (1002, 2)
    // [2](x-a, y+a) のパターン a > x - y で、確実に xy > (x-a)(y-b)
    // (1002, 2) = 2004, a=0
    // ~~~~~~~~~~~~~~~~~~~~~~~
    // (1001, 3) = 3003, a=1, a < x-y = 1000
    // (1000, 4) = 4000, a=2, a < x-y = 1000
    // ( 999, 5) = 4995, a=3, a < x-y = 1000
    // ...
    // (   3, 1001) = 3003, a= 999, a < x-y = 1000
    // (   2, 1002) = 2004, a=1000, a = x-y = 1000 -----------------
    // (   1, 1003) = 1003, a=1001, a > x-y = 1000

    // [3](x-a, y-b) のパターン a <= x - y のケース で、 xy > (x-a)(y-b) なものを探す
    // x-a と y-b の差が最も小さいy-bを探す。このときのy-bが最大のy-bである。
    // (1002, 2) = 2004, a=0
    // ~~~~~~~~~~~~~~~~~~~~~~~
    // (667,  3) = 2001
    // (500,  4) = 2000
    // (400,  5) = 2000
    // (333,  6) = 1998
    // ...
    // ( 66, 30) = 1980
    // ...
    // ( 47, 42) = 1974
    // ( 46, 43) = 1978
    // ( 45, 44) = 1980 <- x-a と y-b の差(45-44)が最も小さい。 3~44 は、積が 2004 以下になる相手が確実にいる

    
}

fn judge(mid: usize, x: usize, y: usize) -> bool {
    let xy = x * y;
    let opp = if xy % mid != 0 {
        xy / mid
    } else {
        xy / mid - 1
    };

    return opp >= mid
}
