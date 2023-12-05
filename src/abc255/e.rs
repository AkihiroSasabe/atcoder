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
    // 2023-12-05 14:48-15:22 (34min)
    input! {
        n: usize,
        m: usize,
        s: [isize; n-1],
        x: [isize; m],
    }

    // mがすくないな。最大10個しかない。
    // a[0] を決めると、自動的に、残りのa[1:n]が決定される。
    // a[0] =  a[0]
    // a[1] = -a[0] + s[0]
    // a[2] =  a[0] + s[1] - s[0]
    // a[3] = -a[0] + s[2] - s[1] + s[0]
    // a[n-1] = (-1)^(n-1) * a[0] + s[n-2] - s[n-3] + ... + (-1)^(n-1) * s[0]
    // よって、取り合えずsの累積和っぽいものを求めればいい

    // sの累積和っぽいもの
    let mut cum_s = vec![0; n-1];
    cum_s[0] = s[0];
    for i in 1..n-1 {
        if i % 2 != 0 {
            cum_s[i] = cum_s[i-1] - s[i];
        }
        else {
            cum_s[i] = cum_s[i-1] + s[i];
        }
    }
    // println!("cum_s = {:?}", cum_s);

    for i in 0..n-1 {
        if i % 2 == 1 {
            cum_s[i] = - cum_s[i];
        }
    }
    // println!("cum_s = {:?}", cum_s);

    // HashMap<a[0]の値, Aがラッキーナンバーとなる個数>
    let mut hash: HashMap<isize, usize> = HashMap::new();
    for i in 0..m {
        *hash.entry(x[i]).or_insert(0) += 1;
        for j in 0..n-1 {
            if j % 2 == 0 {
                *hash.entry(-(x[i] - cum_s[j])).or_insert(0) += 1;
            }
            else {
                *hash.entry(x[i] - cum_s[j]).or_insert(0) += 1;
            }
        }
    }

    let mut ans = 0;
    for (k, v) in hash {
        ans = max(ans, v);
    }
    println!("{}", ans);


}