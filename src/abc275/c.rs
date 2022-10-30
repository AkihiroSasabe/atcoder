#![allow(dead_code, unused_imports)]
use proconio::input;
use itertools::Itertools;
use std::cmp::{max, min};
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;
use proconio::marker::Chars;
use std::f64::consts::PI;
use std::mem::swap;
use superslice::*;
fn main() {
    input! {
        s: [Chars; 9]
    }

    let mut ans = 0;
    for r in 0..9 {
        // println!("{:?}", s[r]);
        for c in 0..9 {
            if s[r][c] == '#' {

                for r2 in (r+1)..9 {
                    if s[r2][c] == '#' {
                        let c2 = c + r2 - r;
                        if c2 < 9 && s[r][c2] == '#' && s[r2][c2] == '#' {
                            ans += 1;
                        }
                    }
                }
                // 斜め
                for dy in 1..9 {
                    for dx in 1..9 {
                        let r2 = r + dy;
                        let c2 = c + dx;
                        if r2 < 9 && c2 < 9 {
                            if r + dy < dx {continue}
                            let r3 = r + dy - dx;
                            let c3 = c + dx + dy;
                            if r < dx {continue}
                            let r4 = r - dx;
                            let c4 = c + dy;
                            if r3 < 9 && r4 < 9 && c3 < 9 && c4 < 9 {
                                if s[r2][c2] == '#' && s[r3][c3] == '#' && s[r4][c4] == '#' {
                                    ans += 1;
                                }
                            }
                            
                        }
                    }
                }
            }
        }
    }
    println!("{}", ans);


}
