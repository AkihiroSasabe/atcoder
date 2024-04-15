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
    input! {
        s: Chars,
        t: Chars,
    }

    // let INF = 1 << 60;
    // let mut first = INF;
    // let mut seconf = INF;
    // let mut third = INF;
    let mut firsts = vec![];
    let mut seconds = vec![];
    let mut thirds = vec![];
    for i in 0..s.len() {
        if (s[i] as usize - 'a' as usize) == (t[0] as usize - 'A' as usize) {
            firsts.push(i);
        }
        if (s[i] as usize - 'a' as usize) == (t[1] as usize - 'A' as usize) {
            seconds.push(i);
        }
        if (s[i] as usize - 'a' as usize) == (t[2] as usize - 'A' as usize) {
            thirds.push(i);
        }
    }
    if t[2] == 'X' {
        if firsts.len() == 0 || seconds.len() == 0 {
            println!("No");
            return
        }
        else {
            if t[0] == t[1] && t[0] == t[2] {
                if firsts.len() < 3 {
                    println!("No");
                    return
                }
                else {
                    println!("Yes");
                    return
                }
            }
            if t[0] == t[1] {
                if firsts.len() <= 1 {
                    println!("No");
                    return
                }
                else {
                    if firsts[1] < thirds[thirds.len()-1] {
                        println!("Yes");
                        return
                    }
                    else {
                        println!("No");
                        return
                    }
                    return
                }
            }
            if firsts[0] < seconds[seconds.len() - 1] {
                println!("Yes");
                return
            }
            else {
                println!("No");
                return
            }
        }
    }

    if firsts.len() == 0 || seconds.len() == 0 || thirds.len() == 0 {
        println!("No");
        return
    }
    if t[0] == t[1] && t[0] != t[2]{
        if firsts.len() <= 1 {
            println!("No");
            return
        } 
        else {
            if firsts[1] < thirds[thirds.len() - 1] {
                println!("Yes");
                return
            }
            else {
                println!("No");
                return
            }
        }
    }
    else if t[0] == t[2] && t[0] != t[1]{
        if firsts.len() <= 1 {
            println!("No");
            return
        }
        for i in seconds {
            if firsts[0] < i && i < thirds[thirds.len() - 1] {
                println!("Yes");
                return
            }
        }
        println!("No");
        return
    }
    else if t[1] == t[2] && t[0] != t[2] {
        if seconds.len() <= 1 {
            println!("No");
            return
        }
        if firsts[0] < seconds[seconds.len()-2] {
            println!("Yes");
            return
        }
        else {
            println!("No");
            return
        }
    }
    else if t[0] == t[1] && t[1] == t[2] {
        if firsts.len() <= 2 {
            println!("No");
            return
        }
        else {
            println!("No");
            return
        }
    }
    if firsts[0] > thirds[thirds.len() - 1] {
        println!("No");
        return
    }
    else {
        for i in seconds {
            if firsts[0] < i && i < thirds[thirds.len() - 1] {
                println!("Yes");
                return
            }
        }
        println!("No");
        return
    
    }




}