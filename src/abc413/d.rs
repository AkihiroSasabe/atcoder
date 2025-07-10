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

    // 2025-07-05 21:04-21:23 (19min)
    // 2025-07-05 21:36-21:55 (19min)
    // Total: 38min
    input! {
        t: usize,
    }
    let mut n = vec![];
    let mut a = vec![];
    for i in 0..t {
        input!{
            ni: usize,
            ai: [isize; ni],
        }
        n.push(ni);
        a.push(ai);
    }
    for i in 0..t {
        let ni = n[i];
        if n[i] == 2 {
            println!("Yes");
            continue;
        }
        let mut a2 = vec![];
        for j in 0..n[i] {
            a2.push((a[i][j].abs(), a[i][j], j));
        }
        a2.sort();
        if a2[0].0 == a2[1].0 {
            // r = -1 か 1
            let mut set = HashMap::new();
            for j in 0..ni {
                *set.entry(a[i][j]).or_insert(0) += 1;
            }
            if set.len() == 1 {
                println!("Yes");
            }
            else if set.len() == 2 {
                let mut nums: Vec<isize> = vec![];
                for (vv, num) in set {
                    nums.push(num);
                }
                if (nums[0] - nums[1]).abs() <= 1 {
                    println!("Yes");
                }
                else {
                    println!("No");
                }
            }
            else {
                println!("No");
            }
        }
        else {
            for j in 0..ni-2 {
                if a2[j].1 * a2[j+1].1 < 0 {
                    if a2[j+2].1 * a2[j+1].1 > 0 {
                        println!("No");
                        break;
                    }
                }
                else {
                    if a2[j+2].1 * a2[j+1].1 < 0 {
                        println!("No");
                        break;
                    }
                }
                if (a2[j].1 * a2[j+2].1 != a2[j+1].1 * a2[j+1].1) {
                    println!("No");
                    break;
                }
                if j == ni - 3 {
                    println!("Yes");
                }
            }
        }
    }

    // for i in 0..t {
    //     if n[i] == 2 {
    //         println!("Yes");
    //         continue;
    //     }

    //     let mut a2 = vec![];
    //     for j in 0..n[i] {
    //         a2.push((a[i][j].abs(), a[i][j], j));
    //     }
    //     a2.sort();

    //     let e0 = a2[0].1;
    //     let e1 = a2[1].1;
    //     if e0 == e1 {
    //         // 1 か -1
    //         let mut set = HashMap::new();
    //         for j in 0..n[i] {
    //             *set.entry(a[i][j]).or_insert(0) += 1;
    //         }
    //         if set.len() > 1 {
    //             println!("No");
    //         }
    //         else if set.len() == 1 {
    //             println!("Yes");
    //         }
    //         else {
    //             // 2
    //             let mut nums: Vec<isize> = vec![];
    //             for (vv, num) in set {
    //                 nums.push(num);
    //             }
    //             if (nums[0] - nums[1]).abs() <= 1 {
    //                 println!("Yes");
    //             }
    //             else {
    //                 println!("No");
    //             }
    //         }
    //     }
    //     else {
    //         let mut is_ok = true;
    //         for j in 0..n[i]-2 {
    //             if a2[j].1 * a2[j+2].1 != a2[j+1].1 * a2[j+1].1 {
    //                 is_ok = false;
    //                 break;
    //             }
    //         }
    //         if is_ok {
    //             println!("Yes");
    //         }
    //         else {
    //             println!("No");
    //         }
    //     }

    // }
}