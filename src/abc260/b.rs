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
        n: usize,
        x: usize,
        y: usize,
        z: usize,
        a: [isize; n],
        b: [isize; n],
    }
    let mut gokaku = vec![false; n];
    let mut sugaku = vec![];
    let mut eigo = vec![];
    let mut both = vec![];
    
    for i in 0..n {
        sugaku.push(vec![a[i], (n - i) as isize]);
        eigo.push(vec![b[i], (n - i) as isize]);
        both.push(vec![a[i] + b[i],  (n - i) as isize]);
    }
    sugaku.sort();
    eigo.sort();
    both.sort();

    sugaku.reverse();
    eigo.reverse();
    both.reverse();

    for i in 0..n {
        sugaku[i][1] = (sugaku[i][1] - n as isize) * (-1);
        eigo[i][1] = (eigo[i][1] - n as isize) * (-1);
        both[i][1] = (both[i][1] - n as isize) * (-1);
    }

    // println!("{:?}", sugaku);
    // println!("{:?}", eigo);
    // println!("{:?}", both);


    for i in 0..x {
        gokaku[sugaku[i][1] as usize] = true;
    }
    let mut count = 0;
    for i in 0..n {
        if count == y {break}
        if gokaku[eigo[i][1] as usize] {continue}
        gokaku[eigo[i][1] as usize] = true;
        count += 1;
    }

    let mut count = 0;
    for i in 0..n {
        if count == z {break}
        if gokaku[both[i][1] as usize] {continue}
        gokaku[both[i][1] as usize] = true;
        count += 1;
    }
    
    for i in 0..n {
        if gokaku[i] {
            println!("{}", i+1);
        }
    }

}