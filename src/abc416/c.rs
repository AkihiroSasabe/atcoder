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
    // 2025-07-26 21:14-21:23 (9min)
    // 2025-07-26 21:23-21:34 (11min)
    // Total: 20min
    input! {
        n: usize,
        k: usize,
        x: usize,
        s: [Chars; n]
    }

    // solve_on_contest(n, k, x, s);
    solve_after_contest(n, k, x, s);
}

fn solve_after_contest(n: usize, k: usize, x: usize, s: Vec<Vec<char>>) {
    // 真面目にDFSで解く。
    // fn dfs(n: usize, k: usize, s: &Vec<Vec<char>>, stack: &mut Vec<usize>, alls: &mut Vec<Vec<Vec<char>>>) {
    fn dfs(n: usize, k: usize, s: &Vec<Vec<char>>, stack: &mut Vec<usize>, alls: &mut Vec<Vec<char>>) {
        if stack.len() == k {
            let mut temp = vec![];
            // for &i in stack.iter() {
            //     temp.push(s[i].clone());
            // }
            for &i in stack.iter() {
                for j in 0..s[i].len() {
                    temp.push(s[i][j]);
                }
            }
            alls.push(temp);
            return;
        }

        for i in 0..n {
            stack.push(i);
            dfs(n, k, s, stack, alls);
            stack.pop();
        }
    }
    let mut stack = vec![];
    let mut alls = vec![];
    dfs(n, k, &s, &mut stack, &mut alls);
    alls.sort();
    let ans = alls[x-1].clone();
    println!("{}", ans.iter().collect::<String>());

    // for ai in ans {
    //     print!("{}", ai.iter().collect::<String>());
    // }

}

fn solve_on_contest(n: usize, k: usize, x: usize, s: Vec<Vec<char>>) {
    let mut all = vec![];

    if k == 5 {
        for i0 in 0..n {
            for i1 in 0..n {
                for i2 in 0..n {
                    for i3 in 0..n {
                        for i4 in 0..n {
                            let mut temp = vec![];
                            let mut aaa = vec![s[i0].clone(), s[i1].clone(), s[i2].clone(), s[i3].clone(), s[i4].clone()];
                            for ai in aaa {
                                for aii in ai {
                                    temp.push(aii);
                                }
                            }
                            all.push(temp);
                        }
                    }
                }
            }
        }
    }
    else if k == 4 {
        for i0 in 0..n {
            for i1 in 0..n {
                for i2 in 0..n {
                    for i3 in 0..n {
                        let mut temp = vec![];
                        let mut aaa = vec![s[i0].clone(), s[i1].clone(), s[i2].clone(), s[i3].clone()];
                        for ai in aaa {
                            for aii in ai {
                                temp.push(aii);
                            }
                        }
                        all.push(temp);
                    }
                }
            }
        }
    }
    else if k == 3 {
        for i0 in 0..n {
            for i1 in 0..n {
                for i2 in 0..n {
                    let mut temp = vec![];
                    let mut aaa = vec![s[i0].clone(), s[i1].clone(), s[i2].clone()];
                    for ai in aaa {
                        for aii in ai {
                            temp.push(aii);
                        }
                    }
                    all.push(temp);
                }
            }
        }
    }
    else if k == 2 {
        for i0 in 0..n {
            for i1 in 0..n {
                let mut temp = vec![];
                let mut aaa = vec![s[i0].clone(), s[i1].clone()];
                for ai in aaa {
                    for aii in ai {
                        temp.push(aii);
                    }
                }
                all.push(temp);
            }
        }
    }
    else if k == 1 {
        for i0 in 0..n {
            let mut temp = vec![];
            let mut aaa = vec![s[i0].clone()];
            for ai in aaa {
                for aii in ai {
                    temp.push(aii);
                }
            }
            all.push(temp);
        }
    }
    all.sort();
    let ans = all[x-1].clone();
    println!("{}", ans.iter().collect::<String>());

}