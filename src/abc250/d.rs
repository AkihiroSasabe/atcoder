use proconio::input;
fn main() {
    input! {
        n: usize
    }

    let mut dp = vec![vec![]; n + 1];


    let mut ans = 0;
    for i in 2..(n+1) {
        let primes = prime_factorize(i);
        if primes.len() == 2 {
            if primes[0][1] == 1 && primes[1][1] == 3 {
                ans += 1;
            }
        }
        // println!("i: {}", i);
    }

    println!("{}", ans);
}


fn prime_factorize(n: usize) -> Vec<Vec<usize>> {
    let mut primes = vec![];
    let mut x = n;
    let mut prime = 2;
    while prime * prime <= n {
        let mut exp = 0;
        while x % prime == 0 {
            x = x / prime;
            exp += 1;
        }
        if exp > 0 {
            primes.push(vec![prime, exp]);
        }
        prime += 1;
    }
    if x != 1 {
        primes.push(vec![x, 1]);
    }
    return primes
}