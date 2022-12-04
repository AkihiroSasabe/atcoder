use itertools::Itertools;
use proconio::input;
use proconio::marker::Chars;
use std::cmp::Ordering;
use std::cmp::{max, min};
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::f64::consts::PI;
use std::mem::swap;
use superslice::*;
fn main() {
    input! {
        H: usize,
        W: usize,
        N: usize,
        h: usize,
        w: usize,
        a: [[usize; W]; H]
    }
    // マスクがかかっていない状態で、マスに書かれている数値をハッシュマップに格納
    let mut hash_map = HashMap::new();
    for i in 0..H {
        for j in 0..W {
            if hash_map.contains_key(&a[i][j]) {
                *hash_map.get_mut(&a[i][j]).unwrap() += 1;
            } else {
                hash_map.insert(a[i][j], 1);
            }
        }
    }
    // (k, l) = (0, 0)のときにマスクされたマスの数値をハッシュマップから除去
    for i in 0..h {
        for j in 0..w {
            if hash_map[&a[i][j]] == 1 {
                hash_map.remove(&a[i][j]);
            } else {
                *hash_map.get_mut(&a[i][j]).unwrap() -= 1;
            }
        }
    }

    for k in 0..(H - h + 1) {
        let mut row = format!("");
        for l in 0..(W - w + 1) {
            if k == 0 && l == 0 {
                row = format!("{}",hash_map.len());
            } 
            else {
                // 下にスライド
                if (k % 2 == 1 && l == 0) || (k % 2 == 0 && l == 0) {
                    for j in 0..w {
                        let mut x = j;
                        if k % 2 == 1 {
                            x = W - w + j;
                        }
                        let inner = a[k-1][x];
                        let outer = a[k+h-1][x];
                        in_out_hashmap(&mut hash_map, inner, outer);                        
                    }
                    row = format!("{}",hash_map.len());
                }
                // 左右にスライド
                else {
                    for i in k..k + h {
                        // 右にスライド
                        if k % 2 == 0 {
                            let inner = a[i][l-1];
                            let outer = a[i][l+w-1];
                            in_out_hashmap(&mut hash_map, inner, outer);
                        }
                        // 左にスライド
                        else {
                            let inner = a[i][W-l];
                            let outer = a[i][W-w-l];
                            in_out_hashmap(&mut hash_map, inner, outer);
                        }
                    }
                    if k % 2 == 0 {
                        row = format!("{} {}",row, hash_map.len());
                    }
                    else {
                        row = format!("{} {}",hash_map.len(), row);
                    }
                }
            }
        }
        println!("{}", row);
    }
}

fn in_out_hashmap(hash_map: &mut HashMap<usize, usize>, inner: usize, outer: usize) {
    if hash_map.contains_key(&inner) {
        *hash_map.get_mut(&inner).unwrap() += 1;
    } else {
        hash_map.insert(inner, 1);
    }
    if hash_map[&outer] == 1 {
        hash_map.remove(&outer);
    } else {
        *hash_map.get_mut(&outer).unwrap() -= 1;
    }
}