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
use std::collections::HashSet;
use proconio::marker::Chars;
use std::f64::consts::PI;
use std::mem::swap;
use superslice::*;
use std::fmt;
use std::ops;

fn main() {
    let MODULUS = 998244353;
    let mi_generator = ModintGenerator::new(MODULUS);

    input! {
        q: usize
    }

    // [1]ModIntを使った解き方
    let mut s = mi_generator.generate(1);
    let mut queue = VecDeque::new();
    queue.push_back(1);
    let mut power_of_10 = mi_generator.generate(1);
    for i in 0..q {
        input! {
            kind: usize
        }
        // dbg!(s);
        // println!("queue: {:?}", queue);
        if kind == 1 {
            input! {
                x: usize
            }
            s = s * 10 + x;
            power_of_10 = power_of_10 * 10;
            queue.push_back(x);
        }
        else if kind == 2 {
            let head = queue.pop_front().unwrap();
            s = s - head * power_of_10;
            power_of_10 /= 10;
        }
        else {
            println!("{}", s);
        }
    }
    return;

    // ModIntの動作確認
    let mut a = mi_generator.generate(5);
    let mut b = mi_generator.generate(998244357); // 4
    let mut c = mi_generator.generate(998244352);

    // ModIntのprint
    println!("a={}", a); // {}だけだと値だけ表示
    println!("a={:?}", a); // {:?}だとmodulusも表示

    // ModIntの演算子の確認
    println!("modulus = {}", MODULUS);
    println!("{} * {} = {}", a, b, a*b);
    println!("{} * {} = {}", a, c, a*c);
    println!("{} + {} = {}", a, b, a+b);
    println!("{} + {} = {}", a, c, a+c);
    println!("{} - {} = {}", a, b, a-b);
    println!("{} - {} = {}", a, c, a-c);
    println!("{}^3={}", a, a.pow(3));
    println!("{} / {} = {}", a, b, a/b);
    println!("{} / {} = {}", a, c, a/c);
    
    // 代入演算子の確認
    println!("==== assignment operator ====");
    println!("{} *= {}", a, b);
    a *= b;
    println!("a: {}", a);

    println!("{} += {}", a, b);
    a += b;
    println!("a: {}", a);

    println!("{} -= {}", a, b);
    a -= b;
    println!("a: {}", a);

    println!("{} /= {}", a, b);
    a /= b;
    println!("a: {}", a);


    // usize型との乗算
    println!("==== mul operation with usize ====");
    // ModInt * 整数の確認
    println!("a={}, a * {} = {}", a, 6, a * 6_usize);

    // 整数 * ModIntの確認
    println!("a={}, {} * a = {}", a, 6, 6_usize * a);

    // 代入演算子の確認
    println!("a={}, a *= {}", a, 6);
    a *= 6;
    println!("a: {}", a);


    // usize型との加算
    println!("==== add operation with usize ====");
    // ModInt * 整数の確認
    println!("a={}, a + {} = {}", a, 6, a + 6_usize);

    // 整数 * ModIntの確認
    println!("a={}, {} + a = {}", a, 6, 6_usize + a);

    // 代入演算子の確認
    println!("a={}, a += {}", a, 6);
    a += 6;
    println!("a: {}", a);


    // usize型との減算
    println!("==== sub operation with usize ====");
    // ModInt - 整数の確認
    println!("a={}, a - {} = {}", a, 6, a - 6_usize);

    // 整数 - ModIntの確認
    println!("a={}, {} - a = {}", a, 6, 6_usize - a);

    // 代入演算子の確認
    println!("a={}, a -= {}", a, 6);
    a -= 6;
    println!("a: {}", a);
    

    // usize型との割算
    println!("==== div operation with usize ====");
    // ModInt / 整数の確認
    println!("a={}, a / {} = {}", a, 6, a / 6_usize);

    // 整数 / ModIntの確認
    println!("a={}, {} / a = {}", a, 6, 6_usize / a);

    // 代入演算子の確認
    println!("a={}, a /= {}", a, 6);
    a /= 6;
    println!("a: {}", a);
    
    // [2]ModIntを使わない解き方
    // let mut s = 1;
    // let mut queue = VecDeque::new();
    // queue.push_back(1);
    // let mut power_of_10: usize = 1;
    // for i in 0..q {
    //     input! {
    //         kind: usize
    //     }
    //     // dbg!(s);
    //     // println!("queue: {:?}", queue);
    //     if kind == 1 {
    //         input! {
    //             x: usize
    //         }
    //         s = (s % MODULUS) * 10 + x;
    //         power_of_10 = (power_of_10 * 10) % MODULUS;
    //         s = s % MODULUS;
    //         queue.push_back(x);
    //     }
    //     else if kind == 2 {
    //         let head = queue.pop_front().unwrap();
    //         s = MODULUS + s - ((head * power_of_10) % MODULUS);
    //         power_of_10 = mod_dev(power_of_10, 10, MODULUS);
    //         s = s % MODULUS;
    //     }
    //     else {
    //         println!("{}", s % MODULUS);
    //     }
    // }
}


