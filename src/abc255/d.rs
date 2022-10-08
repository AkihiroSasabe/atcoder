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
        q: usize,
        mut a: [usize; n],
        x: [usize; q]
    }

    a.sort();
    let mut diff_sum_from_min = vec![0; n];
    let mut diff_sum_from_max = vec![0; n];
    for i in 1..n {
        let diff_from_min = a[i] - a[0];
        let diff_from_max = a[n-1] - a[n-1-i];
        diff_sum_from_min[i] = diff_sum_from_min[i-1] + diff_from_min;
        diff_sum_from_max[i] = diff_sum_from_max[i-1] + diff_from_max;
    }

    for i in 0..q {
        // x_iが数列aの最小値より小さいとき
        if x[i] <= a[0] {
            println!("{}", (a[0] - x[i]) * n + diff_sum_from_min[n-1]);
        }
        // x_iが数列aの最大値より大きいとき
        else if a[n-1] <= x[i] {
            println!("{}", (x[i] - a[n-1]) * n + diff_sum_from_max[n-1]);
        }
        // x_iが数列aの最小値～最大値に収まっているとき
        else {
            // x_iより大きなaの項のうち、最小のindexを求める
            let index = a.lower_bound(&x[i]);
            let mut answer = (x[i] - a[0]) * index - diff_sum_from_min[index-1];
            answer += (a[n-1] - x[i]) * (n - index) - diff_sum_from_max[n-1-index];
            println!("{}", answer);
        }
    }
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
