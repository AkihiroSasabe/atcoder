use proconio::input;
fn main() {
    input! {
        n: usize,
        k: usize,
    }
    let mut a = vec![vec![]; n];

    for i in 0..n {
        input! {
            a_i: usize
        }
        a[i] = vec![a_i, i+1];
    }

    input! {
        b: [usize; k]
    }

    a.sort();

    let mut max_a = a[n-1][0].clone();
    let mut max_index_list = vec![];

    for i in 0..n {
        let j = n-1-i;
        if max_a != a[j][0] {
            break
        }
        else {
            max_index_list.push(j);
        }
    }
    
    let mut answer = "No";
    for i in b {
        for j in max_index_list.iter() {
            if a[*j][1] == i {
                answer = "Yes";
                break
            }
        }
    }
    println!("{}", answer);

}