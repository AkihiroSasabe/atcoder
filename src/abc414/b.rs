#![allow(dead_code, unused_imports)]
use proconio::{input, marker::{Usize1, Isize1, Chars}};
use itertools::Itertools;
use std::cmp::{max, min, Ordering, Reverse};
use std::ops::Bound::{Excluded, Included, Unbounded};
use std::collections::{VecDeque, BinaryHeap, HashMap, BTreeMap, HashSet, BTreeSet};
use std::f64::consts::PI;
use std::mem::swap;
use superslice::*;
use rand::Rng;
use num::{BigUint, ToPrimitive, Integer};
use num_bigint::ToBigUint;
fn main() {

    let aaa = 1000000000000000000;
    let ans: usize = aaa * 18 + 446744073709551716;
    println!("{}", ans);

    let strs = vec![
        '1','2','3','4','5','6','7','8','9',
        ':',';','<','=','>','?','@',
        'A','B','C','D','E','F','G','H','I','J','K','L','M','N','O','P','Q','R','S','T','U','V','W','X','Y','Z',
        '[','\\',']','^','_','`',
        'a','b','c','d','e','f','g','h','i','j','k','l','m','n','o','p','q','r','s','t','u','v','w','x','y','z','{','|','}','~',
        '¡','¢','£','¤','¥','¦','§','¨','©','ª','«','¬','­','®','¯','°','±','²','³','´','µ','¶','·','¸','¹','º','»','¼','½','¾','¿','À','Á','Â','Ã','Ä','Å','Æ','Ç','È','É','Ê','Ë','Ì','Í','Î','Ï','Ð','Ñ','Ò','Ó','Ô','Õ','Ö','×','Ø','Ù','Ú','Û','Ü','Ý','Þ','ß','à','á','â','ã','ä','å','æ','ç','è','é','ê','ë','ì','í','î','ï','ð','ñ','ò','ó','ô','õ','ö','÷','ø','ù','ú','û','ü','ý','þ'
    ];
    // println!("{}", strs.len());
    // println!("100");
    println!("19");
    for i in 0..19 {
    // for i in 0..100 {
        // println!("{} {}", ('!' as u8 + i as u8) as char, 1_000_000_000_000_000_000);
        // println!("{} {}", (i as u8) as char, 1_000_000_000_000_000_000);
        // print!("'{}',", (i as u8) as char);
        println!("{} {}", strs[i], 1_000_000_000_000_000_000);
        // println!("{i}, {} {}", strs[i], 1_000_000_000_000_000_000);
        // println!("{} {}", i, 1_000_000_000_000_000_000);
    }
    return;
    // 7766279631452241920
    // 18446744073709551615
    // 18_446_744_073_709_551_615

    input! {
        n: usize,
    }
    let mut c = vec![];
    let mut l = vec![];
    let mut len = 0;
    for i in 0..n {
        input!{
            ci: char,
            li: usize,
        }
        l.push(li);
        len += li;
        if len > 100 {
            println!("Too Long");
            return;
        }
        c.push(ci);
    }
    
    for i in 0..n {
        let ci = c[i];
        let li = l[i];
        for j in 0..li {
            print!("{}", ci);
        }
    }
    println!("")
}