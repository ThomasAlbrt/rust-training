#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

mod stack_heap;
mod debugging;
mod conditions;
mod loops;
mod match_st;
mod combination_lock;
mod data_structures;

use std::mem;
fn main() {
    // variables();
    // operators();
    // stack_heap::stack_heap();
    // debugging::debugging();
    // conditions::conditions();
    // loops::loops();
    // match_st::match_statement();
    // combination_lock::lock();
    data_structures::data_s();
}

fn variables() {
    let a: u8 = 5;
    println!("a = {}, takes up {} bytes", a, mem::size_of_val(&a));

    let b: char = 'c';
    println!("b = {}, takes up {} bytes", b, mem::size_of_val(&b));

    let c: &str = "coucou";
    println!("c = {}, takes up {} bytes", c, mem::size_of_val(&c));

    let d: bool = true;
    println!("d = {}, takes up {} bytes", d, mem::size_of_val(&d));
}

fn operators() {
    let a: i32 = 2 * 3;
    println!("{}", a);

    let a_cubed = i32::pow(a, 3);
    println!("{} cubed gives {}", a, a_cubed);

    // bitwise
    let b: u8 = 1 | 2;
    println!("b = {}", b);

    // logical
    let pi_less_4 = std::f64::consts::PI > 4.0;
    println!("{}", pi_less_4);

    let c = a == b.into();
    println!("{}", c);
}