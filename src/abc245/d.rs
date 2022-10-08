use proconio::input;
fn main() {
    input! {
        n: usize,
        m: usize,
        a: [isize; n+1],
        c: [isize; m+n+1]
    }

    let mut b: Vec<isize> = vec![0; m+1];
    
    for i in 0..m+1 {
        let mut sum = 0;
        for j in 0..i {
            if j+1 > n {continue}
            if i-j-1 > m {continue}
            sum += a[j+1]*b[i-j-1];
        }
        if a[0] != 0 {
            b[i] = (c[i] - sum)/ a[0];
        }
    }

    for i in b {
        print!("{} ", i);
    }
}