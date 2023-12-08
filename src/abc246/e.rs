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
    // 2023-12-07 21:54-22:16 (22min)
    // 2023-12-07 22:26-22:36 (10min)
    // 2023-12-07 23:06-23:34 (28min) debug コンパニオン使った
    // total: 60min
    input! {
        n: usize,
        mut ax: usize,
        mut ay: usize,
        mut bx: usize,
        mut by: usize,
        s: [Chars; n]
    }
    ax -= 1;
    ay -= 1;
    bx -= 1;
    by -= 1;

    // const INF: usize = 1 << 63;
    const INF: usize = 99;
    let mut dist = vec![vec![INF; n]; n];
    dist[ax][ay] = 0;
    let mut queue = VecDeque::new();

    // 左下
    // 右下
    // 左上
    // 右上
    let mut dirs = vec![[-1, 1], [1, 1], [-1, -1], [1, -1]];
    queue.push_back([ax, ay]);
    while queue.len() > 0 {
        let v = queue.pop_front().unwrap();
        let x = v[0];
        let y = v[1];

        for i in 0..dirs.len() {
            let dx = dirs[i][0];
            let dy = dirs[i][1];
            let mut count = 0;
            loop {
                count += 1;
                let nx = x as isize + dx * count;
                let ny = y as isize + dy * count;
                // println!("(x, y, nx, ny), ({}, {}, {}, {})", x, y, nx, ny);

                // 場外
                if nx < 0 || n as isize <= nx {break}
                if ny < 0 || n as isize <= ny {break}
                let nx = nx as usize;
                let ny = ny as usize;
                
                // 白のボーンがあったらストップ
                if s[nx][ny] == '#' {break}
                
                // 既に通ったときはスルー
                if dist[nx][ny] == dist[x][y] + 1 {continue} // スキップ <- なんでこれが無いと通らないのか分からない。2023-12-08 test_016 で確認するのが良いと。思う。正解は3だけど、自分は5で回答してしまっている。
                if dist[nx][ny] != INF {break} // 打ち切り

                dist[nx][ny] = dist[x][y] + 1;
                // println!("(x, y, nx, ny), ({}, {}, {}, {}), dist[x][y]={}, dist[nx][ny]={}", x, y, nx, ny, dist[x][y], dist[nx][ny]);
                queue.push_back([nx, ny]);
            }
            // return;
        }
    }
    // s.print_2d_array();
    // dist.print_2d_array();

    if dist[bx][by] == INF {
        println!("-1");
    }
    else {
        println!("{}", dist[bx][by]);
    }

}

fn bfs(v0: usize, graph: &Vec<Vec<usize>>) -> Vec<usize> {
    // v0が始点
    let init_distance: usize = 1_000_000_000_000_000_000; // 10^18
    let mut queue = VecDeque::new();
    let mut distance = vec![init_distance; graph.len()];
    distance[v0] = 0;
    queue.push_back(v0);
    while queue.len() > 0 {
        let v = queue.pop_front().unwrap();
        for i in 0..graph[v].len() {
            let nv = graph[v][i];
            if distance[nv] != init_distance {continue}
            distance[nv] = distance[v] + 1;
            queue.push_back(nv);
        }
    }
    return distance
}


// デバッグ用に2次元配列をprintするトレイト
pub trait Print2DArray {
    fn print_2d_array(&self);
    fn print_2d_array_with_name(&self, name: &str);
    fn print_2d_array_transposed(&self);
    fn print_2d_array_transposed_with_name(&self, name: &str);
}
impl<T: std::fmt::Display> Print2DArray for Vec<Vec<T>> {
    fn print_2d_array(&self) {
        for y in 0..self.len() {
            for x in 0..self[y].len() {
                print!("{:2} ", self[y][x]);
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
                print!("{} ", self[y][x]);
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