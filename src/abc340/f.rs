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
    // 2024-02-11 10:26-13:22 (2h56m)降参

    let xxx: (isize, isize, isize) = extended_gcd(111, 30);
    // let yyy = solve_linear_diophantine_equation_in_two_variables(111, 30, 12);

    // let xxx: (isize, isize, isize) = extended_gcd(3, 5);
    // let yyy = solve_linear_diophantine_equation_in_two_variables(3, -5, 2);

    // println!("xxx = {:?}", xxx);
    // println!("yyy = {:?}", yyy);
    return;

    input! {
        x: isize,
        y: isize,
    }

    // 拡張ユークリッドの互除法 で
    // |AY - BX| == 2
    // <=> AY - BX = 2, 
    //     AY - BX = -2
    // となるような、 (A,B) を求める。

    // 例: (X,Y) = (3,5)
    // 5A - 3B = 2
    // gcd(3,5) = 1
    // 5a - 3b = gcd(5,-3) = gcd(-3%5,5) = gcd(2,5) = 1 を満たす(a,b)を拡張ユークリッドの互除法で取得すると、
    // (a,b)=(2,3)
    // 5a - 3b = 1 の両辺に2をかけて、
    // 5(2a) - 3(2b) = 2
    // 5A - 3B = 2
    // ∴ (A,B) = (2a,2b) = (4,6)
    
    // A = (2 + BX) / Y
    // |X|,|Y| <= 10^17

    if let Some((x1, y1, g)) = solve_linear_diophantine_equation_in_two_variables(y, - x, 2) {
        // println!("aaaa ----, g = {g}");
        println!("{} {}", x1, y1);
    }
    else {
        if let Some((x1, y1, g)) = solve_linear_diophantine_equation_in_two_variables(y, - x, -2) {
        // println!("bbbb ----, g = {g}");
        println!("{} {}", x1, y1);
        }
        else {
            println!("-1");
        }
    }

    // g = GCD(X, Y) とすると、
    // X = g * x0
    // Y = g * y0となる
    // AY - BX = g*(y0*A - x0*B)
    // ゆえに、g > 2 だと、|AY - BX| > 2 となってしまうので、解なし。
    // g == 1 のとき、2 = y0*A - x0*B となるような、(A,B)を探す
    // g == 2 のとき、1 = y0*A - x0*B となるような、(A,B)を探す

    // 外積を求める必要がある。

    // (x0, y0), (x1, y1) 
    // x0*y1 - y0*x1
    // (a * y - b * x).abs() / 2 == 1

    // (a * y - b * x).abs() == 2

    // <=>
    // (a * y - b * x) == 2
    // (a * y - b * x) == - 2

    // <=>
    // a == (b * x + 2) / y
    // a == (b * x - 2) / y



    // let (mut a, mut b, mut exist) = solve(x, y, 2);
    // if !exist {
    //     (a, b, exist) = solve(x, y, -2);
    // }

    // if exist {
    //     println!("{} {}", a, b);
    //     // println!("S = {}", a*y - b*x);
    // }
    // else {
    //     println!("-1");
    // }

}

fn solve_linear_diophantine_equation_in_two_variables(a: isize, b: isize, c: isize) -> Option<(isize, isize, isize)> {
    // 2変数の線形ディオファントス方程式 (一次不定方程式の整数版)
    // AX + BY = C の解(X,Y) を求める。(X,Y∈Z で、A,B,Cは整数の定数) 
    // ただし、C % gcd(A,B) == 0 である必要がある。
    // https://math.libretexts.org/Courses/Mount_Royal_University/MATH_2150%3A_Higher_Arithmetic/5%3A_Diophantine_Equations/5.1%3A_Linear_Diophantine_Equations#:~:text=A%20Linear%20Diophantine%20equation%20(LDE,and%20y%20are%20unknown%20variables.
    
    // 一次不定方程式が解を持つ条件 
    // <=> C % gcd(A,B) == 0
    // https://qiita.com/drken/items/b97ff231e43bce50199a#3-1-%E4%B8%80%E6%AC%A1%E4%B8%8D%E5%AE%9A%E6%96%B9%E7%A8%8B%E5%BC%8F-ax--by--c-%E3%81%8C%E6%95%B4%E6%95%B0%E8%A7%A3%E3%82%92%E3%82%82%E3%81%A4%E6%9D%A1%E4%BB%B6

    // Ax + By = gcd(A, B) を満たす解(x,y)の一つは、拡張ユークリッドの互除法で求められる。
    // C = c * gcd(A, B) とおいて、両辺にcをかけると、
    // c*Ax + c*By = c*gcd(A, B)
    // A(cx) + B(cy) = C
    // となる。つまり、(X,Y) = (cx, cy)で求まる。

    let (x, y, g) = extended_gcd(a, b); // Ax + By = gcd(A, B) を満たす解(x,y)を得る
    
    if c % g == 0 {
        // C が gcd(A,B)の整数倍であるときだけ、解が得られる。
        let c0 = c / g;
        return Some((c0*x, c0*y, g)) 
    }
    else {
        return None
    }
}

fn extended_gcd(a: isize, b: isize) -> (isize, isize, isize) {
    // 拡張ユークリッドの互除法で、 a*x + b*y = gcd(a, b) の解(x,y) を求める関数
    // 参考: https://qiita.com/drken/items/b97ff231e43bce50199a#3-1-%E4%B8%80%E6%AC%A1%E4%B8%8D%E5%AE%9A%E6%96%B9%E7%A8%8B%E5%BC%8F-ax--by--c-%E3%81%8C%E6%95%B4%E6%95%B0%E8%A7%A3%E3%82%92%E3%82%82%E3%81%A4%E6%9D%A1%E4%BB%B6
    // 参考: https://youtu.be/eiJyDb9c3Js?si=nThoqiWoLN2DqUTd

    // 拡張ユークリッドの互除法とは、整数の組 (a,b) を入力として与えたときに以下の手順によって 
    // ax+by=±gcd(a,b) となる整数の組 (x,y) を O(log(min(∣a∣,∣b∣))) で計算するアルゴリズム
    // max(|x|, |y|) <= max(|a|, |b|)であることは保証されている。

    // [1] ax + by = gcd(a, b) となる解(x,y)が必ず存在する
    // <=> ax + by = gcd(b % a, a) となる解(x,y)が必ず存在する
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

fn solve(x: isize, y: isize, two: isize) -> (isize, isize, bool) {
    // TLEしてしまう。
    let mut a = 1;
    let mut b = 1;
    let mut exist = true;

    if y != 0 {
        if y.abs() == 1 {
            b = 0;
            a = two / y;
        }
        else {
            let r = y.abs() - two;
            if x == 0 {
                if two % y == 0 {
                    a = two / y;
                    b = 1;
                }
                else {
                    exist = false;
                }
            }
            else {
                if r % x == 0 {
                    b = r / x;
                    a = (x * b + two) / y;
                }
                else {
                    exist = false;
                }
            }

        }
    }
    else if y == 0 {
        if two % x == 0 {
            b = -two / x;
            a = 1;
        }
        else {
            exist = false;
        }
    }

    return (a, b, exist)
}
