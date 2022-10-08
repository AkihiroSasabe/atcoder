use proconio::input;
use itertools::Itertools;
use std::cmp::{max, min};
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use proconio::marker::Chars;
use std::f64::consts::PI;
fn main() {
    input! {
        h1: usize,
        h2: usize,
        h3: usize,
        w1: usize,
        w2: usize,
        w3: usize,
    }

    // 122_023_936 = 10^8
    let mut count = 0;
    let mut masu = vec![vec![0;3]; 3];
    for s1a in 0..h1-1 {
        for s1b in (s1a+1)..h1-1 {
            for s2a in 0..h2-1 {
                for s2b in (s2a+1)..h2-1 {
                    for s3a in 0..h3-1 {
                        for s3b in (s3a+1)..h3-1 {
                            masu[0][0] = s1a + 1;
                            masu[0][1] = s1b - s1a;
                            masu[0][2] = h1 - 1 -s1b;

                            masu[1][0] = s2a + 1;
                            masu[1][1] = s2b - s2a;
                            masu[1][2] = h2 - 1 -s2b;

                            masu[2][0] = s3a + 1;
                            masu[2][1] = s3b - s3a;
                            masu[2][2] = h3 - 1 -s3b;

                            if masu[0][0] + masu[1][0] + masu[2][0] == w1 && masu[0][1] + masu[1][1] + masu[2][1] == w2 && masu[0][2] + masu[1][2] + masu[2][2] == w3 {
                                count += 1;
                            }
                        }
                    }
                }
            }
        } 
    }
    println!("{}", count);
}