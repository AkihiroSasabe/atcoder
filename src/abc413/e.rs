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
    // 2025-07-05 21:55-22:40 (45min)
    // 2025-07-06 0:04-??
    // 2025-07-08 20:58-21:23 (25min, ngtknの解説をみてupsolve。)
    // Total 70min
    input! {
        t: usize,
    }
    let mut ns = vec![];
    let mut ps = vec![];
    for i in 0..t {
        input!{
            ni: usize,
            pi: [usize; (1 << ni)],
        }
        ns.push(ni);
        ps.push(pi);
    }

    solve(t, ns, ps);
}

fn solve(t: usize, ns: Vec<usize>, ps: Vec<Vec<usize>>) {
    for qi in 0..t {
        let n = ns[qi];
        let mut p = ps[qi].clone();

        for b in (1..=n).rev() {
            for a in 0..1<<(n-b) {
                let st = a*(1<<b);
                let width = 1 << b;
                let en = st + width - 1;
                let mut l_min = 1 << 60;
                let mut r_min = 1 << 60;
                for i in st..st+width/2 {
                    l_min = min(l_min, p[i]);
                }
                for i in st+width/2..=en {
                    r_min = min(r_min, p[i]);
                }
                // 反転する必要がない
                if l_min < r_min {continue}

                // 反転する必要がある
                for (index, pos) in (st..st+width/2).enumerate() {
                    p.swap(pos, en - index);
                }
            }
        }
        for i in 0..(1<<n) {
            print!("{} ", p[i]);
        }
        println!("");
        // println!("{}", p.iter().map(|x| (x).to_string()).collect::<Vec<String>>().join(" "));
    }
}

fn solve_botu(n: Vec<usize>, ps: Vec<Vec<usize>>) {
    // let mut set = BTreeSet::new();
    // for b in 0..32 {
    //     set.insert(1 << b);
    // }

    // // 2^18 = 262_144 <= 3* 10^5

    // for i in 0..t {
    //     let ni = n[i];
    //     // 愚直
    //     let mut p = ps[i].clone();

    //     // 先頭から。
    //     for j in 0..(1<<ni) {
    //         if j % 2 == 1 {continue;}
    //         let mut max_b = 0;
    //         for b in 1..32 {
    //             if (1 << b) + j >= ni {break}
    //             max_b = b;
    //         }
    //         // max_b がわかれば、あとは自身より少ないやつを探すだけ
    //         let mut min_val = p[j];
    //         let mut min_ind = j;
    //         for k in j..j+(1<<max_b) {
    //             if p[k] < min_val {
    //                 min_val = p[k];
    //                 min_ind = k;
    //             }
    //         }
    //         if j == min_ind {continue;}
    //         // j と min_ind を入れ替えるにはどうすればいいか?
    //         let diff_ind = min_ind - j;
    //         let mut bit = diff_ind.leading_zeros();
    //         if (1 << bit) != diff_ind {
    //             bit += 1;
    //         }

    //     }

    // }


}