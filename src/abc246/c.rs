use proconio::input;
fn main() {
    input! {
        n: usize,
        k: usize,
        x: usize,
        a: [usize; n]
    }
    // 数列の要素について、x円で割った[商, 余り]を格納
    let mut quotients: Vec<usize> = a.iter().map(|ai| ai/x).collect();
    let mut remainders: Vec<usize> = a.iter().map(|ai| ai%x).collect(); 
    
    let mut q_sum = 0;
    for i in quotients {
        q_sum += i;
    }
    if k <= q_sum {
        let mut answer: usize = a.iter().sum::<usize>() - k * x;
        println!("{}", answer);
    }
    else {
        remainders.sort();
        if remainders.len() > (k-q_sum){
            let answer = remainders[0..remainders.len()-(k-q_sum)].iter().sum::<usize>();
            println!("{}", answer);
        }
        else {
            let answer = 0;
            println!("{}", answer);
        }
    }
}