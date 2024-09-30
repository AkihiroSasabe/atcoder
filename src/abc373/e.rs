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
    // 2024-09-28 21:18-22:39 (1h21min)
    input! {
        n: usize,
        m: usize,
        k: usize,
        a: [usize; n],
    }
    // let mut a2 = a.clone();
    let mut a2 = vec![];
    let mut sum = 0;
    for i in 0..n {
        a2.push((a[i], i));
        sum += a[i];
    }
    a2.sort();
    let remain = k - sum; // 残り

    let (ranked_array, sorted_array, original_to_rank) = rank_array(&a);
    let mut cum = vec![0; n];
    cum[0] = sorted_array[0];
    for i in 1..n {
        cum[i] = cum[i-1] + sorted_array[i];
    }
    for i in 0..n {
        let r = ranked_array[i];
        // 残りの表を全部、iに入れたときに、m位の奴に勝てるか?
        if sorted_array[n-1-(m-1)] > a[i] + remain {
            print!("-1 ");
            continue
        }

        // めぐる式二分探索
        let mut ng = 0;
        let mut ok = remain;
        if judge(ng, remain, &a, i, m, &ranked_array, &sorted_array, &cum) {
            ok = ng;
        }
        while (ng as i128 - ok as i128).abs() > 1 {
            let mid = (ng + ok) / 2;
            let is_ok = judge(mid, remain, &a, i, m, &ranked_array, &sorted_array, &cum);
            if is_ok {
                ok = mid;
            }
            else {
                ng = mid;
            }
        }
        print!("{} ", ok);
        
    }
}

// めぐる式二分探索
fn judge(mid: usize, remain: usize, a: &Vec<usize>, i: usize, m: usize, ranked_array: &Vec<usize>, sorted_array: &Vec<usize>, cum: &Vec<usize>) -> bool {
    let n = a.len();
    // 残りの表を全部、iに入れたときに、m位の奴に勝てるか?
    if sorted_array[n-1-(m-1)] > a[i] + mid {
        return false
    }

    // 他の人に配る票
    let r2 = remain - mid;

    // a[i] + mid 以下は、何人いるか?
    let ind = sorted_array.upper_bound(&(a[i] + mid)); // a[i] も含めていることに注意

    // a[i] + mid より上の人たち
    let num_up = n - ind;

    // 抜かれてはいけない人数
    // if m < num_up {
    //     // ライバルが少ないので木にしなくていい
    //     return true
    // }
    let num_rival = m - num_up;

    // a[i] の順位
    let rank = ranked_array[i];

    // 残りのライバルが、(a[i] + mid + 1)票を、獲得するのに、必要な票数
    let mut x = 0;
    // 残りのライバル達の合計枚数
    let goal = (a[i] + mid + 1) * num_rival;
    
    // 現在のライバル達の所得枚数
    let mut current = 0;
    if ind - num_rival <= rank {
        // ライバルが少ないので、木にしなくていい
        if ind < num_rival + 2 {return true}
        current = cum[ind-1] - cum[ind-num_rival-2] - a[i];
    }
    else {
        // ライバルが少ないので、木にしなくていい
        if ind < num_rival + 1 {return true}
        current = cum[ind-1] - cum[ind-num_rival-1] 
    }

    // x = goal - current;
    // 残りのライバルが、(a[i] + mid + 1)票を、獲得すると負け。
    // if x > r2 {
    if goal > r2 + current {
        return true
    }
    else {
        return false
    }


    // // 今M位には要る。
    // a[i] + mid
    // // a[i]
    // sorted_array[i]
    // return true
}

fn rank_array<T: Ord + std::hash::Hash + Clone + Copy>(array: &Vec<T>) -> (Vec<usize>, Vec<T>, HashMap<T, usize>) {
    // 配列を順位変換する関数 O(NlogN)
    // 要素の値を圧縮することを、目的として使うことを想定している。(座標圧縮)
    // Input: 
    //     array: 配列
    // Output: 
    //    ranked_array:     順位変換済み配列
    //    sorted_array:     ソート済みの配列(順位から元の値をマップさせる)
    //    original_to_rank: 元の値から順位を対応させるマップ
    // Example:
    // let array = vec![333, 111, 444, 111, 555, 999];
    // let (ranked_array, _sorted_array, _original_to_rank) = rank_array(&array);
    // assert_eq!(ranked_array, vec![2, 0, 3, 0, 4, 5]);

    // 配列のサイズ
    let n = array.len();

    // B木<数列中に登場する値, 頻度>
    let mut btree: BTreeMap<T, usize> = BTreeMap::new();
    for i in 0..n {
        *(btree.entry(array[i]).or_insert(0)) += 1;
    }

    // 昇順ソート済みの、順位変換済み配列
    let mut sorted_rank_array = vec![];
    let mut rank = 0;
    for (k, frequency) in btree {
        for j in 0..frequency {
            sorted_rank_array.push(rank);
        }
        rank += frequency; // sorted_rank_array = [0, 0, 2, 3, 4, 5], 
        // ここを1にすると、隙間なくなる。
        // rank += 1; //sorted_rank_array = [0, 0, 1, 2, 3, 4], 
    }
    // println!("sorted_rank_array = {:?}, ", sorted_rank_array);

    // 順位から元の値をマップさせる
    let mut sorted_array = (*array).clone();
    sorted_array.sort();

    // 元の値から順位を対応させるマップ
    let mut original_to_rank: HashMap<T, usize> = HashMap::new();
    for i in 0..n {
        original_to_rank.insert(sorted_array[i], sorted_rank_array[i]);
    }

    // 元の順序の、順位変換済み配列
    let mut ranked_array: Vec<usize> = vec![];
    for i in 0..n {
        ranked_array.push(*(original_to_rank.get(&array[i]).unwrap()));
    }

    return (ranked_array, sorted_array, original_to_rank)
}
