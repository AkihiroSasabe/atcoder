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
        n: usize,
        s: Chars,
        mut w: [usize; n]
    }

    let mut child = vec![];
    let mut adult = vec![];
    for i in 0..n {
        if s[i] == '0' {
            child.push(w[i]);
        }
        else {
            adult.push(w[i]);
        }
    }
    child.sort();
    adult.sort();
    w.sort();
    w.push(w[w.len()-1] + 1);

    let mut ans = 0;
    let child_num = child.len();
    let adult_num = adult.len();
    for i in 0..(n+1) {
        let c_num = child.lower_bound(&w[i]);
        let a_num = adult_num - adult.lower_bound(&w[i]);
        // println!("i, c_num, a_num, w[i], child.lower_bound(&w[i], adult.lower_bound(&w[i]): {} {} {} {} {} {}", i, c_num, a_num, w[i], child.lower_bound(&w[i]), adult.lower_bound(&w[i]));
        ans = max(ans, c_num + a_num);
    }
    println!("{}", ans);

}




// lower_bound=Key★以★上★のインデックス、
// upper_bound=Key★よ★り★大きいインデックス
// sorted_list.lower_bound(&x)は、x以上となる最小のインデックスを返すが、x超えがリスト内に無いときは、sorted_list.len()を返すので注意
/// Equivalent to std::lowerbound and std::upperbound in c++
pub trait BinarySearch<T> {
    fn lower_bound(&self, x: &T) -> usize;
    fn upper_bound(&self, x: &T) -> usize;
}

impl<T: Ord> BinarySearch<T> for [T] {
    fn lower_bound(&self, x: &T) -> usize {
        let mut low = 0;
        let mut high = self.len();

        while low != high {
            let mid = (low + high) / 2;
            match self[mid].cmp(x) {
                Ordering::Less => {
                    low = mid + 1;
                }
                Ordering::Equal | Ordering::Greater => {
                    high = mid;
                }
            }
        }
        low
    }

    fn upper_bound(&self, x: &T) -> usize {
        let mut low = 0;
        let mut high = self.len();

        while low != high {
            let mid = (low + high) / 2;
            match self[mid].cmp(x) {
                Ordering::Less | Ordering::Equal => {
                    low = mid + 1;
                }
                Ordering::Greater => {
                    high = mid;
                }
            }
        }
        low
    }
}
