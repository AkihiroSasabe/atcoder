use proconio::input;
use itertools::Itertools;
use std::cmp::{max, min};
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use proconio::marker::Chars;
use std::f64::consts::PI;
use std::mem::swap;
fn main() {
    input! {
        ax: f64,
        ay: f64,
        bx: f64,
        by: f64,
        cx: f64,
        cy: f64,
        dx: f64,
        dy: f64,
    }
    // println!("{} {} {} {}", ax, ay, bx, by);

    let ab = vec![bx - ax, by - ay];
    let ad = vec![dx - ax, dy - ay];
    let a = get_angle(ab, ad);

    let ba = vec![ax - bx, ay - by];
    let bc = vec![cx - bx, cy - by];
    let b = get_angle(ba, bc);

    let cb = vec![bx - cx, by - cy];
    let cd = vec![dx - cx, dy - cy];
    let c = get_angle(cb, cd);

    let dc = vec![cx - dx, cy - dy];
    let da = vec![ax - dx, ay - dy];    
    let d = get_angle(dc, da);

    // println!("{} {} {} {}", a, b, c, d);
    let sum_angle = a + b + c + d;
    // println!("{}", sum_angle);

    if sum_angle < 359.0 {
        println!("No");
    }
    else if a < 180.0 && b < 180.0 && c < 180.0 && d < 180.0 {
        println!("Yes");
    }
    else {
        println!("No");
    }
}

fn get_angle(vec1: Vec<f64>, vec2: Vec<f64>) -> f64 {
    let cos = (vec1[0] * vec2[0] + vec1[1] * vec2[1]) / ((vec1[0]*vec1[0] + vec1[1] * vec1[1]).sqrt() * (vec2[0]*vec2[0] + vec2[1] * vec2[1]).sqrt());
    let angle = cos.acos() * 180.0 / PI;
    // println!("cos: {}, acos: {}", cos, cos.acos());
    return angle
}