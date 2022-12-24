use proconio::input;
fn main() {
    input! {
        n: usize
    }

    // p * q^3 <= Nなので、q<N^(1/3)
    let q_cubic_root = (n as f64).powf(1.0/3.0); 
    let primes = sieve_oferatosthenes(q_cubic_root as usize);


    // let mut dp = vec![vec![]; n + 1];


    // let mut ans = 0;
    // for i in 2..(n+1) {
    //     let primes = prime_factorize(i);
    //     if primes.len() == 2 {
    //         if primes[0][1] == 1 && primes[1][1] == 3 {
    //             ans += 1;
    //         }
    //     }
    //     // println!("i: {}", i);
    // }

    println!("{}", ans);
}

// エラトステネスの篩(ふるい)
// n以下の素数を全て列挙する為のアルゴリズムO(n*log(log(n)))
fn sieve_oferatosthenes(n: usize) -> Vec<usize> {
    // let mut root_n = 1;
    // while root_n * root_n <= n {
    //     root_n += 1;
    // }
    // let mut sosu_list = vec![1; root_n + 1];
    let mut sosu_list = vec![1; n + 1];
    sosu_list[0] = 0;
    sosu_list[1] = 0;

    let mut sosu = 2;
    
    for i in 2..(n+1) {
        if sosu_list[i] == 0 {continue}
        if n % i == 0 {
            for j in 2..(n/i) {
                sosu_list[j * i] = 0;
            }
        }
    }
    return sosu_list;
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