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
 
    // 2025-06-20 20:52-21:21 (29min)
    // 2025-06-20 23:49-24:18 (29min) 58
    // 2025-06-24 21:08-21:21 (13min) 71min =1h11min
    // 2025-06-25 19:08-21:20 (2h12min, 考察完了) 3h23min
    // 2025-06-26 00:11-00:33 (22min, 実装) 3h25min
    // 2025-06-26 12:24-12:45 (21min, debug) 3h46min
    // 2025-06-26 19:11-19:30 (19min, debug) 4h5min
    // total 4h5m

    // for i in 0..10 {
    //     let x = - i;
    //     println!("x = {x}, x % 5 = {}", x % 5);
    // }
    // return;
    // x = 0, x % 5 = 0
    // x = -1, x % 5 = -1
    // x = -2, x % 5 = -2
    // x = -3, x % 5 = -3
    // x = -4, x % 5 = -4
    // x = -5, x % 5 = 0
    // x = -6, x % 5 = -1
    // x = -7, x % 5 = -2
    // x = -8, x % 5 = -3
    // x = -9, x % 5 = -4

    input! {
        n: usize,
        mut k: usize,
        a: [i128; n]
    }
    // solve_brute_force(n,k,a.clone());
    solve(n,k,a);

    // loop {
    //     let (n, k, a) = random_input(); 
    //     println!("{} {}",n, k);
    //     println!("{}", a.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" "));
    //     let ans_bf = solve_brute_force(n,k,a.clone());
    //     let ans = solve(n,k,a);
    //     if ans_bf != ans {
    //         println!("(ans_bf, ans) = {:?}", (ans_bf, ans));
    //         break
    //     }
    // }


}

fn random_input() -> (usize, usize, Vec<i128>) {
    let mut rng = rand::thread_rng();

    // 1 <= k <= n <= 9
    let n = rng.gen_range(1..=20);
    // let n = rng.gen_range(1..=9);
    let k = rng.gen_range(1..=n);

    // a[i] ∈ [-5, 5]
    // let a: Vec<i128> = (0..n).map(|_| rng.gen_range(-5..=5)).collect();
    let a: Vec<i128> = (0..n).map(|_| rng.gen_range(-100..=10)).collect();

    (n, k, a)
}


fn solve_brute_force(n: usize, mut k: usize, a: Vec<i128>) -> i128 {
    let modulus = 1_000_000_007;
    // 
    let mut negs = vec![];
    let mut poss = vec![];
    let mut zeros = vec![];
    for i in 0..n {
        if a[i] < 0 {
            negs.push(a[i]);
        }
        else if a[i] == 0 {
            zeros.push(a[i]);
        }
        else {
            poss.push(a[i]);
        }
    }
    negs.sort();
    poss.sort();

    let mut anss = vec![];
    for num_pos in 0..poss.len()+1 {
        if k < num_pos {continue}
        // println!("num_pos = {:?} --- ", num_pos);
        // println!("anss = {:?}", anss);

        let num_r = k - num_pos;
        if num_r % 2 == 1 {
            // 負
            if zeros.len() != 0 {
                anss.push(0);
                continue
            }
            if num_r > negs.len() {continue}
            let mut cand = 1;
            for i in 0..num_pos {
                cand *= poss[i];
            }
            for i in 0..num_r {
                cand *= negs[negs.len()-1-i]
            }
            anss.push(cand);
        }
        else {
            // 正
            let mut cand = 1;
            for i in 0..num_pos {
                cand *= poss[poss.len()-1-i];
            }
            if num_r > negs.len() {
                if zeros.len() != 0 {
                    anss.push(0);
                }
                else {
                    continue
                }
            }
            else {
                for i in 0..num_r {
                    cand *= negs[i];
                }
                anss.push(cand);
            }
        }
    }
    // println!("anss = {:?}", anss);
    anss.sort();
    anss.reverse();
    let mut ans = (anss[0] % modulus + modulus) % modulus;
    println!("{}", ans);
    return ans

}

