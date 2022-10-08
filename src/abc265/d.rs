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
        p: usize,
        q: usize,
        r: usize,
        a: [usize; n]
    }

    let mut a_cum = vec![0; n];
    a_cum[0] = a[0];
    for i in 1..n {
        a_cum[i] = a_cum[i-1] + a[i];
    }
    // println!("{:?}", a_cum);

    let mut flag = false;
    for x in 0..n {
        let mut key_for_x = 0;
        if x == 0 {
            key_for_x= p;
        }
        else {
            key_for_x = p + a_cum[x-1];
        }
        // println!("key_for_x: {}", key_for_x);
        let x_index = a_cum.lower_bound(&key_for_x);
        if x_index < x || x_index == n {continue}
        if a_cum[x_index] == key_for_x {
            // println!("ok {}", x_index);
            let y = x_index + 1;

            let key_for_y = q + a_cum[y-1];
            // println!("key_for_y: {}", key_for_y);
            let y_index = a_cum.lower_bound(&key_for_y);
            if y_index < y || y_index == n {continue}
            if a_cum[y_index] == key_for_y {
                // println!("y ok {}", y_index);
                let z = y_index + 1;
                
                let key_for_z = r + a_cum[z-1];
                // println!("key_for_z: {}", key_for_z);
                let z_index = a_cum.lower_bound(&key_for_z);
                if z_index < z || z_index == n {continue}
                if a_cum[z_index] == key_for_z {
                    // println!("z ok {}", z_index);
                    let w = z_index + 1;
                    flag = true;
                    break
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