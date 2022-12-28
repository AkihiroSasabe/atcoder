use proconio::input;
fn main() {
    input! {
        n: usize,
        m: usize,
        a: [isize; n+1],
        c: [isize; m+n+1]
    }
    // 思考実験
    // a = x + 2;
    // b = 2x^2 * +4x + 6
    // c = 2x^3 + 8x^2 + 14x + 12

    // c[0] = a[0] * b[0]
    // c[1] = a[1] * b[0] + a[0] * b[1]
    // c[2] = a[2] * b[0] + a[1] * b[1] + a[0] * b[2]
    // c[3] = a[3] * b[0] + a[2] * b[1] + a[1] * b[2] + a[0] * b[3]

    // b[0]から順番に求められそう。
    // 注意が要るのはa[0] == 0の時で、a[i]==0になる最小のi (=:min_index_for_not0) を求めてからb[0]を計算する必要がある。
    let INF = 1 << 60;
    let mut b: Vec<isize> = vec![INF; m+1];

    let mut min_index_for_not0 = a.len() - 1;
    for i in 0..a.len() {
        if a[i] != 0 {
            b[0] = c[i] / a[i];
            min_index_for_not0 = i;
            break
        }
    }
    b[m] = c[n+m] / a[n];

    for i in 1..b.len() {
        b[i] = c[i + min_index_for_not0];
        for j in 0..i {
            // println!("(i, j) = {}, {}", i, j);
            if i < j {continue}
            if i-j+min_index_for_not0 >= a.len() {continue}
            // println!("----(i, j) = {}, {}", i, j);
            b[i] -= a[i-j+min_index_for_not0] * b[j];
        }
        b[i] /= a[min_index_for_not0];
    }
    for bi in b.iter() {
        print!("{} ", bi);
    }
}