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
        n: usize
    }
    
    // O(N^(1.5))の計算量
    let mut ans = 0;
    for i in 1..n+1 {
        // i*iの素因数分解の結果が知りたいが、iの素因数分解の結果から各素数の指数を2倍すればいいだけ。
        // iの素因数分解はO(i^0.5)でできる。
        let p_list = prime_factorize(i);
        // println!("i={}, {:?}", i, p_list);
        dfs(&p_list, 0, &mut ans, 1, n, i*i);
    }

    // O(n^2)かかる解法
    // let mut ans = 0;
    // for i in 1..n+1 {
    //     for j in 1..n+1 {
    //         if i * i % j == 0 {
    //             if i*i/j > n {continue}
    //             // println!("i={}, i^2 = {} = {}, {}", i, i*i, j, i*i/j);
    //             ans += 1;
    //         }
    //     }
    // }
    println!("{}", ans);

}

fn dfs(p_list: &Vec<Vec<usize>>, depth: usize, ans: &mut usize, value: usize, n: usize, ii: usize) {
    if depth == p_list.len() {
        if ii / value < n + 1 && value < n + 1 {
            // println!("ii:{}, i:{} j:{}", ii, value, ii/value);
            *ans += 1;
        }
        return
    }
    for i in 0..2*p_list[depth][1]+1 {
        let next_value = value * p_list[depth][0].pow(i as u32);
        dfs(p_list, depth+1, ans, next_value, n, ii);
    }
}

// 素因数分解
fn prime_factorize(mut x: usize) -> Vec<Vec<usize>> {
    // let root_x = (x as f64).sqrt() as usize;
    let mut prime_num_list = vec![];
    let mut i = 1;
    while i * i <= x {
    // for i in 2..(root_x+1) {
        i += 1;
        let mut exponent = 0;
        while x % i == 0 {
            x /= i;
            exponent += 1;
        }
        if exponent != 0 {
            prime_num_list.push(vec![i, exponent]);
        }
    }
    if x != 1 {
        prime_num_list.push(vec![x, 1]);
    }
    return prime_num_list
}