fn solve(n: usize, mut k: usize, a: Vec<i128>) -> i128 {
    let modulus = 1_000_000_007;

    fn is_positive(n: usize, mut k: usize, a: Vec<i128>) -> Option<i128> {
        let modulus = 1_000_000_007;
        let mut negs = vec![];
        let mut poss = vec![];
        let mut zeros = vec![];

        for i in 0..n {
            if a[i] < 0 {
                negs.push(a[i]);
            }
            else if a[i] == 0 {
                zeros.push(a[i]);
            }
            else {
                poss.push(a[i]);
            }
        }
        if zeros.len() == n {
            return Some(0)
        }
        negs.sort();
        poss.sort();
        let mut ans = 1;
        if k % 2 == 1 {
            if let Some(v) = poss.pop() {
                ans *= v;
                ans %= modulus;
                k -= 1;
            }
            else {
                if zeros.len() == 0 {
                    return None
                }
                else {
                    return Some(0)
                }
            }
        }
        poss.reverse();
        
        let mut heap = BinaryHeap::new();
        // 2個ずつ取り出す。
        for i in 0..poss.len()/2 {
            heap.push(poss[2*i] * poss[2*i+1]);
        }
        for i in 0..negs.len()/2 {
            heap.push(negs[2*i] * negs[2*i+1]);
        }
        if heap.len() * 2 < k {
            if zeros.len() == 0 {
                return None
            }
            else {
                return Some(0)
            }
        }
        else {
            for _ in 0..k/2 {
                let v = heap.pop().unwrap() % modulus;
                ans *= v;
                ans %= modulus;
            }
            return Some(ans)
        }
    }

    if let Some(ans) = is_positive(n, k, a.clone()) {
        println!("{}", ans);
        return ans
    }
    else {
        let mut negs = vec![];
        let mut poss = vec![];
        let mut zeros = vec![];
        for i in 0..n {
            if a[i] < 0 {
                negs.push(a[i]);
            }
            else if a[i] == 0 {
                zeros.push(a[i]);
            }
            else {
                poss.push(a[i]);
            }
        }
        negs.sort();
        poss.sort();
        let mut ans = 1;
        ans *= negs.pop().unwrap();
        ans %= modulus;
        ans += modulus;
        ans %= modulus;
        k -= 1;
        if k == 0 {
            println!("{}", ans);
            return ans;
        }
        if k % 2 == 1 {
            ans *= poss[0];
            ans %= modulus;
            k -= 1;
            poss.remove(0);
            if k == 0 {
                println!("{}", ans);
                return ans;
            }
        }

        // // 負が偶数個
        // -1,-2,-3,-4
        // // 正
        // 1
        // k=2

        negs.reverse();
        let mut heap =  BinaryHeap::new();
        for i in 0..negs.len()/2 {
            heap.push(Reverse(negs[2*i]*negs[2*i+1]));
        }
        for i in 0..poss.len()/2 {
            heap.push(Reverse(poss[2*i]*poss[2*i+1]));
        }
        for _ in 0..k/2 {
            let Reverse(v) = heap.pop().unwrap();
            ans = ans * v % modulus;
            ans %= modulus;
        }        
        println!("{}", ans);
        return ans
    }


    // // let mut a_abs = a.clone().iter().map(|v| v.abs()).collect::<Vec<isize>>();

    // // 負の数の処理がめんどい。
    // // k個の要素を選ぶときに、負の個数は偶数個にしたい。
    // // -2, 8, 9,-10
    // // オーバーフローの判断が一番大変。
    // // k個の構成要素...
    // // [1]正の個数 >= k ならば、答えは正 <- これは特殊なケース
    // // [2]正の個数 + 負の個数 / 2 >= k ならば答えは正
    // // [3]正の個数 + 負の個数 / 2 < k ならば答えは負 ただし、0があったら答えは0
    // // 
    // // 絶対値でソートする。
    // // [3]のケースなら、絶対値が低いやつからK個かけるだけでok
    // // [1]と[2]なら、絶対値が高いやつから順にK個かける。
    // // -9,8,-7,4,-3,2,1,...
    // // 上からK個の中にある負の個数 % 2 == 0 ならそのままでok
    // // 上からK個の中にある負の個数 % 2 == 1 なら1個ずつパスしていく。


    // // 1,-1,-1,-3 K=2
    // // 絶対値の上位K個に、負の個数が何個あるか?
    // // 負の個数が偶数個だったら、そのままK個かければいい。
    // // 負の個数が奇数個だったら、そのままK個かければいい。
    // // -50,-40,-30,20,  3,2,-1 
    // // K=4だったら、
    // // 一番小さいマイナスを不採用にして、上位K個に入っていない最大のプラス(いれば)を使う。
    // // or 一番小さいプラスを不採用にして、上位K個に入っていない絶対値が最大のマイナス(いれば)を使う。
    // // 上位K個のうち、同率K位がたくさんいる場合は面倒。
    // // 
    // // k= 4
    // // -50,-40,-30,20,20,20,20,-20,  3,2,-1 


    // let mut abss = vec![];
    // let mut poss = vec![];
    // let mut negs = vec![];
    // let mut zeros = vec![];
    // for i in 0..n {
    //     abss.push((a[i].abs(), i));

    //     if a[i] > 0 {
    //         poss.push(a[i]);
    //     }
    //     else if a[i] == 0 {
    //         zeros.push(a[i]);
    //     }    
    //     else {
    //         negs.push(a[i]);
    //     }        
    // }
    // abss.sort();
    // abss.reverse();

    // poss.sort();
    // poss.reverse();
    // negs.sort();

    // // let mut is_oks = vec![false; k+1];
    // // is_oks[0] = 

    // // is_oks[x] := 正をx個取った時、最終的に正にできるか?
    // let mut is_oks = vec![false; poss.len() + 1];

    // let mut is_negative_ans = true;
    // for num_pos in 0..poss.len()+1 {
    //     if num_pos > k {continue}
    //     let rem = k - num_pos;
    //     if rem % 2 == 0 && rem <= negs.len() {
    //         is_oks[num_pos] = true;
    //         is_negative_ans = false;
    //     }
    // }
    // if is_negative_ans {
    //     if zeros.len() != 0 {
    //         println!("0");
    //         return;
    //     }
    //     else {
    //         // 絶対値が小さくなるように答えていく.
    //     }
    // }
    // else {
    //     // 絶対値が大きくなるように、答えていく。
    //     let mut max_num_pos = 0;
    //     for num_pos in 0..(poss.len()+1).rev() {
    //         if is_oks[num_pos] {
    //             max_num_pos = num_pos;
    //             break;
    //         }
    //     }
    //     let mut min_num_pos = 0;
    //     for num_pos in 0..(poss.len()+1) {
    //         if is_oks[num_pos] {
    //             min_num_pos = num_pos;
    //             break;
    //         }
    //     }
    //     if max_num_pos == 0 {
    //         let mut ans = 1;
    //         for i in 0..k {
    //             ans *= ((negs[i] % modulus) + modulus) % modulus;
    //         }
    //         println!("{}", ans);
    //         return;
    //     }
    //     if min_num_pos == poss.len() {
    //         let mut ans = 1;
    //         for i in 0..poss.len() {
    //             ans *= ((poss[i] % modulus) + modulus) % modulus;
    //         }
    //         for i in 0..k-poss.len() {
    //             ans *= ((negs[i] % modulus) + modulus) % modulus;
    //         }
    //         println!("{}", ans);
    //         return;
    //     }
    //     if min_num_pos == max_num_pos {
    //         let mut ans = 1;
    //         for i in 0..min_num_pos {
    //             ans *= ((poss[i] % modulus) + modulus) % modulus;
    //         }
    //         for i in 0..k-min_num_pos {
    //             ans *= ((negs[i] % modulus) + modulus) % modulus;
    //         }
    //         println!("{}", ans);
    //         return;
    //     }

    //     // 下がり始めた一番最初を叩けばいい。
    //     for num_pos in min_num_pos..max_num_pos-2 {
    //         min_num_pos
    //     }

    // }



    // let mut dp = vec![-1; poss.len() + 1];




    // // 0 を考えるのはだるいので、先に0は取り除きたい。
    // // 0 を取らざるを得ないケースは、
    // // 正と負の数をK個取ると、答えが負になるとき。
    // // 正をx個とって、最終的なアンサーを正にすることは可能か?





    // let mut btree = BTreeMap::new();
    // for i in 0..k {
    //     let (ai, index) = abss[i];
    //     btree.entry(ai).or_insert(vec![]).push(index);
    // }

    // let mut count = 0;
    // let mut border_ai_abs = 0;
    // let mut num_pre_pos = 0;
    // let mut num_pre_zero = 0;
    // let mut num_pre_neg = 0;

    // let mut num_border_pos = 0;
    // let mut num_border_zero = 0;
    // let mut num_border_neg = 0;
    // for (ai_abs, vec_ai) in btree.iter().rev() {
    //     if count + vec_ai.len() >= k {
    //         border_ai_abs = ai_abs;
    //         for index in vec_ai {
    //             let ai = a[index];
    //             if ai >  0 {
    //                 num_border_pos += 1;
    //             }
    //             else if ai == 0 {
    //                 num_border_zero += 1;
    //             }
    //             else {
    //                 num_border_neg += 1;
    //             }
    //         }
    //         break
    //     }
    //     count += vec_ai.len();
    //     for index in vec_ai {
    //         let ai = a[index];
    //         if ai >  0 {
    //             num_pre_pos += 1;
    //         }
    //         else if ai == 0 {
    //             num_pre_zero += 1;
    //         }
    //         else {
    //             num_pre_neg += 1;
    //         }
    //     }
    // }
    // // border_ai_abs はK個以内に含まれている。
    // let num_pre_sum = num_pre_pos + num_pre_zero + num_pre_neg;
    // let num_remain = k - num_pre_sum; // 取らないといけない奴ら。

    // if num_neg % 2 == 0 {
    //     // 前から順番にK個取るだけでokなケース
    //     // 全体でN個ある。正をx個とって、負を偶数個にできるか?負を奇数にできるか?
    //     let mut dp = vec![vec![false; 2]; num_remain+1];

    //     dp[0][0] = true;
    //     for i in 00 {

    //     }
    //     if num_border_pos >= remain {
    //         // 前から順番にK個取るだけでok
    //     }
    //     else if num_border_pos >= remain {
    //         // 残り5個
            
    //     }
    // }
    // else {

    // }






    // // if a[index] < 0 {
    // //     num_neg += 1;
    // // }
    // // else if a[index] == 0 {
    // //     num_zero += 1;
    // // }
    // // else {
    // //     num_neg += 1;
    // // }


    // abss.sort();
    // if num_pos + num_neg /2 < k {
    //     if num_zero > 0 {
    //         println!("0");
    //         return;
    //     }
    //     else {
    //         // 小さいやつから採用していく。
    //         for i in 0..k {

    //         }
    //     }
    // }
    // else {
    //     abss.reverse();

    // }


    // // let mut ap = vec![];
    // // let mut an = vec![];
    // // for i in 0..n {
    // //     if a[i] < 0 {
    // //         an.push(a[i]);
    // //     }
    // //     else {
    // //         ap.push(a[i]);
    // //     }
    // // }
    // // an.sort();
    // // ap.sort();

    // // let mut min_cum_n_abs = an.clone().iter().map(|v| v.abs()).collect::<Vec<isize>>();
    // // min_cum_n_abs.sort();
    // // let mut max_cum_n_abs = min_cum_n_abs.clone();
    // // max_cum_n_abs.reverse();
    // // for i in 1..min_cum_n_abs.len() {
    // //     min_cum_n_abs[i] = min_cum_n_abs[i] * min_cum_n_abs[i-1] % modulus;
    // //     max_cum_n_abs[i] = max_cum_n_abs[i] * max_cum_n_abs[i-1] % modulus;
    // // }
    // // min_cum_n_abs.insert(0, 1);
    // // max_cum_n_abs.insert(0, 1);


    // // let mut min_cum_p = ap.clone();
    // // let mut max_cum_p = ap.clone();
    // // max_cum_p.reverse();
    // // for i in 1..ap.len() {
    // //     min_cum_p[i] = min_cum_p[i] * min_cum_p[i-1];
    // //     min_cum_p[i] %= modulus;

    // //     max_cum_p[i] = max_cum_p[i] * max_cum_p[i-1];
    // //     max_cum_p[i] %= modulus;
    // // }
    // // min_cum_p.insert(0, 1);
    // // max_cum_p.insert(0, 1);

    // // // let mut ans = 0;
    // // for num_n in 0..an.len()+1 {
    // //     if k < num_n {continue}
    // //     let num_p = k - num_n;
    // //     if num_n % 2 == 0 {
    // //         // 絶対値を最大にする。
    // //         let cand = max_cum_p[num_p] * 

    // //     }
    // //     else {

    // //     }
    // // }


}