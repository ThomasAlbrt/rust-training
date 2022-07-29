pub fn loops() {

    // while
    let mut x = 1;
    while x < 1000 {
        x *= 2;

        if x == 64 {break};

        println!("{}", x);
    }

    // loop
    let mut y: i32 = 0;
    loop {
        y += 1;
        if y > 100 {break};

        println!("{}", y);
    }

    // for loop
    for z in 1..11 {
        println!("z = {}", z)
    }

    for (pos, a) in (1..=11).enumerate() {
        println!("{} is at index {}", a, pos);
    }
}