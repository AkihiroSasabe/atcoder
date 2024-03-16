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
fn main() {
    // 2024-03-13 20:24-21:37 (1h13min = 73min)
    input! {
        t: usize,
    }

    let mut n = vec![];
    let mut s = vec![];
    let mut k = vec![];
    for i in 0..t {
        input! {
            ni: isize,
            si: isize,
            ki: isize,
        }
        n.push(ni);
        s.push(si);
        k.push(ki);
    }

    for i in 0..t {
        let ni = n[i];
        let si = s[i];
        let ki = k[i];

        // (S+X*K) % N == 0 を満たす最小の 0以上のX が知りたい。
        // <=> S+X*K = Y*N
        // <=> -K*X + N*Y = S を満たす整数解(X, Y)　が存在し、かつその中で、最小の0以上のXが知りたい。

        // -k * x + n * y = s ---- (1)の、解が(x0, y0)だとすると、
        // -k * x0 + n * y0 = s ---- (2) とおける。
        // (1) - (2)より、
        // -k(x-x0) + n(y -y0) = 0
        // (x-x0) = (y -y0) * n / k
        // x - x0 = (y-y0) * n / k
        // x = x0 + (y-y0) * n / k

        if let Some((u1, u0, v1, v0)) = get_general_solution_to_linear_diophantine_equation_in_two_variables(-ki, ni, si) {
            // y はなんでもいい。
            // x は0以上 で最小が条件。
            // x = u1 * t + u0

            let t_min = match u0 >= 0 {
                true => - u0 / u1.abs(),
                _ => {
                    let d_min = u0.abs() / u1.abs();
                    let r_min = u0.abs() % u1.abs();
                    match r_min {
                        0 => d_min,
                        _ => d_min + 1
                    }
                }
            };
            let x_min = u1.abs() * t_min + u0;
            println!("{}", x_min);
        }
        else {
            println!("-1");
        }
    }    
}

fn get_general_solution_to_linear_diophantine_equation_in_two_variables(a: isize, b: isize, c: isize) -> Option<(isize, isize, isize, isize)> {
    // ax + by = c の 一般解を得る
    // ただし、 c % gcd(a,b) == 0 じゃなければ、一般解は得られない。
    // ax1 + by1 = c
    // a(x-x1) + b(y-y1) = 0
    // <=> x - x1 = - b * (y-y1) / a かつ b * (y-y1) % a == 0 
    // <=> x - x1 = - b * (y-y1) / a かつ y - y1 = t * (a / gcd(a,b)) (t ∈ Z)
    // <=> x - x1 = - t * (b / gcd(a,b)) かつ y - y1 = t * (a / gcd(a,b)) (t ∈ Z)
    // <=> x = - b / gcd(a,b) * t + x1 
    //  && y =   a / gcd(a,b) * t + y1 (t ∈ Z)

    // 入力: ax + by = c の係数 (a, b, c)
    // 出力: ax + by = c の一般解 (x,y) の係数 (u1, u0, v1, v0)
    // すなわち、
    // x = u1 t + u0 
    // y = v1 t + v0 (t ∈ Z)
    // としたときの、(u1, u0, v1, v0)
    // ただし、c % gcd(a,b) != 0 は Noneを返す

    match solve_linear_diophantine_equation_in_two_variables(a, b, c) {
        Some((x1, y1, gcd_ab)) => {
            let u1: isize = - b / gcd_ab;
            let u0: isize = x1;
            let v1: isize = a / gcd_ab;
            let v0 = y1;
            Some((u1, u0, v1, v0))
        },
        None => None
    }
}

fn solve_linear_diophantine_equation_in_two_variables(a: isize, b: isize, c: isize) -> Option<(isize, isize, isize)> {
    // 2変数の線形ディオファントス方程式 (一次不定方程式の整数版)
    // ax + by = c の解の一つ(x1, y1) を求める。(x, y ∈ Z で、a,b,cは整数の定数) 
    // ただし、c % gcd(a,b) == 0 である必要がある。
    // https://math.libretexts.org/Courses/Mount_Royal_University/MATH_2150%3A_Higher_Arithmetic/5%3A_Diophantine_Equations/5.1%3A_Linear_Diophantine_Equations#:~:text=A%20Linear%20Diophantine%20equation%20(LDE,and%20y%20are%20unknown%20variables.

    // 入力: a y + b y = c の係数 a, b, c
    // 出力: a x + b y = c の解の一つ (x1, y1) と、aとbの最大公約数 gcd(a, b)

    // 一次不定方程式が解を持つ条件 
    // <=> c % gcd(a,b) == 0
    // https://qiita.com/drken/items/b97ff231e43bce50199a#3-1-%E4%B8%80%E6%AC%A1%E4%B8%8D%E5%AE%9A%E6%96%B9%E7%A8%8B%E5%BC%8F-ax--by--c-%E3%81%8C%E6%95%B4%E6%95%B0%E8%A7%A3%E3%82%92%E3%82%82%E3%81%A4%E6%9D%A1%E4%BB%B6

    // a x0 + b y0 = gcd(a, b) ---- (*) を満たす解(x0, y0)の一つは、拡張ユークリッドの互除法で求められる。
    // c0 := c / gcd(a, b) とおいて、(*)の両辺にc0をかけると、
    // c0 a x0 + c0 b y0 = c0 gcd(a, b)
    // <=> a (c0 x0) + b (c0 y0) = c
    // (x1,y1) := (c0 x0, c0 y0) とおくと、
    // a x1 + b y1 = c 
    // となるので、(x, y) = (x1, y1) = (c0 x0, c0 y0) が a y + b y = c の解となる。

    let (x0, y0, gcd_ab) = extended_gcd(a, b); // ax + by = gcd(a, b) を満たす解の一つ(x0, y0)を得る
    
    if c % gcd_ab == 0 {
        // c が gcd(a,b)の整数倍であるときだけ、解が得られる。
        let c0 = c / gcd_ab;
        return Some((c0 * x0, c0 * y0, gcd_ab)) 
    }
    else {
        return None
    }
}


