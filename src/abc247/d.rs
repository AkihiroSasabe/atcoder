use proconio::input;
use std::collections::VecDeque;

fn main() {
    input! {
        q: usize
        
    }
    let mut queue: VecDeque<Vec<usize>> = VecDeque::new();
    let mut answer = vec![];

    for i in 0..q {
        input! {
            command: usize
        }
        if command == 1 {
            input! {
                x: usize,
                c: usize
            }
            queue.push_back(vec![x, c]);
        }

        else if command == 2 {
            input! {
                mut c: usize
            }
            let mut sum = 0;
            let mut head = queue.pop_front().unwrap();
            while head[1] < c {
                sum += head[0] * head[1];
                c -= head[1];
                head = queue.pop_front().unwrap();
            }
            sum += head[0] * c;
            head = vec![head[0], head[1] - c];
            queue.insert(0, head);
            answer.push(sum);
        }
    }

    for i in answer {
        println!("{}", i);
    }
}