// ModIntの実装参考
// https://qiita.com/namn1125/items/5100cb85021a1d6e8f6c
// https://github.com/rust-lang-ja/ac-library-rs/blob/master/src/modint.rs
// https://github.com/kenkoooo/competitive-programming-rs/blob/master/src/math/mod_int.rs
// AtCoderの公式(C++): https://github.com/atcoder/ac-library/blob/master/document_ja/modint.md
struct ModintGenerator {
    modulus: usize
}

impl ModintGenerator {
    fn new(modulus: usize) -> ModintGenerator {
        ModintGenerator {modulus: modulus}
    }
    fn generate(&self, value: usize) -> Modint {
        let modint = Modint::new(self.modulus, value);
        return modint
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Modint {
    modulus: usize,
    value: usize,
}

impl Modint {
    fn new(modulus: usize, value: usize) -> Modint {
        Modint{modulus: modulus, value: value % modulus}
    }
    // mod p を法とした時の逆数(逆元という) 1 / a の値
    fn inverse(&self) -> Self {
        // フェルマーの小定理
        //     a^(p-1) = 1     (mod p)
        // <=> a * a^(p-2) = 1 (mod p)
        // <=> 1 / a = a^(p-2) (mod p)
        // (ただし、法pは素数)
        self.pow(self.modulus - 2)
    }

    // mod p を法とした時の累乗
    // base^(x) % mod を繰り返し二乗法により、O(log2(x))の計算量で求める　(O(x)だとTLE)
    // No.69参照
    fn pow(&self, mut exponent: usize) -> Self {
        // 例: 3^4= (3^2)^2 = 9^2 = 81^1
        // 初期
        // 3^4
        // remainder=1
        // base=3
        // exp=4

        // i=0:
        // remainder = 1
        // base = 3 * 3 = 9
        // exp = 4 / 2 = 2

        // i=1:
        // remainder = 1
        // base = 9 * 9 = 81
        // exp = 2 / 2 = 1

        // i=2:
        // remainder = remainder * base = 81
        // base = 81 * 81
        // exp = 1 / 2 = 0
        let mut base = self.value;

        let mut remainder = 1;
        while exponent != 0 {
            if exponent % 2 == 1 {
                remainder = (remainder * base) % self.modulus;
            }
            base = (base * base) % self.modulus;
            exponent /= 2;
        }
        Self {
            modulus: self.modulus,
            value: remainder
        }
    }

}

// To use the `{}` marker, the trait `fmt::Display` must be implemented
// manually for the type.
// `{}` というマーカーを使用するためには、
// この型専用の`fmt::Display`というトレイトが実装されていなくてはなりません。
impl fmt::Display for Modint {
    // This trait requires `fmt` with this exact signature.
    // このトレイトは`fmt`が想定通りのシグネチャであることを要求します。
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        // 厳密に最初の要素を、与えられた出力ストリーム `f` に書き込みます。
        // `fmt::Result`を返します。これはオペレーションが成功したか否か
        // を表します。
        // `write!`は`println!`に非常によく似た文法を使用していることに注目。
        write!(f, "{}", self.value)
    }
}

// impl トレイト for 構造体 {}
// +演算子
impl std::ops::Add for Modint {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            modulus: self.modulus,
            value: (self.value + other.value) % self.modulus
        }
    }
}

// += 演算子
impl std::ops::AddAssign for Modint {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            modulus: self.modulus,
            value: (self.value + other.value) % self.modulus
        }
    }
}

// *演算子
impl std::ops::Mul for Modint {
    // The multiplication of rational numbers is a closed operation.
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self {
            modulus: self.modulus,
            value: (self.value * other.value) % self.modulus 
        }
    }
}

// *=演算子
impl std::ops::MulAssign for Modint {
    fn mul_assign(&mut self, other: Self) {
        *self = Self {
            modulus: self.modulus,
            value: (self.value * other.value) % self.modulus
        }
    }
}

// -演算子
impl std::ops::Sub for Modint {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            modulus: self.modulus,
            // 引き算が負にならないようにmodulusを足しておく
            value: (self.modulus + self.value - other.value) % self.modulus
        }
    }
}

// -=演算子
impl std::ops::SubAssign for Modint {
    fn sub_assign(&mut self, other: Self) {
        *self = Self {
            modulus: self.modulus,
            // 引き算が負にならないようにmodulusを足しておく
            value: (self.modulus + self.value - other.value) % self.modulus
        };
    }
}

// /演算子
impl std::ops::Div for Modint {
    // The division of rational numbers is a closed operation.
    type Output = Self;

    fn div(self, other: Self) -> Self {
        if other.value == 0 {
            panic!("Cannot divide by zero-valued `Rational`!");
        }
        Self {
            modulus: self.modulus,
            value: (self.value * other.inverse().value) % self.modulus 
        }
    }
}

