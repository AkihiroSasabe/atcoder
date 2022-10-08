use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
    }

    let s2 = s.clone();

    let small: Vec<char> = "abcdefghijklmnopqrstuvwxyz".chars().collect();
    let big: Vec<char> = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect();
    
    let mut flag = true;
    for i in 0..s2.len() {
        for j in i+1..s2.len() {
            if s[i] == s[j] {
                // println!("{} {}", s[i], s[j]);
                flag = false;
                println!{"No"};
                // println!{"aaa"};
                return
            }
        }
    }

    flag = false;
    for i in 0..s2.len() {
        if flag {break}
        for j in 0..small.len() {
            if s[i] == small[j] {
                flag = true;
                break
            }
        }
    }

    if !flag {
        println!("No");
        // println!{"bbb"};
        return
    }

    flag = false;
    for i in 0..s2.len() {
        if flag {break}
        for j in 0..big.len() {
            if s[i] == big[j] {
                flag = true;
                break
            }
        }
    }

    if flag {
        println!("Yes");
        // println!{"ccc"};
    }
    else {
        println!("No");
        // println!{"ddd"};
    }
}