fn extended_gcd(a: isize, b: isize) -> (isize, isize, isize) {
    // ■拡張ユークリッドの互除法で、 a*x + b*y = gcd(a, b) の解(x,y) を求める関数
    // 参考: https://qiita.com/drken/items/b97ff231e43bce50199a#3-1-%E4%B8%80%E6%AC%A1%E4%B8%8D%E5%AE%9A%E6%96%B9%E7%A8%8B%E5%BC%8F-ax--by--c-%E3%81%8C%E6%95%B4%E6%95%B0%E8%A7%A3%E3%82%92%E3%82%82%E3%81%A4%E6%9D%A1%E4%BB%B6
    // 参考: https://youtu.be/eiJyDb9c3Js?si=nThoqiWoLN2DqUTd

    // ■インターフェース
    // 入力: ax + by = gcd(a, b) の係数 a, b
    // 出力: ax + by = gcd(a, b) の解(x0, y0)と、aとbの最大公約数 gcd(a,b)

    // ■計算量
    // 拡張ユークリッドの互除法とは、整数の組 (a,b) を入力として与えたときに以下の手順によって 
    // ax+by=±gcd(a,b) となる整数の組 (x,y) を O(log(min(∣a∣,∣b∣))) で計算するアルゴリズム
    
    // ■備考
    // max(|x|, |y|) <= max(|a|, |b|)であることは保証されている。

    // ■証明
    // [1] ax + by = gcd(a, b) となる解(x,y)が必ず存在する (∵けんちょん記事参考)
    // <=> ax + by = gcd(b % a, a) となる解(x,y)が必ず存在する (∵ユークリッドの互除法 より、 gcd(a, b) = gcd(b % a, a))
    // [2] (b % a)X + aY = gcd(b % a, a) となる解(X,Y)も存在する
    // 以上を繰り返すと...
    // [3] 0 * X0 + g * Y0 = gcd(0, g) = g となる解(X0,Y0)も存在する (g := gcd(a,b))
    // (X0,Y0) = (0,1) は、上の式の解の一つとなる(X0はなんでも良いが、計算が楽なので0とする)
    
    // [4] [2]の解(X,Y)から[1]の解(x,y)の関係性が分かれば、[3]の解(X0,Y0)から[1]の解(x,y)が、再帰的に得られる。
    // [5] 商と余りの関係式 N = (N / D) * D + N % D に (N,D)=(b,a) を代入する
    // b = b % a + (b / a) * a
    // <=> b % a = - (b / a) * a + b
    // 両辺にXをかける
    // <=> (b % a) * X = - (b / a) * a * X + b * X
    // 移項して整理
    // <=> (b % a) X = a (- (b / a) X) + b X
    // 両辺にa Y を足す
    // <=> (b % a) X + a Y = a (- (b / a) X) + b X + a Y
    // <=> (b % a) X + a Y = a (Y - (b / a) X) + b X
    // 
    // (b % a) X + a Y = gcd(b%a, a) = gcd(a,b)より、
    // ((Y - (b / a)) X, X) が、
    // ax + by = gcd(a, b) の解(x,y)となることがわかる。すなわち、
    // x = (Y - (b / a)X)
    // y = X

    // ■ max(|x|, |y|) <= max(|a|, |b|) の証明 (未完)
    // https://twitter.com/kyopro_friends/status/1766040318846365819
    // |a| <= |b| とする。
    // |X| <= |b| かつ |Y| <= |a| かつ |X| < |Y| が成り立つと仮定したとき、
    // x_next = Y - (b/a)
    // y_next = X
    // より、
    // |x_next| <= |b|
    // |y_next| <= |a|
    // が示せれば、帰納法でいけるらしい。(正直わからなかった) 

    if a == 0 {
        // ユークリッドの互除法
        // gcd(a,b) = gcd(b % a, a)
        // の終了条件は、gcd(0, g) = g であるから、
        // 0*x + g*y = g
        // ∴ (x,y) = (0,1) <- xは任意の整数が解となりうるが、楽なので0とする。
        let x0 = 0;
        let y0 = 1;
        let g = b;
        // println!("(a, b) = {}, {}, (x0, y0) = , {}, {}", a, b, x0, y0);
        return (x0, y0, g)
    }
    let (nx, ny, g) = extended_gcd(b % a, a);
    let x = ny - (b / a) * nx;
    let y = nx;
    // println!("(a, b) = ({}, {}), (x, y) = ({}, {})", a, b, x, y);
    return (x, y, g)
}
