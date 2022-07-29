pub fn conditions() {
    let temp: u8 = 31;

    if temp > 30 {
        println!("It's very hot");
    } else if 20 < temp && temp < 30 {
        println!("It's hot");
    } else {
        println!("It's cold");
    }

    let day: &str = if temp > 15 {"hot"} else {"cold"};
    println!("{}", day);
}