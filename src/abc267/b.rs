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

    let mut collumn_stand = vec![true; 7];

    if s[6] == '0' {
        collumn_stand[0] = false;
    }
    if s[3] == '0' {
        collumn_stand[1] = false;
    }
    if s[1] == '0' && s[7] == '0' {
        collumn_stand[2] = false;
    }
    if s[0] == '0' && s[4] == '0' {
        collumn_stand[3] = false;
    }
    if s[2] == '0' && s[8] == '0' {
        collumn_stand[4] = false;
    }
    if s[5] == '0' {
        collumn_stand[5] = false;
    }
    if s[9] == '0' {
        collumn_stand[6] = false;
    }


    let mut flag = false;
    if s[0] == '1' {
        println!("No");
    }
    else {
        for i in 0..7 {
            for j in (i+1)..7 {
                for k in (j+1)..7 {
                    if collumn_stand[i] && !collumn_stand[j] && collumn_stand[k] {
                        flag = true;
                    }
                }
            }
        }
        if flag {
            println!("Yes");
        }
        else {
            println!("No");
        }
    }
}