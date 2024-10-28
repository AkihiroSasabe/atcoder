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
use rand::Rng;
fn main() {
    // 2024-10-03 20:50-?
    // 2024-10-22 
    // 2024-10-28 20:08-20:25 (17min, 解説見た)
    // 参考 https://drken1215.hatenablog.com/entry/2024/10/17/035413
    // (x,y)について、実験すると、「|Y−X|≥1」 で先手必勝なことに気が付く。
    // ゲームの「勝ちパターン」の証明：
    //      [1] 勝ちパターンである局面からは、上手い手を打つことで、負けパターンにできる
    //      [2] 負けパターンである局面から、どの手を打っても、負けパターンではなくなる
    // これは、Nimでも出てきた証明方法。
    // 
    // 今回は、勝ちパターンは「|Y−X|>=1」で過不足ないと考えているので、それを示そう。
    // [1]は、2i取れば、差をiだけ縮めることができるので、|Y−X|<1 の負けパターンに持ち込める
    // [2]は、|Y−X|<1 の状態から、2iとっても、差は広がらないので、|Y−X|>=1 の勝ちパターンに持っていけない。
    input! {
        x: usize,
        y: usize,
    }

    // let is_win = brute_force(x, y);
    let nx = min(x, y);
    let ny = max(x, y);
    if ny - nx > 1 {
        println!("Alice");
    }
    else {
        println!("Brown");
    }
    return;
    
    // 2i個の石を取る。
    // i個捨てる。i個反対側の山に置く。
    // nimっぽい。
    
    // 終状態
    // (1,1) or (1,0) or (0,1)
    // (2,1) -> (0,2) -> (1,0) Bobの勝ち

    // ただの再帰でいけるか? -> REする

    // 実験
    let h = 15;
    let w = 15;
    // img[y][x] := (x, y)だったときの勝者
    let mut img = vec![vec!['A'; w]; h];
    for x in 0..w {
        for y in 0..h {
            print!("{} {} ", x, y);
            let is_win = brute_force(x, y);

            if !is_win {
                img[y][x] = 'B';
            }
        }
    }
    // img[y][x] := (x, y)だったときの勝者
    // img.print_2d_array();
    // ↓Y軸, X軸->
    // 'B' 'B' 'A' 'A' 'A' 'A' 'A' 'A' 'A' 'A' 'A' 'A' 'A' 'A' 'A' 
    // 'B' 'B' 'B' 'A' 'A' 'A' 'A' 'A' 'A' 'A' 'A' 'A' 'A' 'A' 'A' 
    // 'A' 'B' 'B' 'B' 'A' 'A' 'A' 'A' 'A' 'A' 'A' 'A' 'A' 'A' 'A' 
    // 'A' 'A' 'B' 'B' 'B' 'A' 'A' 'A' 'A' 'A' 'A' 'A' 'A' 'A' 'A' 
    // 'A' 'A' 'A' 'B' 'B' 'B' 'A' 'A' 'A' 'A' 'A' 'A' 'A' 'A' 'A' 
    // 'A' 'A' 'A' 'A' 'B' 'B' 'B' 'A' 'A' 'A' 'A' 'A' 'A' 'A' 'A' 
    // 'A' 'A' 'A' 'A' 'A' 'B' 'B' 'B' 'A' 'A' 'A' 'A' 'A' 'A' 'A' 
    // 'A' 'A' 'A' 'A' 'A' 'A' 'B' 'B' 'B' 'A' 'A' 'A' 'A' 'A' 'A' 
    // 'A' 'A' 'A' 'A' 'A' 'A' 'A' 'B' 'B' 'B' 'A' 'A' 'A' 'A' 'A' 
    // 'A' 'A' 'A' 'A' 'A' 'A' 'A' 'A' 'B' 'B' 'B' 'A' 'A' 'A' 'A' 
    // 'A' 'A' 'A' 'A' 'A' 'A' 'A' 'A' 'A' 'B' 'B' 'B' 'A' 'A' 'A' 
    // 'A' 'A' 'A' 'A' 'A' 'A' 'A' 'A' 'A' 'A' 'B' 'B' 'B' 'A' 'A' 
    // 'A' 'A' 'A' 'A' 'A' 'A' 'A' 'A' 'A' 'A' 'A' 'B' 'B' 'B' 'A' 
    // 'A' 'A' 'A' 'A' 'A' 'A' 'A' 'A' 'A' 'A' 'A' 'A' 'B' 'B' 'B' 
    // 'A' 'A' 'A' 'A' 'A' 'A' 'A' 'A' 'A' 'A' 'A' 'A' 'A' 'B' 'B' 
}

fn brute_force(x: usize, y: usize) -> bool {
    // 愚直解
    let mut set = HashMap::new();
    let nx = min(x, y);
    let ny = max(x, y);
    // set.insert((nx, ny), false);
    let is_win =dfs(nx, ny, &mut set);
    if is_win {
        println!("Alice");
    }
    else {
        println!("Brown");
    }
    return is_win
}

// デバッグ用に2次元配列をprintするトレイト
pub trait Print2DArray {
    fn print_2d_array(&self);
    fn print_2d_array_with_name(&self, name: &str);
    fn print_2d_array_transposed(&self);
    fn print_2d_array_transposed_with_name(&self, name: &str);
}
impl<T: std::fmt::Debug> Print2DArray for Vec<Vec<T>> {
    fn print_2d_array(&self) {
        for y in 0..self.len() {
            for x in 0..self[y].len() {
                print!("{:?} ", self[y][x]);
            }
            println!("");
        }
    }
    fn print_2d_array_with_name(&self, name: &str) {
        println!("-=-=-=-= {} -=-=-=-=", name);
        self.print_2d_array();
        println!("-=-=-=-=-=-=-=-=");
    }
    fn print_2d_array_transposed(&self) {
        for x in 0..self[0].len() {
            print!("{:02}: ", x);
            for y in 0..self.len() {
                print!("{:?} ", self[y][x]);
            }
            println!("");
        }
    }
    fn print_2d_array_transposed_with_name(&self, name: &str) {
        println!("-=-=-=-= transposed {} -=-=-=-=", name);
        self.print_2d_array_transposed();
        println!("-=-=-=-=-=-=-=-=");
    }
}

fn dfs(x: usize, y: usize, set: &mut HashMap<(usize, usize), bool>) -> bool {
    if x < 2 && y < 2 {
        return false
    }

    let key = (min(x,y), max(x,y));
    if set.contains_key(&key) {
        let res = *set.get(&key).unwrap();
        return res
    }

    // x から削る
    let mut i = 1;
    while 2 * i <= x {
        let xx = x - 2 * i;
        let yy = y + i;

        let nx = min(xx, yy);
        let ny = max(xx, yy);
        let is_win = !dfs(xx, yy, set);
        if is_win {
            set.insert(key, is_win);
            return true
        }
        i += 1;
    }

    // y から削る
    let mut i = 1;
    while 2 * i <= y {
        let xx = x + i;
        let yy = y - 2 * i;

        let nx = min(xx, yy);
        let ny = max(xx, yy);
        let is_win = !dfs(xx, yy, set);
        if is_win {
            set.insert(key, is_win);
            return true
        }
        i += 1;
    }

    set.insert(key, false);
    return false

}