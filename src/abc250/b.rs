use proconio::input;
fn main() {
    input! {
        n: usize,
        a: usize,
        b: usize
    }

    let mut map = vec![vec!['#'; n*b]; n*a];
    // let mut map = vec![vec![1; n*b]; n*a];

    for i in 0..n {
        for j in 0..n {
            // println!("i % 2 {}", i % 2);
            // println!("j % 2 {}", j % 2);
            if (i + j) % 2 == 0 { 
                // println!("");   
                // map[i*a..i*(a+1)][j*b..j*(b+1)] = ".";
                // map[i*a..i*(a+1)][j*b..j*(b+1)] = &vec![vec![2; a]; b];
                for y in i*a..(i+1)*a {
                    for x in j*b..(j+1)*b {
                        map[y][x] = '.';
                        // map[y][x] = 2;
                    }
                }
            }
        }
    }

    for i in map {
        for j in i {
            // println!("{:?}", i);
            print!("{}", j);
        }
        println!("");
    }


}