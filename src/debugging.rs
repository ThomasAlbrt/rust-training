#![allow(dead_code)]
#![allow(unused_variables)]

fn double_value(v: i32) -> i32 {
    v * 2
}

pub fn debugging() {
    let mut x: i32 = 3;
    println!("x = {}", x);
    
    x = double_value(x);
    println!("x = {}", x);
}