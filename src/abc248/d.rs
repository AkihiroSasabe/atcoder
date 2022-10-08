use proconio::input;
use std::cmp::Ordering;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        q: usize,
        lrx: [[usize; 3]; q]
    }

    // 数列の値をKey、数列の順番をValueとした辞書を用意
    let mut order_dict: Vec<Vec<usize>> = vec![vec![]; n+1];

    for i in 0..n {
        order_dict[a[i]].push(i+1)
    }

    // println!("{:?}", order_dict);
    for i in lrx {
        // println!("[i[2]: {}", i[2]);
        // println!("order_dict[i[2]]: {:?}", order_dict[i[2]]);
        // println!("order_dict[i[2]].lower_bound(&i[0]): {:?}", order_dict[i[2]].lower_bound(&i[0]));
        // println!("order_dict[i[2]].lower_bound(&i[1])): {:?}", order_dict[i[2]].lower_bound(&i[1]));
        let answer =  order_dict[i[2]].lower_bound(&(i[1]+1)) - order_dict[i[2]].lower_bound(&i[0]);
        println!("{}", answer);
    }

    // これだと計算量がQ * ( NlogN: ソート + logN: 2分探索)となってしまう。
    // n = 2 * 10 ** 5
    // 488_242_905_821.207 = 488 * 10**9 
    // for i in lrx {
    //     let a2 = &mut a.clone()[i[0]-1..i[1]];
    //     a2.sort();
    //     let ind_x_ormore = a2.lower_bound(&(i[2]));
    //     let ind_xp1_ormore = a2.lower_bound(&(i[2]+1));
    //     let answer = ind_xp1_ormore - ind_x_ormore;
    //     println!("{}", answer);
    // }

}

// lower_bound=Key★以★上★のインデックス、upper_bound=Key★よ★り★大きいインデックス
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