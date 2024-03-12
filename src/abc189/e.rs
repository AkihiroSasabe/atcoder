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
    // 2024-03-10 15:30-15:45 (15m)
    // 2024-03-11 12:10-12:45 (35m)
    // 2024-03-11 20:04-21:33 (1h29m)
    // 2h19min
    input! {
        n: usize
    }

    // 初期位置(x,y)が、i回直後に位置(Xi,Yi)に居るとすると、
    // Xi = Ai x + Bi y + Ci
    // Yi = Di x + Ei y + Fi
    // とおけることに気がつく。(アフィン変換)
    // (A0, B0, C0) = (1,0,0)
    // (F0, E0, F0) = (0,1,0)
    // 例えば、(3)の操作で、
    // Xi+1 = p - (Xi-p)
    //      = - Xi + 2p
    //      = - (Ai x + Bi y + Ci) + 2p
    //      = - Ai x - Bi y - Ci + 2p
    // Yi+1 = Yi 
    //      = Di x + Ei y + Fi
    // となるから、
    // Ai+1 = - Ai
    // Bi+1 = - Bi
    // Ci+1 = - Ci + 2p
    // Di+1 = Di
    // Ei+1 = Ei
    // Fi+1 = Fi
    // となる。
    // よって、各回の累積変換を、事前に前処理で求めることができる。

    let mut x = vec![];
    let mut y = vec![];
    for i in 0..n {
        input!{
            xi: isize,
            yi: isize,
        }
        x.push(xi);
        y.push(yi);
    }
    input! {
        m: usize
    }
    let mut k = vec![];
    let mut p = vec![];
    for i in 0..m {
        input!{
            opi: isize,
        }
        k.push(opi);
        if opi == 3 || opi == 4 {
            input! {
                pi: isize
            }
            p.push(pi);
        }
        else {
            p.push(0);
        }
    }


    let mut a = vec![];
    let mut b = vec![];
    input! {
        q: usize
    }
    for i in 0..q {
        input!{
            ai: isize,
            bi: usize,
        }
        a.push(ai);
        b.push(bi-1);
    }

    let mut transforms = vec![];
    let mut aa = 1;
    let mut bb = 0;
    let mut cc = 0;
    let mut dd = 0;
    let mut ee = 1;
    let mut ff = 0;
    for i in 0..m {
        match k[i] {
            1 => (aa, bb, cc, dd, ee, ff) = transform_1(aa, bb, cc, dd, ee, ff),
            2 => (aa, bb, cc, dd, ee, ff) = transform_2(aa, bb, cc, dd, ee, ff),
            3 => (aa, bb, cc, dd, ee, ff) = transform_3(aa, bb, cc, dd, ee, ff, p[i]),
            4 => (aa, bb, cc, dd, ee, ff) = transform_4(aa, bb, cc, dd, ee, ff, p[i]),
            _ => ()
        }
        transforms.push((aa, bb, cc, dd, ee, ff));
    }
    // println!("transforms = {:?}", transforms);


    for i in 0..q {
        let ai = a[i];
        let bi = b[i];
        if ai == 0 {
            println!("{} {}", x[bi], y[bi]);
            continue
        }

        let (aa, bb, cc, dd, ee, ff) = transforms[ai as usize - 1].clone();
        let xx = aa * x[bi] + bb * y[bi] + cc;
        let yy = dd * x[bi] + ee * y[bi] + ff;
        // Ai​個目の操作を行った直後に、駒Bi​がある座標を出力
        println!("{} {}", xx, yy);
    }

}

fn transform_for_symmetry(a: isize, b: isize, c: isize, p: isize)  -> (isize, isize, isize) {
    // X = Ax + By + C で表されるとき、
    // X = p で変換したとき、
    // NX = -X + 2*p 
    //    = -A*x -B*y - C + 2*p

    let na = - a;
    let nb = - b;
    let nc = - c + 2*p;

    return (na, nb, nc)

}

fn transform_for_rotation(a: isize, b: isize, c: isize, d: isize, e: isize, f: isize, is_left_rot: bool) -> (isize, isize, isize, isize, isize, isize) {
    let na = -d;
    let nb = -e;
    let nc = -f;
    let nd = a;
    let ne = b;
    let nf = c;

    if is_left_rot {
        return (na, nb, nc, nd, ne, nf)
    }
    else {
        return (-na, -nb, -nc, -nd, -ne, -nf)
    }
}

fn transform_1(a: isize, b: isize, c: isize, d: isize, e: isize, f: isize) -> (isize, isize, isize, isize, isize, isize) {
    // 原点を中心に時計回りに 90 度回転
    let (na, nb, nc, nd, ne, nf) = transform_for_rotation(a, b, c, d, e, f, false);
    
    return (na, nb, nc, nd, ne, nf)
}

fn transform_2(a: isize, b: isize, c: isize, d: isize, e: isize, f: isize) -> (isize, isize, isize, isize, isize, isize) {
    let (na, nb, nc, nd, ne, nf) = transform_for_rotation(a, b, c, d, e, f, true);
    
    return (na, nb, nc, nd, ne, nf)
}

fn transform_3(a: isize, b: isize, c: isize, d: isize, e: isize, f: isize, p: isize) -> (isize, isize, isize, isize, isize, isize) {
    let (na, nb, nc) = transform_for_symmetry(a, b, c, p);
    let nd = d;
    let ne = e;
    let nf = f;
    
    return (na, nb, nc, nd, ne, nf)
}

fn transform_4(a: isize, b: isize, c: isize, d: isize, e: isize, f: isize, p: isize) -> (isize, isize, isize, isize, isize, isize) {
    let na = a;
    let nb = b;
    let nc = c;
    let (nd, ne, nf) = transform_for_symmetry(d, e, f, p);

    return (na, nb, nc, nd, ne, nf)
}
