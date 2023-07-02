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
use std::collections::HashSet;
use proconio::marker::Chars;
use std::f64::consts::PI;
use std::mem::swap;
use superslice::*;
fn main() {
    input! {
        ha: usize,
        wa: usize,
        a: [Chars; ha],
        hb: usize,
        wb: usize,
        b: [Chars; hb],
        hx: usize,
        wx: usize,
        x: [Chars; hx],
    }

    // aから余白部分をカットして圧縮する
    let mut ay_max = 0;
    let mut ax_max = 0;
    let mut ay_min = ha-1;
    let mut ax_min = wa-1;
    for i in 0..ha {
        for j in 0..wa {
            if a[i][j] == '#' {
                ay_max = max(ay_max, i);
                ax_max = max(ax_max, j);
                ay_min = min(ay_min, i);
                ax_min = min(ax_min, j);
            }
        }
    }
    let mut new_a = vec![];
    for i in ay_min..ay_max+1 {
        let mut row = vec![];
        for j in ax_min..ax_max+1 {
            row.push(a[i][j]);
        }
        new_a.push(row);
    }

    // bから余白部分をカットして圧縮する
    let mut by_max = 0;
    let mut bx_max = 0;
    let mut by_min = ha-1;
    let mut bx_min = wa-1;
    for i in 0..hb {
        for j in 0..wb {
            if b[i][j] == '#' {
                by_max = max(by_max, i);
                bx_max = max(bx_max, j);
                by_min = min(by_min, i);
                bx_min = min(bx_min, j);
            }
        }
    }

    let mut new_b = vec![];
    for i in by_min..by_max+1 {
        let mut row = vec![];
        for j in bx_min..bx_max+1 {
            row.push(b[i][j]);
        }
        new_b.push(row);
    }

    // xから余白部分をカットして圧縮する
    let mut xy_max = 0;
    let mut xx_max = 0;
    let mut xy_min = ha-1;
    let mut xx_min = wa-1;
    for i in 0..hx {
        for j in 0..wx {
            if x[i][j] == '#' {
                xy_max = max(xy_max, i);
                xx_max = max(xx_max, j);
                xy_min = min(xy_min, i);
                xx_min = min(xx_min, j);
            }
        }
    }

    let mut new_x = vec![];
    for i in xy_min..xy_max+1 {
        let mut row = vec![];
        for j in xx_min..xx_max+1 {
            row.push(x[i][j]);
        }
        new_x.push(row);
    }

    // 余白削除後のxより、aやbの方が大きければその時点でNo
    if new_x.len() < new_a.len() || new_x[0].len() < new_a[0].len() || new_x.len() < new_b.len() || new_x[0].len() < new_b[0].len() {
        println!("No");
        return
    }

    // 全体の計算量: O(10^6)
    // aとbの左上の位置を、x内で走査する(O(10^4))
    for ay_corner in 0..1+new_x.len()-new_a.len() {
        for ax_corner in 0..1+new_x[0].len()-new_a[0].len() {
            for by_corner in 0..1+new_x.len()-new_b.len() {
                for bx_corner in 0..1+new_x[0].len()-new_b[0].len() {

                    // x内のマスを全探索して、条件を満たすか確認(O(10^2))
                    let mut flag = true;
                    for i in 0..new_x.len() {
                        if !flag {break}
                        for j in 0..new_x[0].len() {
                            if new_x[i][j] == '#' {
                                if ay_corner <= i && i - ay_corner < new_a.len() && ax_corner <= j && j - ax_corner < new_a[0].len() && new_a[i - ay_corner][j - ax_corner] == '#' {
                                    continue
                                }
                                if by_corner <= i && i - by_corner < new_b.len() && bx_corner <= j && j - bx_corner < new_b[0].len() && new_b[i - by_corner][j - bx_corner] == '#' {
                                    continue
                                }
                                flag = false;
                                break
                            }
                            else {
                                if ay_corner <= i && i - ay_corner < new_a.len() && ax_corner <= j && j - ax_corner < new_a[0].len() && new_a[i - ay_corner][j - ax_corner] == '#' {
                                    flag = false;
                                    break
                                }
                                if by_corner <= i && i - by_corner < new_b.len() && bx_corner <= j && j - bx_corner < new_b[0].len() && new_b[i - by_corner][j - bx_corner] == '#' {
                                    flag = false;
                                    break
                                }
                            }
                        }
                    }
                    if flag {
                        println!("Yes");
                        return
                    }
                }
            }
        }
    }

    println!("No");
    return
}