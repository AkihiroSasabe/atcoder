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
    

    let mut answer = 0;
    for i in 1..(n+1){
        let mut count = 0 
        // i^2を素因数分解O(n^(1/2))
        let p_list = splecial_prime_factrize(i.clone());
        for p in p_list {
            p[0].pow(p[1] as u32)
        }
        answer += count;
    }
    println!("{}", answer);
}

fn prime_factrize(mut n: usize) {
    let root = (n as f64).sqrt() as usize;
    let mut prime_factrized_list = vec![];
    // 素因数分解
    for j in 2..root {
        if n % j !=0 {continue}
        let mut ex = 0;

        while n % j == 0 {
            ex += 1;
            n /= j;
        }
        prime_factrized_list.push(vec![j, ex]);
    }
    if n != 1 {
        prime_factrized_list.push(vec![n, 1]);
    }
    println!("number: {}, prime_factrized_list: {:?}", root*root, prime_factrized_list);
}

// n^2について素因数分解する
fn splecial_prime_factrize(mut n: usize) {
    let root = (n as f64).sqrt() as usize;
    let mut prime_factrized_list = vec![];
    // 素因数分解
    for j in 2..root {
        if n % j !=0 {continue}
        let mut ex = 0;

        while n % j == 0 {
            ex += 1;
            n /= j;
        }
        prime_factrized_list.push(vec![j, ex*2]);
    }
    if n != 1 {
        prime_factrized_list.push(vec![n, 2]);
    }
    println!("number: {}, prime_factrized_list: {:?}", root*root, prime_factrized_list);


}