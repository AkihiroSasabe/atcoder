use proconio::input;
use itertools::Itertools;
use superslice::Ext;
use std::cmp::{max, min};
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::collections::BinaryHeap;
use std::collections::{HashMap, HashSet};
use proconio::marker::Chars;
use std::f64::consts::PI;
fn main() {
    // 2023-12-04 16:16-17:49 (1h33min) 自力AC
    input! {
        n: usize,
        q: usize,
        x: usize,
        w: [usize; n],
        mut k: [usize; q]
    }
    for i in 0..q {
        k[i] -= 1;
    }

    // じゃがの個数を答える

    // 重さの累積和
    let mut cum_w = vec![0; n];
    cum_w[0] = w[0];
    for i in 1..n {
        cum_w[i] = w[i] + cum_w[i-1];
    }
    // println!("cum_w = {:?}", cum_w);

    // そのじゃがいもから始めたら、特定の個数が箱に入る
    let mut num_list = vec![0; n];

    // 周回分必要な　じゃが芋　の個数
    let num_cycle = n * (x / cum_w[n-1]);
    num_list[0] = num_cycle;
    if x % cum_w[n-1] != 0 {
        num_list[0] += 1 + cum_w.lower_bound(&(x % cum_w[n-1]));
    }    

    for i in 1..n {
        num_list[i] = num_cycle;
        let mut nokori_x = x % cum_w[n-1];
        if nokori_x > cum_w[n-1] - cum_w[i-1] {
            nokori_x -= cum_w[n-1] - cum_w[i-1];
            num_list[i] += n - i;
            num_list[i] += 1 + cum_w.lower_bound(&nokori_x);
        }
        else {
            let ind = cum_w.lower_bound(&(nokori_x + cum_w[i-1]));
            let num = 1 + ind - i;
            num_list[i] += num;
        }
    }
    // println!("num_list = {:?}", num_list);

    // <位置, 箱の番号>
    let mut hash: HashMap<usize, usize> = HashMap::new();
    let mut now: usize = 0; // 箱を詰め始める位置
    let mut period = 0; // 同じ箱を詰め始める位置に、繰り返し到達するのにかかる周期
    let mut count = 0; // 詰める箱のindex
    let mut box_pos_list = vec![]; // box_pos_list[i] := 箱iの位置
    loop {
        if hash.contains_key(&now) {
            // 同じ箱にぶち当たる
            period = count - hash[&now];
            box_pos_list.push(now);
            break
        }
        else {
            hash.insert(now, count);
            box_pos_list.push(now);
            now = (now + num_list[now]) % n;
        }
        count += 1;
    }
    // println!("box_pos_list = {:?}", box_pos_list);
    // println!("period = {:?}", period);
    let init_box = hash[&now];
    // println!("init_box = {:?}", init_box);

    for i in 0..q {
        if k[i] < box_pos_list.len() {
            let box_pos = box_pos_list[k[i]];
            let num = num_list[box_pos];
            println!("{}", num);
            // println!("---{}", box_pos);
        }
        else {
            // 
            let more_box_num = k[i] - init_box;
            let box_pos = box_pos_list[more_box_num % period + init_box];
            let num = num_list[box_pos];
            println!("{}", num);
        }
    }

}