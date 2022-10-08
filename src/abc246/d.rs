use proconio::input;
fn main() {
    input! {
        n: usize
    }

    let M = 1_000_000;

    let INF = 1 << 60;
    let mut answer = INF;
    for b in 0..M {
        // println!("");
        // println!("b {}", b);
        let mut left = b;
        let mut right = M;
        let mut x = INF;
        let mut a = (left + right) / 2;
        while right > left + 1 {
            // println!("------");
            // println!("left {}", left);
            // println!("right {}", right);
            a = (left + right) / 2;
            x = f(a,b);
            // println!("a: {}", a);
            // println!("x: {}", x);
            if x < n {
                left = a;
            }
            else if x == n {
                println!("{}", x);
                return
            }
            else if n < x {
                right = a;
            }
            // println!("after left {}", left);
            // println!("after right {}", right);
        }

        x = f(left,b);
        // leftがちょうど条件を満たせるとき
        if x >= n {
            // pass
        }
        else {
            x = f(left + 1 , b);
            // println!("x= {}", x);
        }
        if answer > x {
            answer = x;
        }
    }
    println!("{}", answer);
}

fn f(a: usize, b: usize) -> usize {
    a*a*a + a*a*b + a*b*b + b*b*b
}