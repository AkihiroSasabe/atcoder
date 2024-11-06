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
    // 2024-07-20 21:29-22:40 (1h11min)
    // 2024-07-21 11:00-12:09 (1h9min)
    // 2h20min
    input! {
        n: usize
    }
    

    // 法則性を考えると、解ける。実装に落とし込むのが大変だった。
    // 数: 順位
    // 0: 1
    // 1: 2
    // ...
    // 9: 10
    // 11: 11
    // 22: 12
    // ...
    // 99: 19
    // 101: 20
    // 111: 21
    // 121: 22
    // ...
    // 191: 29
    // 202: 30
    // 212: 31
    // ...
    // 292: 39
    // 303: 40
    // 313: 41
    // 323: 42
    // ...
    // 393: 49
    // ...
    // 404: 50
    // ...
    // 505: 60
    // ...
    // 909: 100
    // ...
    // 999: 109
    // 1001: 110
    // ...
    // 9999: 199
    // 10001: 200
    // 10901: 209
    // 11011: 210

    let mut ketas: Vec<usize> = vec![0; 36];
    let mut cums: Vec<usize> = vec![0; 36];
    let mut cums0 = vec![0; 36];
    let mut cums1 = vec![0; 36];
    ketas[1] = 10;
    // ketas[2] = 10; // 00 を含む
    ketas[2] = 9; // 00 を含まない
    ketas[3] = 9 * 10;
    ketas[4] = 9 * 10;

    cums1[1] = 10;
    cums0[2] = 10; // 00 を含む
    cums1[3] = ketas[3] + cums1[1];
    cums0[4] = ketas[4] + cums0[2];

    for i in 5..ketas.len() {
        if i % 2 == 0 {
            ketas[i] = 9 * cums0[i-2]; // 00の分
            cums0[i] = ketas[i] + cums0[i-2];
        }
        else {
            ketas[i] = 9 * cums1[i-2]; // 00の分
            cums1[i] = ketas[i] + cums1[i-2];
        }
        // println!("i = {:?}, ketas[i] = {}", i, ketas[i]);
    }

    cums[1] = ketas[1];
    for i in 2..cums.len() {
        // println!("i = {:?}", i);
        cums[i] = ketas[i] + cums[i-1];
    }
    
    // println!("ketas = {:?}", ketas);
    // println!("cums = {:?}", cums);

    let mut keta = 0;
    for i in 1..ketas.len() {
        if n <= cums[i] {
            keta = i;
            break
        }
    }
    // println!("keta = {:?}", keta);

    let mut ans = vec![];

    if keta == 1 {
        println!("{}", n-1);
        return
    }
    else if keta == 2 {
        let num = n % 10;
        ans.push(num);
    }
    else  {
        let aaa = if keta % 2 == 0 {
            cums0[keta - 2]
        } else {
            cums1[keta - 2]    
        };
        let num = (n - cums[keta-1] - 1) / aaa + 1;
        ans.push(num);
        // println!("num = {:?}", num);
        let n2 = (n - cums[keta-1] - 1) % aaa + 1;
        saiki(n2, keta - 2, &mut ketas, &mut cums, &mut cums0, &mut cums1, &mut ans);
    }
    // println!("ans = {:?}", ans);

    let mut temp = ans.clone();
    if keta %2 == 1 {
        temp.pop();
    }
    temp.reverse();
    for t in temp {
        ans.push(t);
    }

    for i in ans {
        print!("{}", i);
    }
    // 1_000_000_000_000_000_000;
    // 90_000_000_000_000_000_000_000_000_000_000_009
    // 206_380_123_533_373_789 <- 順位
    // 11 * 3 + 2 = 33 + 2 = 35 
}

fn saiki(n: usize, keta: usize, ketas: &mut Vec<usize>, cums: &mut Vec<usize>, cums0: &mut Vec<usize>, cums1: &mut Vec<usize>, ans: &mut Vec<usize>) {
    // println!("~~~~ keta = {:?} ~~~~", keta);
    // println!("n = {:?}", n);
    if keta == 1 {
        let num = n - 1;
        ans.push(num);
        // println!("num = {:?}", num);
        return
    }
    if keta == 2 {
        let num = (n - 1) % 10;
        ans.push(num);
        // println!("num = {:?}", num);
        return
    }

    let aaa = if keta % 2 == 0 {
        cums0[keta - 2]
    } else {
        cums1[keta - 2]    
    };
    // println!("aaa = {:?}", aaa);

    // 2位のやつ...
    if n <= aaa {
        let num = 0;
        // println!("num = {:?}", num);

        ans.push(num);
        let n2 = n;
        saiki(n2, keta-2, ketas, cums, cums0, cums1, ans);
        return
    }
    let num = (n - 1) / aaa;
    ans.push(num);
    // println!("num = {:?}", num);

    // 次の順位
    let n2 = (n - 1) % aaa + 1;
    saiki(n2, keta-2, ketas, cums, cums0, cums1, ans);
}