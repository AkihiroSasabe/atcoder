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

// テストケース: 01_small_01.txt
// 2
// -1 1
// x x
// o x
// ! 1 2

// テストケース: 
// xox
// xxo
// xxx
// ! 1 3

fn read() -> usize {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    s.trim().parse().unwrap()
}

fn main() {
    // https://atcoder.jp/contests/abc244/editorial/3625 の解説
    // インタラクティブな問題でproconioを使うときは下記2行が必須。
    // let stdin = stdin();
    // let mut source = LineSource::new(BufReader::new(stdin.lock()));
    // さらにinput!する変数名の上の行に from &mut source, も必要。
    // (例)
    // input! {
    //     from &mut source,
    //     n: usize
    // }

    // proconio (doc) を使っているユーザが多いと思われますが、これは --release とそうでないときで挙動が異なります。 
    // 具体的には、--release のときは「入力を一通り読んでから処理を行う」のに対し、そうでないときは一行ずつ読んで処理を行います。
    // また、println!の後に下記でflashもする必要がある。
    // stdout().flush().unwrap();
    // => めんどくさすぎる。後ろにflash入れるのは、忘れてミスる原因になりかねない。
    // 【結論】素直にproconioを使わずにstd::io::stdin().read_line()を使うべし。
    
    let n = read();

    // yの場所を特定する
    // めぐる式2分探索: ok以上の座標に、空きマスがある
    let mut ok = 1;
    let mut ng = n+1;
    while ng > ok + 1 {
        let mid = (ok + ng) / 2;
        println!("? {} {} {} {}", mid, n, 1, n);
        let t = read();
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
        let t = read();        
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
    println!("! {} {}", y, x);
}