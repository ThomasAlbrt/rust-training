#![allow(unused_mut)]

use std::os::unix::process;
use std::mem;

// Structs
struct Point {
    x: f32,
    y: f32
}

pub fn data_s() {
    println!("Data structures");
    // structs();
    // enums();
    // unions();
    arrays();
}

fn structs() {
    let p = Point {x: 3.0, y: 5.0};
    println!("point 1 has ({}, {}) coords", p.x, p.y)
}

// Enums
enum Color {
    Red,
    Green,
    Blue,
    RgbColor(u8,u8,u8), //tuple
    Cmyk {cyan: u8, magenta: u8, yellow: u8, black: u8} //struct
}

fn enums() {
    let c: Color = Color::Cmyk { cyan: 0, magenta: 128, yellow: 0, black: 254 };
    match c {
        Color::Red => {
            println!("Red")
        },
        Color::Green => {
            println!("Green")
        },
        Color::Blue => {
            println!("Blue")
        },
        Color::RgbColor(0,0,0) => println!("Black"),
        Color::RgbColor(r,g,b) => println!("rgb({},{},{})", r,g,b),
        Color::Cmyk { cyan: _, magenta: _, yellow: _, black: 255 } => {
            println!("Cmyk black")
        }
        _ => ()
    }
}

// Unions
union IntOrFloat {
    i: i32,
    f: f32
}

fn process_value(iof: IntOrFloat) {
    unsafe {
        match iof {
            IntOrFloat { i: 42 } => {
                println!("meaning of life value")
            }

            IntOrFloat { f } => {
                println!("value = {}", f)
            }
        }
    }
}

fn unions() {
    let mut iof = IntOrFloat{ i:123 };
    iof.i = 234;

    let value = unsafe { iof.i };
    println!("iof.i = {}", value);

    process_value(IntOrFloat{ i:5 })
}


fn arrays() {
    let mut a:[i32;5] = [1, 2, 3, 4, 5];
    println!("a has {} elements, first is {}", a.len(), a[0]);

    a[0] = 12345;
    println!("a[0] = {}", a[0]);

    println!("whole array = {:?}", a);

    if a != [12345, 2, 3, 4, 5] {
        println!("array a is different from [1, 2, 3, 4, 5]");
    }

    let b = [1u8;10];
    println!("b = {:?}", b);

    for i in 0..b.len() {
        println!("i = {}", i);
    }

    println!("b takes up {} bytes", mem::size_of_val(&b));

    let mtx: [[f32; 3]; 2] = [
        [0.0, 0.0, 0.0],
        [1.0, 2.0, 0.0]
    ];
    println!("mtx = {:?}", mtx);

    for i in 0..mtx.len() {
        println!("{}", i);
        for o in 0..mtx[i].len() {
            println!("{}", o)
        }
    }
}