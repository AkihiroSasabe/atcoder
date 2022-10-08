use proconio::input;
use itertools::Itertools;
use std::cmp::{max, min};
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use proconio::marker::Chars;
use std::f64::consts::PI;
use std::mem::swap;
fn main() {
    input! {
        s: Chars
    }

    // // swapの練習
    // let mut a = 1;
    // let mut b = 2;
    // swap(&mut a, &mut b);
    // println!("a {}", a);
    // println!("b {}", b);

    let mut a: Vec<char> = "atcoder".chars().collect();
    // key: 文字、value: 文字のあるべき位置
    let mut hash_map = HashMap::new();
    for i in 0..a.len() {
        hash_map.insert(a[i], i);
    }

    // 文字列Sの配置順
    let mut s_list = vec![];
    for i in 0..s.len() {
        s_list.push(hash_map[&s[i]]);
        // s_list.push(hash_map[&s[i]] as isize);
    }
    // println!("{}", hash_map[&'t']);     // 1
    // println!("{:?}", s_list);           // [2, 0, 1, 6, 5, 4, 3]

    
    let mut ans = 0;
    loop {
        let mut swap_flag = false;
        for i in 0..(s_list.len()-1) {
            if s_list[i] > s_list[i+1] {
                s_list.swap(i, i+1);
                swap_flag = true;
                ans += 1;
            }
        }
        if !swap_flag {
            break
        }
    }
    println!("{}", ans);
}
