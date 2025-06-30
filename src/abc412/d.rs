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

    // 2025-06-28 21:37-22:37 (1h)
    // 2025-06-28 22:37-23:00 (23min)
    // Total: 1h23min
    input! {
        n: usize,
        m: usize,
    }

    // ◆解法
    // 
    // N=8なら、
    // 7! + 8C3*(2!*4!) + 8C4*(3!*3!) 
    // = 5040 + 56*38 + 70*36 
    // = 5040 + 2128 + 2520
    // = 9688
    // 9688通り調べるだけでよい。
    // 1個のグラフの構築には、N*(N-1)log(N-1)<=8*7*2=56=112 の計算量がいるので、
    // 9688*112 <= 10^3*10^2 <= 10^5 くらいで全列挙できそう。

    let mut a = vec![];
    let mut b = vec![];
    // let mut g = vec![vec![]; n];
    let mut g: Vec<BTreeSet<usize>> = vec![BTreeSet::new(); n];
    for i in 0..m {
        input!{
            ai: Usize1,
            bi: Usize1,
        }
        a.push(ai);
        b.push(bi);
        g[ai].insert(bi);
        g[bi].insert(ai);
        // g[ai].push(bi);
        // g[bi].push(ai);
    }

    // 7!= 5040
    // n^2=64
    let mut ans: usize = 1 << 60;

    for mut perm in (1..n).permutations(n-1) {
        // println!("perm = {:?}", perm);

        let mut g2 = g.clone();
        let mut cand = 0;

        perm.push(0);
        let mut pv = perm[perm.len()-1];
        for i in 0..perm.len() {
            let v = perm[i];
            let nv = perm[(i+1)%n];

            let mut deletes = vec![];
            let mut has_nv = false;
            let mut has_pv = false;
            for &vvv in g2[v].iter() {
                if vvv != nv && vvv != pv {
                    deletes.push(vvv);
                }
                if vvv == nv {
                    has_nv = true;
                }
                if vvv == pv {
                    has_pv = true;
                }
            }
            cand += deletes.len();

            for d in deletes {
                g2[v].remove(&d);
                g2[d].remove(&v);
            }

            if !has_nv {
                cand += 1;
                g2[v].insert(nv);
                g2[nv].insert(v);
            }
            if !has_pv {
                cand += 1;
                g2[v].insert(pv);
                g2[pv].insert(v);
            }
            pv = v;
        }
        ans = min(ans, cand);
    }


    if n == 6 {
        // 3-3に分ける
        for comb in (0..n).combinations(3) {
            solve(n, comb, &g, &mut ans);
        }
    }
    else if n == 7 {
        // 3-4に分ける
        for comb in (0..n).combinations(3) {
            solve(n, comb, &g, &mut ans);
        }
    }
    else if n == 8 {
        // 4-4に分ける
        // 3-5に分ける
        for comb in (0..n).combinations(3) {
            solve(n, comb, &g, &mut ans);
        }
        for comb in (0..n).combinations(4) {
            solve(n, comb, &g, &mut ans);
        }
    }


    println!("{}", ans);

}

fn solve(n: usize, comb: Vec<usize>, g: &Vec<BTreeSet<usize>>, ans: &mut usize) {
    let mut other = vec![];
    for i in 0..n {
        let mut is_con = false;
        for &c in comb.iter() {
            if c == i {
                is_con = true;
            }
        }
        if !is_con {
            other.push(i);
        }
    }
    // println!("----");
    // println!("comb = {:?}", comb);
    // println!("other = {:?}", other);
    // println!("");


    for mut perm in (1..comb.len()).permutations(comb.len()-1) {
        perm.push(0);
        for mut perm2 in (1..other.len()).permutations(other.len()-1) {

            let mut g2 = g.clone();
            let mut cand = 0;

            perm2.push(0);

            // println!("perm = {:?}", perm);
            // println!("perm2 = {:?}", perm2);

            let mut pv = other[perm2[perm2.len()-1]];
            for i in 0..perm2.len() {
                let v = other[perm2[i]];
                let nv = other[perm2[(i+1)% perm2.len()]];

                let mut deletes = vec![];
                let mut has_nv = false;
                let mut has_pv = false;
                for &vvv in g2[v].iter() {
                    if vvv != nv && vvv != pv {
                        deletes.push(vvv);
                    }
                    if vvv == nv {
                        has_nv = true;
                    }
                    if vvv == pv {
                        has_pv = true;
                    }
                }
                // println!("v = {v}, deletes = {:?}", deletes);
                cand += deletes.len();

                for d in deletes {
                    g2[v].remove(&d);
                    g2[d].remove(&v);
                }

                if !has_nv {
                    cand += 1;
                    g2[v].insert(nv);
                    g2[nv].insert(v);
                }
                if !has_pv {
                    cand += 1;
                    g2[v].insert(pv);
                    g2[pv].insert(v);
                }
                pv = v;
            }

            let mut pv = comb[perm[perm.len()-1]];
            for i in 0..perm.len() {
                let v = comb[perm[i]];
                let nv = comb[perm[(i+1)%perm.len()]];

                let mut deletes = vec![];
                let mut has_nv = false;
                let mut has_pv = false;
                for &vvv in g2[v].iter() {
                    if vvv != nv && vvv != pv {
                        deletes.push(vvv);
                    }
                    if vvv == nv {
                        has_nv = true;
                    }
                    if vvv == pv {
                        has_pv = true;
                    }
                }
                // println!("v = {v}, deletes = {:?}", deletes);
                cand += deletes.len();

                for d in deletes {
                    g2[v].remove(&d);
                    g2[d].remove(&v);
                }

                if !has_nv {
                    cand += 1;
                    g2[v].insert(nv);
                    g2[nv].insert(v);
                }
                if !has_pv {
                    cand += 1;
                    g2[v].insert(pv);
                    g2[pv].insert(v);
                }
                pv = v;
            }

            // println!("cand = {:?}", cand);
            *ans = min(*ans, cand);
        }
    }

}