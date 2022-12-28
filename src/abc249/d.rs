use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [usize; n]
    }
    // a[i] == a[j] * a[k];

    // a.sort();
    // println!("{:?}", a);
    let a_max = 200_000;
    let mut hash = vec![vec![]; a_max+1];

    for i in 0..n {
        hash[a[i]].push(i);
    }

    let mut ans = 0;
    for x in 1..a_max+1 {
        for y in 1..(a_max/x+1) {
            let z = x * y;
            if hash[x].len() > 0 && hash[y].len() > 0 && hash[z].len() > 0 {
                // println!("x*y=z, {} * {} = {}", x, y, z);
                ans += (hash[x].len() * hash[y].len() * hash[z].len());
            }
        }
    }
    println!("{}", ans);
}