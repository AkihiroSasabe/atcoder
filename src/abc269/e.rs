// use proconio::input;
use proconio::{input, source::line::LineSource};
use itertools::Itertools;
use std::cmp::{max, min};
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use proconio::marker::Chars;
use std::f64::consts::PI;
use std::mem::swap;
use superslice::*;
use std::io::{stdin, stdout, BufReader, Write};



xx
ox

// 
xox
xxo
xxx
!1 3



fn main() {
    // https://atcoder.jp/contests/abc244/editorial/3625 の解説
    // インタラクティブな問題では下記2行が必須。
    // proconio (doc) を使っているユーザが多いと思われますが、これは --release とそうでないときで挙動が異なります。 
    // 具体的には、--release のときは「入力を一通り読んでから処理を行う」のに対し、そうでないときは一行ずつ読んで処理を行います。
    // また、println!の後にflashもする必要がある。
    
    let stdin = stdin();
    let mut source = LineSource::new(BufReader::new(stdin.lock()));

    // debug
    input! {
        from &mut source,
        n: usize
    }
    // input! {
    //     from &mut source,
    //     matrix: [[char; n]; n]
    // }
    // let xy = get_xy(&matrix, n);

    // yの場所を特定する
    // めぐる式2分探索: ok以上の座標に、空きマスがある
    let mut ok = 1;
    let mut ng = n+1;
    while ng > ok + 1 {
        let mid = (ok + ng) / 2;
        println!("? {} {} {} {}", mid, n, 1, n);
        stdout().flush().unwrap();
        input! {
            from &mut source,
            t: usize
        }
        // let t = get_num_by_y(&xy, mid, n);
        // println!("ok:{} ng:{} mid:{}", ok, ng, mid);
        // 空きマスがないとき
        if (n + 1) - mid == t {
            // println!("No empyty cell between {}-{}", mid, n);
            ng = mid;
        }
        // 空きマスがあるとき
        else {
            // println!("Empyty cell exists between {}-{}", mid, n);
            ok = mid;
        }
        // println!("ok:{} ng:{} mid:{}", ok, ng, mid);
    }
    let y = ok;
    // println!("y: {}", y);

    // xの場所を特定する
    let mut ok = 1;
    let mut ng = n+1;
    while ng > ok + 1 {
        let mid = (ok + ng) / 2;
        println!("? {} {} {} {}", 1, n, mid, n);
        stdout().flush().unwrap();
        input! {
            from &mut source,
            t: usize
        }
        // let t = get_num_by_x(&xy, mid, n);
        // println!("ok:{} ng:{} mid:{}", ok, ng, mid);
        // 空きマスがないとき
        if (n + 1) - mid == t {
            // println!("No empyty cell between {}-{}", mid, n);
            ng = mid;
        }
        // 空きマスがあるとき
        else {
            // println!("Empyty cell exists between {}-{}", mid, n);
            ok = mid;
        }
        // println!("ok:{} ng:{} mid:{}", ok, ng, mid);
    }
    let x = ok;
    println!("! {} {}", x, y);
    stdout().flush().unwrap();
    // if x == xy[0] && y == xy[1] {
    //     println!("ok!!");
    // }
    // else {
    //     println!("bug!! =======================");
    // }

}

fn get_xy(matrix: &Vec<Vec<char>>, n: usize) -> Vec<usize> {
    let mut flag = false;
    let mut x = n;
    let mut y = n;
    for i in 0..n {
        for j in 0..n {
            if matrix[i][j] == '_' {
                y = i;
                x = j;
                flag = true;
                break
            }
        }
        if flag {
            break
        }
    }
    return vec![x+1, y+1]
}

fn get_num_by_x(xy: &Vec<usize>, xs: usize, xe: usize) -> usize {
    let x = xy[0];
    let mut ans = xe - xs + 1;
    if xs <= x && x <= xe {
        ans -= 1;
    }
    return ans
}

fn get_num_by_y(xy: &Vec<usize>, ys: usize, ye: usize) -> usize {
    let y = xy[1];
    let mut ans = ye - ys + 1;
    if ys <= y && y <= ye {
        ans -= 1;
    }
    return ans
}