// /=演算子
impl std::ops::DivAssign for Modint {
    fn div_assign(&mut self, other: Self) {
        *self = Self {
            modulus: self.modulus,
            value: (self.value * other.inverse().value) % self.modulus
        };
    }
}


// usize型との演算
// ModInt * 整数
impl std::ops::Mul<usize> for Modint {
    type Output = Modint;

    fn mul(self, other: usize) -> Modint {
        Modint {
            modulus: self.modulus,
            value: (self.value * (other % self.modulus)) % self.modulus,
        }
    }
}

// 整数 * ModInt
impl std::ops::Mul<Modint> for usize {
    type Output = Modint;

    fn mul(self, other: Modint) -> Modint {
        Modint {
            modulus: other.modulus,
            value: ((self % other.modulus) * other.value) % other.modulus,
        }
    }
}

// ModInt *= 整数
impl std::ops::MulAssign<usize> for Modint {
    fn mul_assign(&mut self, other: usize) {
        *self = Self {
            modulus: self.modulus,
            value: (self.value * (other % self.modulus)) % self.modulus
        }
    }
}



// ModInt + 整数
impl std::ops::Add<usize> for Modint {
    type Output = Modint;

    fn add(self, other: usize) -> Modint {
        Modint {
            modulus: self.modulus,
            value: (self.value + other) % self.modulus,
        }
    }
}

// 整数 + ModInt
impl std::ops::Add<Modint> for usize {
    type Output = Modint;

    fn add(self, other: Modint) -> Modint {
        Modint {
            modulus: other.modulus,
            value: ((self % other.modulus) + other.value) % other.modulus,
        }
    }
}

// ModInt += 整数
impl std::ops::AddAssign<usize> for Modint {
    fn add_assign(&mut self, other: usize) {
        *self = Self {
            modulus: self.modulus,
            value: (self.value + (other % self.modulus)) % self.modulus
        }
    }
}







// ModInt - 整数
impl std::ops::Sub<usize> for Modint {
    type Output = Modint;

    fn sub(self, other: usize) -> Modint {
        Modint {
            modulus: self.modulus,
            value: (self.modulus + self.value - other) % self.modulus,
        }
    }
}

// 整数 - ModInt
impl std::ops::Sub<Modint> for usize {
    type Output = Modint;

    fn sub(self, other: Modint) -> Modint {
        Modint {
            modulus: other.modulus,
            value: (other.modulus + (self % other.modulus) - other.value) % other.modulus,
        }
    }
}

// ModInt -= 整数
impl std::ops::SubAssign<usize> for Modint {
    fn sub_assign(&mut self, other: usize) {
        *self = Self {
            modulus: self.modulus,
            value: (self.modulus + self.value - (other % self.modulus)) % self.modulus
        }
    }
}






// ModInt / 整数
impl std::ops::Div<usize> for Modint {
    type Output = Modint;

    fn div(self, other: usize) -> Modint {
        let other_modint = Modint::new(self.modulus, other);
        return self / other_modint
    }
}

// 整数 / ModInt
impl std::ops::Div<Modint> for usize {
    type Output = Modint;

    fn div(self, other: Modint) -> Modint {
        let self_modint = Modint::new(other.modulus, self);
        return self_modint / other
    }
}

// ModInt /= 整数
impl std::ops::DivAssign<usize> for Modint {
    fn div_assign(&mut self, other: usize) {
        let other_modint = Modint::new(self.modulus, other);
        *self = *self / other_modint;
    }
}





// mod p を法とした時の割り算 a / b の値
fn mod_dev(a: usize, b: usize, modulo: usize) -> usize {
    return a * mod_inverse(b, modulo) % modulo
}

// mod p を法とした時の逆数(逆元という) 1 / b の値
fn mod_inverse(a: usize, modulo: usize) -> usize {
    // フェルマーの小定理
    //     a^(p-1) = 1     (mod p)
    // <=> a * a^(p-2) = 1 (mod p)
    // <=> 1 / a = a^(p-2) (mod p)
    // (ただし、法pは素数)

    return mod_pow(a, modulo - 2, modulo)
}

// mod p を法とした時の累乗
// base^(x) % mod を繰り返し二乗法により、O(log2(x))の計算量で求める　(O(x)だとTLE)
// No.69参照
fn mod_pow(mut base: usize, mut exponent: usize, modulo: usize) -> usize {
    // 例: 3^4= (3^2)^2 = 9^2 = 81^1
    // 初期
    // 3^4
    // remainder=1
    // base=3
    // exp=4

    // i=0:
    // remainder = 1
    // base = 3 * 3 = 9
    // exp = 4 / 2 = 2

    // i=1:
    // remainder = 1
    // base = 9 * 9 = 81
    // exp = 2 / 2 = 1

    // i=2:
    // remainder = remainder * base = 81
    // base = 81 * 81
    // exp = 1 / 2 = 0

    let mut remainder = 1;
    while exponent != 0 {
        if exponent % 2 == 1 {
            remainder = (remainder * base) % modulo;
        }
        base = (base * base) % modulo;
        exponent /= 2;
    }
    return remainder
}
